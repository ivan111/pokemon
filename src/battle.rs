//! トレーナーバトルのシミュレーション
//! AIでの自動対戦のみに対応する。エミュレーションはしない。

use std::cmp::Ordering;
use std::sync::Arc;

use rand::prelude::*;

use crate::pokepedia::Pokepedia;
use crate::pokemon::{Pokemon, Stats, IVs};
use crate::types::Type;
use crate::moves::*;

macro_rules! debug_print {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::print!($($arg)*); })
}

macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

pub const MS_PER_TURN: i32 = 500;  // 500 ミリ秒/ターン
pub const TURN_PER_SEC: i32 = 2;  // 2 ターン/秒
pub const LIMIT_TURN: i32 = (4 * 60 + 30) * TURN_PER_SEC;  // 制限ターン数(4分30秒)

/// プレイヤーがとることのできる行動
#[derive(Debug, Clone, Copy)]
pub enum Action {
    FastMove,  // ノーマルアタック
    ChargeMove(u8),  // スペシャルアタック([data] 0: わざ1, 1: わざ2)
    SwitchPokemon(u8),  // ポケモンを交代([data] インデックス)
    None,  // 何もしない
           // 硬直、相手がスペシャルアタック中、相手が交換ポケモン選択中などで待機のときにも自動設定
}

#[derive(Debug, Clone)]
pub enum Phase {
    Neutral,
    TimeOver(u8),  // 時間オーバー([data] 0: プレイヤー0の勝利, 1: プレイヤー1の勝利, 2: 引き分け)
    GameOver(u8),  // ゲームオーバー([data] 0: プレイヤー0の勝利, 1: プレイヤー1の勝利, 2: 引き分け)
}

pub struct Battle {
    pub states: Vec<State>,  // 各状態。一度登録した状態は変更しない。
    pub actions: Vec<[Action; 2]>,  // 行動

    pub strategy0: fn(&Player) -> Action,
    pub shield0: fn(&Player) -> bool,
    pub switch0: fn(&Player) -> usize,

    pub strategy1: fn(&Player) -> Action,
    pub shield1: fn(&Player) -> bool,
    pub switch1: fn(&Player) -> usize,
}

// TODO: 制限時間4分30秒

#[derive(Clone)]
pub struct State {
    pub player0: Player,
    pub player1: Player,
    pub phase: Phase,

    pub turn: i32,  // 現在のターン数。実際に行動した単位。
    pub elapsed_ms: i32,  // 経過時間(ミリ秒)
    pub logs: Vec<String>,
}

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub team: Vec<BattlePokemon>,
    pub cur_poke: usize,  // 現在のポケモン。インデックス
    pub num_shields: i32,  // シールドの数
    pub switch_ms: i32,  // ポケモンを交代可能になるまでのミリ秒。0なら可能
    pub pending: Action,  // 硬直中に実行して保留されたアクション

    pub in_fast_move: bool,  // ノーマルアタック使用中
    pub dur_turns: i32,  // 硬直ターン数。毎ターンの最初に-1されて、0なら行動可能
}

#[derive(Clone)]
pub struct BattlePokemon {
    pub poke: Arc<Pokemon>,

    pub hp: i32,  // 残りHP
    pub energy: i32,  // 充填されたエネルギー。0から100の値をとる。
    pub buff: (i32, i32),  // ランク補正。(攻撃ランク, 防御ランク)。ランクは-4から4の範囲をとる。
    pub is_disable_type_effect: bool,  // タイプ相性を無効にする。PPT(Power Per Turn)の計算に使用
}

fn strategy(player: &Player) -> Action {
    if player.poke().can_charge_move1() {
        Action::ChargeMove(0)
    } else {
        Action::FastMove
    }
}

fn shield_strategy(_player: &Player) -> bool {
    true
}

fn switch_strategy(_player: &Player) -> usize {
    0
}

impl Battle {
    pub fn new(name0: String, team0: Vec<Pokemon>, name1: String, team1: Vec<Pokemon>) -> Self {
        assert!(!team0.is_empty() && !team1.is_empty());

        let player0 = Player::new(name0, team0);
        let player1 = Player::new(name1, team1);

        let start_state = State {
            player0,
            player1,
            phase: Phase::Neutral,
            turn: 1,
            elapsed_ms: 0,
            logs: vec![],
        };

        Battle {
            states: vec![start_state],
            actions: vec![],

            strategy0: strategy,
            shield0: shield_strategy,
            switch0: switch_strategy,

            strategy1: strategy,
            shield1: shield_strategy,
            switch1: switch_strategy,
        }
    }

    pub fn state(&self) -> &State {
        self.states.last().unwrap()
    }

    pub fn is_ended(&self) -> bool {
        let state = self.state();

        state.player0.is_ended() || state.player1.is_ended()
    }

    /*
    pub fn print_timeline(&self) {
        for state in &self.states {
        }
    }
    */

    pub fn start(&mut self) {
        let mut act0;
        let mut act1;

        {
            let state = self.state();
            act0 = (self.strategy0)(&state.player0);
            act1 = (self.strategy1)(&state.player1);
        }

        let mut i = 0;

        while self.do_action([act0, act1]) {
            let state = self.state();
            act0 = (self.strategy0)(&state.player0);
            act1 = (self.strategy1)(&state.player1);

            i += 1;

            if i > 100 {
                break;
            }
        }
    }

    /// 戻り値: true: ゲーム継続中, false: ゲーム終了
    pub fn do_action(&mut self, actions: [Action; 2]) -> bool {
        let mut state = self.state().clone();

        // 1ターン技か？
        let mut use_1turn_move = [false, false];

        // スペシャルアタック使用確認。 0: わざ1, 1: わざ2, 2: 使用しない
        let mut use_charge_move: [usize; 2] = [2, 2];

        match state.phase {
            Phase::GameOver(_) | Phase::TimeOver(_) => {
                debug_println!("Debug: 無意味なdo_action呼び出し, phase = {:?}", state.phase);
                return false;
            },

            _ => (),
        }

        debug_println!();
        debug_println!("Debug: turn {} start [time: {} s] ---------------------------", state.turn, (state.elapsed_ms as f32 / 1000.0));
        debug_println!("Debug: action = [{:?}, {:?}], phase = {:?}", actions[0], actions[1], state.phase);

        for p in state.players() {
            debug_println!("Debug: player {}: dur_turns = {}, switch_ms = {}, num_shields = {}", p.name, p.dur_turns, p.switch_ms, p.num_shields);

            let poke = p.poke();
            debug_print!("Debug:   {}: HP = {} / {}, energy = {}, buff = {:?}", p.poke_name(), poke.hp(), poke.base_hp(), poke.energy, poke.buff);
            debug_println!(", in_fast_move = {}, dur_turns =  {}", p.in_fast_move, p.dur_turns);
        }

        debug_println!();

        match state.phase {
            Phase::GameOver(_) | Phase::TimeOver(_) => (),

            Phase::Neutral => {
                for (player_i, p) in state.players_mut().into_iter().enumerate() {
                    if p.in_fast_move && p.dur_turns != 0 {  // 硬直中
                        // 硬直中に交代やスペシャルアタックをすると、硬直後に実行される。
                        // 硬直中に交代とスペシャルアタックを両方すると、硬直後に代する。
                        debug_println!("Debug: {}は硬直中", p.poke_name());
                    }

                    match actions[player_i] {
                        Action::None => continue,

                        Action::SwitchPokemon(poke_i) => {
                            if p.in_fast_move {
                                p.set_pending(actions[player_i]);
                            } else if p.switch_pokemon(poke_i.into()) {
                                debug_println!("Debug: 交代 player = {}, 出てきたポケモン = {}", p.name, p.poke_name());
                            }
                        },

                        Action::FastMove => {
                            if !p.in_fast_move {  // 硬直中でないなら
                                p.in_fast_move = true;
                                p.dur_turns = p.poke().fast_move().turns() - 1;

                                if p.dur_turns == 0 {
                                    use_1turn_move[player_i] = true;
                                }

                                debug_println!("Debug: {}は{}を使った。", p.poke_name(), p.poke().fast_move().name());
                            }
                        },

                        Action::ChargeMove(i @ (0 | 1)) => {
                            if let Some(mv) = p.poke().charge_move(i as usize) {
                                if p.poke().can_charge_move(i as usize) {
                                    if p.in_fast_move {
                                        p.set_pending(actions[player_i]);
                                    } else {
                                        use_charge_move[player_i] = i as usize;
                                    }
                                } else {
                                    debug_println!("Debug: [{}] {}に必要なエネルギーが足りない", p.name, mv.name());
                                }
                            } else {
                                debug_println!("Debug: [{}] スペシャルアタック{}が存在しない", p.name, i);
                            }
                        },

                        Action::ChargeMove(i) => {
                            debug_println!("Debug: [{}] 不正な値 Action::ChargeMove({})", p.name, i);
                        },
                    }
                }
            },
        }

        let mut incr_ms = MS_PER_TURN;
        let switch_strategy = [self.switch0, self.switch1];
        let shield_strategy = [self.shield0, self.shield1];

        // ノーマルアタックとスペシャルアタックが同時に使われた場合は、
        // ノーマルアタックが有効になることも、無効になることもあるらしい。
        // このソフトの仕様では常に有効になる。

        // １ターン技はランダムに先行が決まる仕様にしている
        // 本物は先に打った方が先行になる
        state.do_1turn_move(use_1turn_move);
        incr_ms += state.switch_if_faint(switch_strategy);

        // スペシャルアタック。switch_if_faintを含む
        incr_ms += state.do_charge_move(use_charge_move, shield_strategy, switch_strategy);

        // スペシャルアタックを打った相手が硬直中ならすぐにノーマルアタックできる
        state.do_fast_move();
        incr_ms += state.switch_if_faint(switch_strategy);

        state.set_gameover_phase();

        state.increment_turns(incr_ms);

        let ret_val = !matches!(state.phase, Phase::GameOver(_) | Phase::TimeOver(_));

        self.states.push(state);
        self.actions.push(actions);

        ret_val
    }
}

pub const CHARGE_MOVE_MS: i32 = 20 * MS_PER_TURN;
/// ポケモンを倒されてプレイヤーが次のポケモンを選ぶのにかかった時間をこれと仮定
pub const SWITCH_MS: i32 = 10 * MS_PER_TURN;

impl State {
    fn elapsed_ms(&self) -> i32 {
        self.turn * MS_PER_TURN
    }

    fn player(&self, i: usize) -> &Player {
        if i == 0 {
            &self.player0
        } else {
            &self.player1
        }
    }

    fn player_mut(&mut self, i: usize) -> &mut Player {
        if i == 0 {
            &mut self.player0
        } else {
            &mut self.player1
        }
    }

    fn players(&self) -> Vec<&Player> {
        vec![&self.player0, &self.player1]
    }

    fn players_mut(&mut self) -> Vec<&mut Player> {
        vec![&mut self.player0, &mut self.player1]
    }

    fn do_1turn_move(&mut self, use_1turn_move: [bool; 2]) {
        let p0 = &mut self.player0;
        let p1 = &mut self.player1;

        if rand::random() {
            if use_1turn_move[0] && p0.in_fast_move && p0.dur_turns == 0 {
                p0.team[p0.cur_poke].do_fast_move(&mut p1.team[p1.cur_poke]);
                p0.in_fast_move = false;
            }

            let is_faint = p1.team[p1.cur_poke].is_faint();

            if use_1turn_move[1] && p1.in_fast_move && p1.dur_turns == 0 && !is_faint {
                p1.team[p1.cur_poke].do_fast_move(&mut p0.team[p0.cur_poke]);
                p1.in_fast_move = false;
            }
        } else {
            if use_1turn_move[1] && p1.in_fast_move && p1.dur_turns == 0 {
                p1.team[p1.cur_poke].do_fast_move(&mut p0.team[p0.cur_poke]);
                p1.in_fast_move = false;
            }

            let is_faint = p0.team[p0.cur_poke].is_faint();

            if use_1turn_move[0] && p0.in_fast_move && p0.dur_turns == 0 && !is_faint {
                p0.team[p0.cur_poke].do_fast_move(&mut p1.team[p1.cur_poke]);
                p0.in_fast_move = false;
            }
        }
    }

    fn do_fast_move(&mut self) {
        let p0 = &mut self.player0;
        let p1 = &mut self.player1;

        if p0.in_fast_move && p0.dur_turns == 0 {
            p0.team[p0.cur_poke].do_fast_move(&mut p1.team[p1.cur_poke]);
            p0.in_fast_move = false;
        }

        if p1.in_fast_move && p1.dur_turns == 0 {
            p1.team[p1.cur_poke].do_fast_move(&mut p0.team[p0.cur_poke]);
            p1.in_fast_move = false;
        }
    }

    fn sort_use_charge_move(&self, mut use_charge_move: [usize; 2]) -> Vec<(usize, usize)> {
        if self.player0.poke().is_faint() {
            use_charge_move[0] = 2;
        }

        if self.player1.poke().is_faint() {
            use_charge_move[1] = 2;
        }

        let mut v = vec![];

        match use_charge_move {
            [i0 @ (0 | 1), i1 @ (0 | 1)] => {  // ２人とも使う
                let atk0 = self.player0.poke().stats().attack.floor() as u32;
                let atk1 = self.player1.poke().stats().attack.floor() as u32;

                if atk0 == atk1 {  // random
                    if rand::random() {
                        v.push((0, i0));
                        v.push((1, i1));
                    } else {
                        v.push((1, i1));
                        v.push((0, i0));
                    }
                } else if atk0 < atk1 {  // player1が先
                    v.push((1, i1));
                    v.push((0, i0));
                } else {  // player0が先
                    v.push((0, i0));
                    v.push((1, i1));
                }
            },

            [i @ (0 | 1), 2] => {
                v.push((0, i));
            }

            [2, i @ (0 | 1)] => {
                v.push((1, i));
            }

            _ => (),
        }

        v
    }

    fn do_charge_move(&mut self, use_charge_move: [usize; 2], shield_strategy: [fn(&Player) -> bool; 2], switch_strategy: [fn(&Player) -> usize; 2]) -> i32 {
        assert!((0..=2).contains(&use_charge_move[0]) && (0..=2).contains(&use_charge_move[1]));

        let v = self.sort_use_charge_move(use_charge_move);

        let mut incr_ms = 0;

        let num_shields = [self.player0.num_shields, self.player1.num_shields];

        for (player_i, mv_i) in v {
            let opponent_i = if player_i == 0 { 1 } else { 0 };
            let shield = num_shields[opponent_i] > 0 && (shield_strategy[opponent_i])(self.player(opponent_i));

            {
                let p0 = &mut self.player0;
                let p1 = &mut self.player1;
                let poke0 = p0.poke_mut();
                let poke1 = p1.poke_mut();

                if player_i == 0 {
                    poke0.do_charge_move(mv_i, poke1, 1.0, shield);
                    p1.dur_turns = 0; // CCT(差し込み)

                    if shield {
                        p1.num_shields = std::cmp::max(0, p1.num_shields - 1);
                    }
                } else {
                    poke1.do_charge_move(mv_i, poke0, 1.0, shield);
                    p0.dur_turns = 0; // CCT(差し込み)

                    if shield {
                        p0.num_shields = std::cmp::max(0, p0.num_shields - 1);
                    }
                }
            }

            if self.player(opponent_i).poke().is_faint() {
                incr_ms += self.switch_if_faint(switch_strategy);
                break;
            }
        }

        incr_ms
    }

    /// 気絶しているポケモンがいたら、ポケモンを交代させる
    fn switch_if_faint(&mut self, switch_strategy: [fn(&Player) -> usize; 2]) -> i32 {
        let mut fainted = vec![];  // 交換が必要なプレイヤーの番号を入れる

        for (player_i, p) in self.players_mut().into_iter().enumerate() {
            if p.poke().is_faint() && p.num_remains() > 0 {
                fainted.push(player_i);
            }
        }

        let mut incr_ms = 0;

        for player_i in fainted {
            let i = (switch_strategy[player_i])(self.player(player_i));

            if !self.player_mut(player_i).switch_pokemon(i) {
                self.player_mut(player_i).force_switch();
            } else {
                incr_ms = SWITCH_MS;
            }
        }

        incr_ms
    }

    fn set_gameover_phase(&mut self) {
        let p0_ended = self.player0.is_ended();
        let p1_ended = self.player1.is_ended();

        match (p0_ended, p1_ended) {
            (false, true) => {  // プレイヤー0の勝利
                debug_println!("Debug: Game Over. {}の勝ち", self.player0.name);
                self.phase = Phase::GameOver(0);
            },
            (true, false) => {  // プレイヤー1の勝利
                debug_println!("Debug: Game Over. {}の勝ち", self.player1.name);
                self.phase = Phase::GameOver(1);
            },
            (true, true) => {  // 引き分け
                debug_println!("Debug: Game Over. 引き分け");
                self.phase = Phase::GameOver(2);
            },
            (false, false) => (),
        }
    }

    fn increment_turns(&mut self, mut incr_ms: i32) {
        incr_ms = std::cmp::max(MS_PER_TURN, incr_ms);  // 最低でも500msは進める

        for i in 0..2 {
            let p = self.player_mut(i);

            p.dur_turns = std::cmp::max(p.dur_turns - 1, 0);
            p.switch_ms = std::cmp::max(p.switch_ms - incr_ms, 0);
        }

        self.turn += 1;
        self.elapsed_ms += incr_ms;

        if self.turn > LIMIT_TURN {
            // のこりHPが多いほうが勝ち
            let hp0 = self.player0.sum_hp();
            let hp1 = self.player1.sum_hp();

            self.phase = match hp0.cmp(&hp1) {
                Ordering::Less => Phase::TimeOver(1),
                Ordering::Greater => Phase::TimeOver(0),
                Ordering::Equal => Phase::TimeOver(2),
            };

            debug_println!("Debug: Time Over");
        }
    }
}

impl Player {
    fn new(name: String, poke_team: Vec<Pokemon>) -> Self {
        let team: Vec<_> = poke_team.into_iter().map(|poke| BattlePokemon::new(Arc::new(poke))).collect();

        Player {
            name,
            team,
            cur_poke: 0,
            num_shields: 2,
            switch_ms: 0,
            pending: Action::None,
            in_fast_move: false,
            dur_turns: 0,
        }
    }

    pub fn poke(&self) -> &BattlePokemon {
        &self.team[self.cur_poke]
    }

    pub fn poke_mut(&mut self) -> &mut BattlePokemon {
        &mut self.team[self.cur_poke]
    }

    pub fn poke_name(&self) -> &'static str {
        self.team[self.cur_poke].name()
    }

    pub fn num_remains(&self) -> i32 {
        self.team.iter().filter(|p| p.hp > 0).count().try_into().unwrap()
    }

    pub fn sum_hp(&self) -> i32 {
        self.team.iter().map(|p| p.hp).sum()
    }

    pub fn is_ended(&self) -> bool {
        self.team.iter().all(|p| p.hp <= 0)
    }

    pub fn set_pending(&mut self, action: Action) {
        match action {
            Action::ChargeMove(_) => {
                match self.pending {
                    Action::SwitchPokemon(_) => (),  // 交代のほうが優先
                    _ => {
                        self.pending = action;
                    },
                }
            },

            Action::FastMove => (),  // ノーマルアタックが保留になることはない

            Action::SwitchPokemon(_) | Action::None => {
                self.pending = action;
            },
        }
    }

    pub fn switch_pokemon(&mut self, i: usize) -> bool {
        if self.switch_ms > 0 {
            return false;
        }

        if i >= self.team.len() {
            return false;
        }

        if self.team[i].hp <= 0 {
            return false;
        }

        // ポケモンを交代させると、「こうげき」と「ぼうぎょ」に対する効果がリセットされる
        self.team[self.cur_poke].buff = (0, 0);

        self.cur_poke = i;
        self.switch_ms = 60 * TURN_PER_SEC * MS_PER_TURN;  // これから1分間交代できない

        // リセット
        self.in_fast_move = false;
        self.dur_turns = 0;
        self.pending = Action::None;

        true
    }

    // ポケモンを倒されて、次のポケモンを選ぶ時間が12秒与えられるが、
    // それを過ぎてもポケモンを選ばなかった場合にこちらが選ぶ
    pub fn force_switch(&mut self) -> bool {
        // team.len()が3なら1, 2, 0の順に選ぶ。
        for i in (1..self.team.len()).chain([0].into_iter()) {
            if i == self.cur_poke {
                continue;
            }

            if self.team[i].hp <= 0 {
                continue;
            }

            self.cur_poke = i;

            // リセット
            self.in_fast_move = false;
            self.dur_turns = 0;
            self.pending = Action::None;

            return true;
        }

        false
    }
}

#[test]
fn test_player() {
    let p0 = Pokemon::new("ココロモリ", "エアスラッシュ", "サイコファング", None, 1489, None, (10, 9, 12)).unwrap();
    let p1 = Pokemon::new("ブラッキー", "バークアウト", "あくのはどう", None, 1498, None, (2, 14, 0)).unwrap();
    let p2 = Pokemon::new("ナマズン", "みずでっぽう", "どろばくだん", None, 1474, None, (8, 15, 14)).unwrap();

    let mut p = Player::new(String::from("test"), vec![p0, p1, p2]);

    assert_eq!(p.poke_name(), "ココロモリ");
    assert_eq!(p.poke().fast_move().name(), "エアスラッシュ");
    assert_eq!(p.poke().charge_move1().name(), "サイコファング");
    assert_eq!(p.poke().charge_move2().is_none(), true);

    p.team[0].add_buff(1, 1);
    assert_eq!(p.poke().buff, (1, 1));

    p.switch_pokemon(2);
    assert_eq!(p.poke_name(), "ナマズン");
    assert_eq!(p.team[0].buff, (0, 0));  // ステータス変化がリセットされているか？

    p.force_switch();
    assert_eq!(p.poke_name(), "ブラッキー");

    assert_eq!(p.num_remains(), 3);
    assert_eq!(p.is_ended(), false);
}

/// ランク補正
pub const RANK_MUL: [f64; 9] = [0.5, 4.0/7.0, 2.0/3.0, 4.0/5.0, 1.0, 5.0/4.0, 3.0/2.0, 7.0/4.0, 2.0];

pub fn rank_mul(buff: i32) -> f64 {
    assert!((-4..=4).contains(&buff));

    RANK_MUL[(buff + 4) as usize]
}

pub const TRAINER_BATTLE_BONUS: f64 = 1.3;  // トレーナーバトルボーナス

impl BattlePokemon {
    pub fn new(poke: Arc<Pokemon>) -> Self {
        let hp = poke.hp();

        BattlePokemon {
            poke,
            hp,
            energy: 0,
            buff: (0, 0),
            is_disable_type_effect: false,
        }
    }

    pub fn dict(&self) -> &'static Pokepedia {
        self.poke.dict()
    }

    pub fn name(&self) -> &'static str {
        self.poke.name()
    }

    pub fn types(&self) -> Vec<Type> {
        self.poke.types()
    }

    pub fn base_stats(&self) -> Stats {
        self.poke.base_stats()
    }

    pub fn lv(&self) -> f32 {
        self.poke.lv()
    }

    pub fn ivs(&self) -> IVs {
        self.poke.ivs()
    }

    pub fn fast_move(&self) -> &'static FastMove {
        self.poke.fast_move()
    }

    pub fn charge_move1(&self) -> &'static ChargeMove {
        self.poke.charge_move1()
    }

    pub fn charge_move2(&self) -> Option<&'static ChargeMove> {
        self.poke.charge_move2()
    }

    // iが0ならスペシャルアタック1を返す
    // iが1ならスペシャルアタック2を返す
    pub fn charge_move(&self, i: usize) -> Option<&'static ChargeMove> {
        match i {
            0 => Some(self.charge_move1()),
            1 => self.charge_move2(),
            _ => None,
        }
    }

    pub fn cp(&self) -> i32 {
        self.poke.cp()
    }

    pub fn scp(&self) -> i32 {
        self.poke.scp()
    }

    pub fn dcp(&self) -> i32 {
        self.poke.dcp()
    }

    pub fn cpm(&self) -> f64 {
        self.poke.cpm()
    }

    pub fn stats(&self) -> Stats {
        self.poke.stats()
    }

    pub fn base_hp(&self) -> i32 {
        self.poke.hp()
    }

    pub fn hp(&self) -> i32 {
        self.hp
    }

    /// ポケモンは瀕死か？
    pub fn is_faint(&self) -> bool {
        self.hp <= 0
    }

    /// タイプ相性ボーナスを無効にする
    pub fn disable_type_effect(&mut self) {
        self.is_disable_type_effect = true;
    }

    /// ステータス変化
    pub fn add_buff(&mut self, buff_atk: i32, buff_def: i32) -> (i32, i32) {
        let prev = self.buff;

        self.buff = (std::cmp::max(-4, std::cmp::min(self.buff.0 + buff_atk, 4)),
                     std::cmp::max(-4, std::cmp::min(self.buff.1 + buff_def, 4)));

        (self.buff.0 - prev.0, self.buff.1 - prev.1)
    }

    pub fn can_charge_move(&self, i: usize) -> bool {
        match i {
            0 => self.can_charge_move1(),
            1 => self.can_charge_move2(),
            _ => false,
        }
    }


    /// スペシャルアタック1を実行できるか？
    pub fn can_charge_move1(&self) -> bool {
        self.energy >= self.charge_move1().energy()
    }

    /// スペシャルアタック2を実行できるか？
    pub fn can_charge_move2(&self) -> bool {
        if let Some(mv) = self.charge_move2() {
            self.energy >= mv.energy()
        } else {
            false
        }
    }

    /// ノーマルアタックを実行する
    pub fn do_fast_move(&mut self, opponent: &mut Self) -> i32 {
        let mv = self.fast_move();
        let power = mv.real_power(&self.types());  // 威力(タイプ一致を含む)
        let attack = self.stats().attack * rank_mul(self.buff.0);
        let defense = opponent.stats().defense * rank_mul(opponent.buff.1);

        // ダメージ補正 = タイプ相性 × タイプ一致ボーナス(STAB) × トレーナーバトル
        let type_effect = if self.is_disable_type_effect {  // タイプ相性
            1.0
        } else {
            mv.mtype().type_effect_bonus(&opponent.types())
        };

        let damage_m = type_effect * TRAINER_BATTLE_BONUS;

        let damage = (0.5 * power * (attack / defense) * damage_m).floor() as i32 + 1;

        debug_print!("Debug: {} [fast_move {}] 威力 = {:.1}, こうげき = {:.1}, ぼうぎょ = {:.1}",
                     self.name(), mv.name(), power, attack, defense);
        debug_println!(", タイプ相性 = {}, ダメージ = {}", type_effect, damage);

        opponent.hp = std::cmp::max(opponent.hp - damage, 0);
        self.energy = std::cmp::min(self.energy + mv.energy(), 100);

        damage
    }

    /// スペシャルアタックを実行する
    /// ダメージを返す
    pub fn do_charge_move(&mut self, i: usize, opponent: &mut Self, mut cm_bonus: f64, shield: bool) -> i32 {
        let mv = if let Some(mv) = self.charge_move(i) {
            mv
        } else {
            return 0;
        };

        if self.energy < mv.energy() {
            return 0;
        }

        let damage;

        if shield {
            damage = 1;
            debug_println!("Debug: {} [charge_move {}] *** シールド ***", self.name(), mv.name());
        } else {
            let power = mv.real_power(&self.types());  // 威力(タイプ一致を含む)
            let attack = self.stats().attack * rank_mul(self.buff.0);  // 攻撃ステータス * ステータス変化
            let defense = opponent.stats().defense * rank_mul(opponent.buff.1);  // 防御ステータス * ステータス変化

            // ダメージ補正 = タイプ相性 * タイプ一致ボーナス(STAB) * トレーナーバトル * スペシャルアタック
            let type_effect = if self.is_disable_type_effect {  // タイプ相性
                1.0
            } else {
                mv.mtype().type_effect_bonus(&opponent.types())
            };

            // スペシャルアタックボーナス
            if cm_bonus < 0.0 {
                cm_bonus = 0.0;
            } else if cm_bonus > 1.0 {
                cm_bonus = 1.0;
            }

            let damage_m = type_effect * TRAINER_BATTLE_BONUS * cm_bonus;

            damage = (0.5 * power * (attack / defense) * damage_m).floor() as i32 + 1;

            debug_print!("Debug: {} [charge_move {}] 威力 = {:.1}, こうげき = {:.1}, ぼうぎょ = {:.1}",
                         self.name(), mv.name(), power, attack, defense);
            debug_println!(", タイプ相性 = {}, ダメージ = {}", type_effect, damage);
        }

        // ステータス変化
        if let Some(Buff(you_buff_atk, you_buff_def, opponent_buff_atk, opponent_buff_def)) = mv.buff() {
            let mut rng = rand::thread_rng();
            let rand_val = rng.gen::<f32>() * 100.0;

            if rand_val < mv.buff_prob() {
                self.add_buff(you_buff_atk.into(), you_buff_def.into());
                opponent.add_buff(opponent_buff_atk.into(), opponent_buff_def.into());
                debug_println!("Debug: ステータス変化 {:?}", mv.buff());
            }
        }

        opponent.hp = std::cmp::max(opponent.hp - damage, 0);
        self.energy = std::cmp::max(self.energy - mv.energy(), 0);

        damage
    }
}

#[test]
fn test_battle_pokemon() {
    let koko = Pokemon::new("ココロモリ", "エアスラッシュ", "サイコファング", None, 1489, None, (10, 9, 12)).unwrap();

    let mut p = BattlePokemon::new(Arc::new(koko));

    assert_eq!(p.can_charge_move1(), false);
    assert_eq!(p.can_charge_move2(), false);
    p.energy = p.charge_move1().energy();
    assert_eq!(p.can_charge_move1(), true);
}
