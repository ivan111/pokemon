//! トレーナーバトルのシミュレーション
//! AIでの自動対戦のみに対応する。エミュレーションはしない。

use std::cmp::Ordering;
use std::sync::Arc;

use rand::prelude::*;

use crate::pokemon::Pokemon;
use crate::moves::*;

macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

pub const MS_PER_TURN: i32 = 500;  // 500 ミリ秒/ターン
pub const TURN_PER_SEC: i32 = 2;  // 2 ターン/秒
pub const LIMIT_TURN: i32 = 4 * 60 * TURN_PER_SEC;  // 制限ターン数(4分)

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
}

// TODO: 制限時間4分

#[derive(Clone)]
pub struct State {
    pub player0: Player,
    pub player1: Player,
    pub phase: Phase,

    pub turn: i32,  // 現在のターン数
    pub msgs: Vec<String>,
}

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub team: Vec<BattlePokemon>,
    pub cur_poke: usize,  // 現在のポケモン。インデックス
    pub num_shields: i32,  // シールドの数
    pub switch_turns: i32,  // ポケモンを交代可能になるまでのターン数。0なら可能

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

impl Battle {
    pub fn new(name0: String, team0: Vec<Pokemon>, name1: String, team1: Vec<Pokemon>) -> Self {
        assert!(team0.len() >= 1 && team1.len() >= 1);

        let player0 = Player::new(name0, team0);
        let player1 = Player::new(name1, team1);

        let start_state = State {
            player0,
            player1,
            phase: Phase::Neutral,
            turn: 0,
            msgs: vec![],
        };

        Battle {
            states: vec![start_state],
            actions: vec![],
        }
    }

    pub fn get_state(&self) -> &State {
        self.states.last().unwrap()
    }

    pub fn is_ended(&self) -> bool {
        let state = self.get_state();

        state.player0.is_ended() || state.player1.is_ended()
    }

    pub fn get_possible_actions(&self, you: &Player, opponent: &Player) -> Vec<Action> {
        let mut actions = vec![];

        let state = self.get_state();

        /*
        match state.phase {
            Phase::GameOver(_) | Phase::TimeOver(_) => (),

            Phase::SuspendSwitch(player_i @ (0 | 1), num_turns @ ..=0) => {  // 強制入れ替え
            }
        }
        */

        actions
    }

    /// 戻り値: true: ゲーム継続中, false: ゲーム終了
    pub fn do_action(&mut self, actions: [Action; 2]) -> bool {
        let mut state = self.get_state().clone();

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
        debug_println!("Debug: turn {} start --------------------------------------", state.turn);
        debug_println!("Debug: action = [{:?}, {:?}]", actions[0], actions[1]);
        debug_println!("Debug: phase = {:?}", state.phase);

        for p in state.players() {
            debug_println!("Debug: player {}: dur_turns = {}, switch_turns = {}", p.name, p.dur_turns, p.switch_turns);

            for poke in &p.team {
                debug_println!("Debug:       {}: HP = {} / {}, energy = {}, buff = {:?}", poke.poke.poke.name, poke.hp, poke.poke.hp, poke.energy, poke.buff);

                /*
                let mv = poke.poke.fast_move;
                debug_println!("Debug:               {}: power = {}, energy = {}, turns = {}", mv.name, mv.tb_power, mv.tb_energy, mv.tb_turns);

                let mv = poke.poke.charge_move1;
                debug_println!("Debug:               {}: power = {}, energy = {}, buff = {:?}, buff_probt = {}", mv.name, mv.tb_power, mv.tb_energy, mv.tb_buff, mv.tb_buff_prob);

                if let Some(mv) = poke.poke.charge_move2 {
                    debug_println!("Debug:               {}: power = {}, energy = {}, buff = {:?}, buff_prob = {}", mv.name, mv.tb_power, mv.tb_energy, mv.tb_buff, mv.tb_buff_prob);
                }
                */

                debug_println!("Debug:       in_fast_move = {}, dur_turns =  {}", p.in_fast_move, p.dur_turns);
            }
        }

        debug_println!();

        match state.phase {
            Phase::GameOver(_) | Phase::TimeOver(_) => (),

            Phase::Neutral => {
                // 硬直時間が過ぎたらノーマルアタックのダメージを与える
                state.do_fast_move();

                for (player_i, p) in state.players_mut().into_iter().enumerate() {
                    if p.in_fast_move {  // 硬直中
                        // 硬直中に交代やスペシャルアタックをすると、硬直後に実行される。
                        // 硬直中に交代とスペシャルアタックを両方すると、硬直後に代する。
                        debug_println!("Debug: {}は硬直中", p.poke_name());
                    }

                    match actions[player_i] {
                        Action::None => continue,

                        Action::SwitchPokemon(poke_i) => {
                            if p.switch_pokemon(poke_i.into()) {
                                debug_println!("Debug: 入れ替え player = {}, 出てきたポケモン = {}", p.name, p.poke_name());
                            }
                        },

                        Action::FastMove => {
                            if !p.in_fast_move {  // 硬直中でないなら
                                p.in_fast_move = true;
                                p.dur_turns = p.poke().poke.fast_move.tb_turns - 1;

                                debug_println!("Debug: {}は{}を使った。", p.poke_name(), p.poke().poke.fast_move.name);
                            }
                        },

                        Action::ChargeMove(i @ (0 | 1)) => {
                            if let Some(mv) = p.poke().get_charge_move(i as usize) {
                                if p.poke().can_charge_move(i as usize) {
                                    if p.in_fast_move {
                                        // TODO pending
                                    } else {
                                        use_charge_move[player_i] = i as usize;
                                    }
                                } else {
                                    debug_println!("Debug: [{}] {}に必要なエネルギーが足りない", p.name, mv.name);
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

        let mut next_turn = state.turn + 1;

        // 1ターン技があるなら、このターンに発動する
        state.do_fast_move();
        next_turn += state.check_faint();

        next_turn += state.do_charge_move(use_charge_move);
        next_turn += state.check_faint();

        state.set_gameover_phase();

        state.increment_turns(next_turn);

        debug_println!("Debug: turn {} end ----------------------------------------", state.turn);

        state.turn = next_turn;

        let phase = state.phase.clone();

        self.states.push(state);
        self.actions.push(actions);

        match phase {
            Phase::GameOver(_) | Phase::TimeOver(_) => false,
            _ => true
        }
    }
}

pub const CHARGE_MOVE_TURNS: i32 = 20;
pub const FORCE_SWITCH_TURNS: i32 = 10;

impl State {
    fn get_elapsed_ms(&self) -> i32 {
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

    fn do_fast_move(&mut self) {
        let p0 = &mut self.player0;
        let p1 = &mut self.player1;

        if p0.in_fast_move && p0.dur_turns == 0 {
            p0.team[p0.cur_poke].fast_move(&mut p1.team[p1.cur_poke]);
            p0.in_fast_move = false;
        }

        if p1.in_fast_move && p1.dur_turns == 0 {
            p1.team[p1.cur_poke].fast_move(&mut p0.team[p0.cur_poke]);
            p1.in_fast_move = false;
        }
    }

    fn do_charge_move(&mut self, mut use_charge_move: [usize; 2]) -> i32 {
        assert!((0..=2).contains(&use_charge_move[0]) && (0..=2).contains(&use_charge_move[1]));

        let incr_turns;

        let poke0 = self.player0.poke_mut();
        let poke1 = self.player1.poke_mut();

        if poke0.is_faint() {
            use_charge_move[0] = 2;
        }

        if poke1.is_faint() {
            use_charge_move[1] = 2;
        }

        match use_charge_move {
            [i0 @ (0 | 1), i1 @ (0 | 1)] => {  // ２人とも使う
                let atk0 = poke0.poke.attack.floor() as u32;
                let atk1 = poke1.poke.attack.floor() as u32;

                match atk0.cmp(&atk1) {
                    Ordering::Less => {
                        poke1.charge_move(i1, poke0, 1.0);
                        // TODO シールド
                        poke0.charge_move(i0, poke1, 1.0);
                    },
                    Ordering::Greater => {
                        poke0.charge_move(i0, poke1, 1.0);
                        poke1.charge_move(i1, poke0, 1.0);
                    },
                    Ordering::Equal => {  // random
                        if rand::random() {
                            poke0.charge_move(i0, poke1, 1.0);
                            poke1.charge_move(i1, poke0, 1.0);
                        } else {
                            poke1.charge_move(i1, poke0, 1.0);
                            poke0.charge_move(i0, poke1, 1.0);
                        }
                    },
                }

                incr_turns = CHARGE_MOVE_TURNS * 2;
            },

            [i @ (0 | 1), 2] => {
                poke0.charge_move(i, poke1, 1.0);
                incr_turns = CHARGE_MOVE_TURNS;
            }

            [2, i @ (0 | 1)] => {
                poke1.charge_move(i, poke0, 1.0);
                incr_turns = CHARGE_MOVE_TURNS;
            }

            _ => {
                incr_turns = 0;
            },
        }

        incr_turns
    }

    fn check_faint(&mut self) -> i32 {
        let mut incr_turns = 0;

        for (player_i, p) in self.players_mut().into_iter().enumerate() {
            if p.poke().is_faint() {
                if p.get_num_remains() > 0 {
                    p.force_switch();
                    incr_turns = FORCE_SWITCH_TURNS;
                }
            }
        }

        incr_turns
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

    fn increment_turns(&mut self, mut next_turn: i32) {
        next_turn = std::cmp::max(next_turn, self.turn + 1);  // 最低でも1はターンを進める

        if next_turn > LIMIT_TURN {
            // TODO
            //self.phase = Phase::TimeOver();
        }

        let num_turns = next_turn - self.turn;

        for i in 0..2 {
            let p = self.player_mut(i);

            p.dur_turns = std::cmp::max(p.dur_turns - num_turns, 0);
            p.switch_turns = std::cmp::max(p.switch_turns - num_turns, 0);
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
            switch_turns: 0,
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
        self.team[self.cur_poke].poke.poke.name
    }

    pub fn get_num_remains(&self) -> i32 {
        self.team.iter().filter(|p| p.hp > 0).count().try_into().unwrap()
    }

    pub fn is_ended(&self) -> bool {
        self.team.iter().all(|p| p.hp <= 0)
    }

    pub fn switch_pokemon(&mut self, i: usize) -> bool {
        if self.switch_turns > 0 {
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
        self.switch_turns = 60 * TURN_PER_SEC;  // これから1分間交代できない

        // リセット
        self.in_fast_move = false;
        self.dur_turns = 0;

        return true;
    }

    pub fn force_switch(&mut self) {
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

            return;
        }

        panic!("force_switch: 入れ替えるポケモンがいない");
    }
}

#[test]
fn test_player() {
    let p0 = Pokemon::new("ココロモリ", "エアスラッシュ", "サイコファング", None, 1489, None, 10, 9, 12).unwrap();
    let p1 = Pokemon::new("ブラッキー", "バークアウト", "あくのはどう", None, 1498, None, 2, 14, 0).unwrap();
    let p2 = Pokemon::new("ナマズン", "みずでっぽう", "どろばくだん", None, 1474, None, 8, 15, 14).unwrap();

    let mut p = Player::new(String::from("test"), vec![p0, p1, p2]);

    assert_eq!(p.poke_name(), "ココロモリ");
    assert_eq!(p.poke().poke.fast_move.name, "エアスラッシュ");
    assert_eq!(p.poke().poke.charge_move1.name, "サイコファング");
    assert_eq!(p.poke().poke.charge_move2.is_none(), true);

    p.team[0].add_buff(1, 1);
    assert_eq!(p.poke().buff, (1, 1));

    p.switch_pokemon(2);
    assert_eq!(p.poke_name(), "ナマズン");
    assert_eq!(p.team[0].buff, (0, 0));  // ステータス変化がリセットされているか？

    p.force_switch();
    assert_eq!(p.poke_name(), "ブラッキー");

    assert_eq!(p.get_num_remains(), 3);
    assert_eq!(p.is_ended(), false);
}

/// ランク補正
pub const RANK_MUL: [f64; 9] = [0.5, 4.0/7.0, 2.0/3.0, 4.0/5.0, 1.0, 5.0/4.0, 3.0/2.0, 7.0/4.0, 2.0];

pub fn get_rank_mul(buff: i32) -> f64 {
    assert!((-4..=4).contains(&buff));

    RANK_MUL[buff as usize]
}

pub const TRAINER_BATTLE_BONUS: f64 = 1.3;  // トレーナーバトルボーナス

impl BattlePokemon {
    pub fn new(poke: Arc<Pokemon>) -> Self {
        let hp = poke.hp;

        BattlePokemon {
            poke,
            hp,
            energy: 0,
            buff: (0, 0),
            is_disable_type_effect: false,
        }
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

    // iが0ならスペシャルアタック1を返す
    // iが1ならスペシャルアタック2を返す
    pub fn get_charge_move(&self, i: usize) -> Option<&'static ChargeMove> {
        match i {
            0 => Some(self.poke.charge_move1),
            1 => self.poke.charge_move2,
            _ => None,
        }
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
        self.energy >= self.poke.charge_move1.tb_energy
    }

    /// スペシャルアタック2を実行できるか？
    pub fn can_charge_move2(&self) -> bool {
        if let Some(mv) = self.poke.charge_move2 {
            self.energy >= mv.tb_energy
        } else {
            false
        }
    }

    /// ノーマルアタックを実行する
    pub fn fast_move(&mut self, opponent: &mut Self) -> i32 {
        let mv = self.poke.fast_move;
        let power = mv.tb_power as f64;
        let attack = self.poke.attack * get_rank_mul(self.buff.0);
        let defense = opponent.poke.defense * get_rank_mul(opponent.buff.1);

        // ダメージ補正 = タイプ相性 × タイプ一致ボーナス(STAB) × トレーナーバトル
        let type_effect = if self.is_disable_type_effect {  // タイプ相性
            1.0
        } else {
            mv.mtype.get_type_effect_bonus(&opponent.poke.poke.get_types())
        };

        let stab = if self.poke.is_stab_fast_move { STAB } else { 1.0 };
        let damage_m = type_effect * stab * TRAINER_BATTLE_BONUS;

        let damage = (0.5 * power * (attack / defense) * damage_m).floor() as i32 + 1;

        debug_println!("Debug: [fast_move] 威力 = {}, こうげき = {:.1}, ぼうぎょ = {:.1},", power, attack, defense);
        debug_println!("Debug:         タイプ相性 = {}, STAB = {}, ダメージ = {},", type_effect, stab, damage);

        opponent.hp = std::cmp::max(opponent.hp - damage, 0);
        self.energy = std::cmp::min(self.energy + (mv.tb_energy * mv.tb_turns), 100);

        damage
    }

    /// スペシャルアタックを実行する
    /// ダメージを返す
    pub fn charge_move(&mut self, i: usize, opponent: &mut Self, mut cm_bonus: f64) -> i32 {
        let mv = if let Some(mv) = self.get_charge_move(i) {
            mv
        } else {
            return 0;
        };

        if self.energy < mv.tb_energy {
            return 0;
        }

        let power = mv.tb_power as f64;  // 威力
        let attack = self.poke.attack * get_rank_mul(self.buff.0);  // 攻撃ステータス * ステータス変化
        let defense = opponent.poke.defense * get_rank_mul(opponent.buff.1);  // 防御ステータス * ステータス変化

        // ダメージ補正 = タイプ相性 * タイプ一致ボーナス(STAB) * トレーナーバトル * スペシャルアタック
        let type_effect = if self.is_disable_type_effect {  // タイプ相性
            1.0
        } else {
            mv.mtype.get_type_effect_bonus(&opponent.poke.poke.get_types())
        };

        let stab = if self.poke.is_stab_fast_move { STAB } else { 1.0 };

        // スペシャルアタックボーナス
        if cm_bonus < 0.0 {
            cm_bonus = 0.0;
        } else if cm_bonus > 1.0 {
            cm_bonus = 1.0;
        }

        let damage_m = type_effect * stab * TRAINER_BATTLE_BONUS * cm_bonus;

        let damage = (0.5 * power * (attack / defense) * damage_m).floor() as i32 + 1;

        debug_println!("Debug: [charge_move] 威力 = {}, こうげき = {:.1}, ぼうぎょ = {:.1},", power, attack, defense);
        debug_println!("Debug:         タイプ相性 = {}, STAB = {}, ダメージ = {},", type_effect, stab, damage);

        // ステータス変化
        if let Some(Buff(you_buff_atk, you_buff_def, opponent_buff_atk, opponent_buff_def)) = mv.tb_buff {
            let mut rng = rand::thread_rng();
            let rand_val = rng.gen::<f32>() * 100.0;

            if rand_val < mv.tb_buff_prob {
                self.add_buff(you_buff_atk.into(), you_buff_def.into());
                opponent.add_buff(opponent_buff_atk.into(), opponent_buff_def.into());
                debug_println!("Debug:    ステータス変化 {:?}", mv.tb_buff);
            }
        }

        opponent.hp = std::cmp::max(opponent.hp - damage, 0);
        self.energy = std::cmp::max(self.energy - mv.tb_energy, 0);

        damage
    }

    /// ポケモンは瀕死か？
    pub fn is_faint(&self) -> bool {
        self.hp <= 0
    }
}

#[test]
fn test_battle_pokemon() {
    let koko = Pokemon::new("ココロモリ", "エアスラッシュ", "サイコファング", None, 1489, None, 10, 9, 12).unwrap();

    let mut p = BattlePokemon::new(Arc::new(koko));

    assert_eq!(p.can_charge_move1(), false);
    assert_eq!(p.can_charge_move2(), false);
    p.energy = p.poke.charge_move1.tb_energy;
    assert_eq!(p.can_charge_move1(), true);

    let blak = Pokemon::new("ブラッキー", "バークアウト", "あくのはどう", None, 1498, None, 2, 14, 0).unwrap();

    let mut p2 = BattlePokemon::new(Arc::new(blak));
}
