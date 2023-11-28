//! ポケモンのデータを保持する。

use std::io::{Read, Write};
use std::fmt;
use std::collections::HashSet;

use rand::prelude::*;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use skim::prelude::*;

use crate::pokepedia::{Pokepedia, pokepedia_by_name, pokepedia_by_no};
use crate::types::{Type, TYPES};
use crate::moves::{FastMove, ChargeMove, Buff, fast_move_by_name, charge_move_by_name, fast_move_by_no, charge_move_by_no};
use crate::cpm::cpm;
use crate::battle::rank_mul;
use crate::utils::jp_fixed_width_string;

#[derive(Debug, Clone)]
pub struct Pokemon {
    dict: &'static Pokepedia,

    lv: f32,  // ポケモンレベル

    // 個体値
    ivs: IVs,

    // 技
    fast_move: &'static FastMove,
    charge_move1: &'static ChargeMove,
    charge_move2: Option<&'static ChargeMove>,
}

/// ステータス
#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub attack: f64,
    pub defense: f64,
    pub stamina: f64,
}

impl Stats {
    pub const fn new(attack: f64, defense: f64, stamina: f64) -> Self {
        Self { attack, defense, stamina }
    }

    /// selfの種族値からステータスを計算する
    pub fn stats(&self, lv: f32, ivs: IVs) -> Self {
        let cpm = cpm(lv);
        let attack = (self.attack + ivs.attack as f64) * cpm;
        let defense = (self.defense + ivs.defense as f64) * cpm;
        let stamina = (self.stamina + ivs.stamina as f64) * cpm;

        Self { attack, defense, stamina }
    }

    /// CP(Combat Power, 戦闘力)を計算する
    pub fn calc_cp(&self) -> i32 {
        let cp = (self.attack * (self.defense * self.stamina).sqrt() / 10.0) as i32;

        if cp < 10 {
            10
        } else {
            cp
        }
    }

    /// SCP(Standard Combat Power, 標準戦闘力)を計算して返す。
    /// SCPは独自の指標でゲームでは表示されることはない。
    /// SCPは攻撃力・防御力・耐久性をバランスよく表した指標。
    /// トレーナーバトルなど1対1の対戦で参考となる。
    pub fn calc_scp(&self) -> i32 {
        let v = self.attack * self.defense * self.stamina.floor();
        let scp = (v.powf(2.0/3.0) / 10.0) as i32;

        if scp < 10 {
            10
        } else {
            scp
        }
    }

    /// DCP(Defensive Combat Power, 防御的戦闘力)を計算して返す。
    /// DCPは独自の指標でゲームでは表示されることはない。
    /// DCPは防御力と耐久性を重視した指標となる。
    pub fn calc_dcp(&self) -> i32 {
        let v = self.attack * self.defense * self.defense * self.stamina * self.stamina;
        let dcp = (v.powf(2.0/5.0) / 10.0) as i32;

        if dcp < 10 {
            10
        } else {
            dcp
        }
    }
}

#[test]
fn test_calc_index() {
    let kure = Pokemon::new("クレセリア", Some(20.0), (2, 15, 13), "ねんりき", "みらいよち", None, 0).unwrap();

    assert_eq!(kure.cp(), 1500);
    assert_eq!(kure.scp(), 1815);
    assert_eq!(kure.dcp(), 2115);

    let fude = Pokemon::new("フーディン", Some(18.0), (1, 15, 15), "ねんりき", "みらいよち", None, 0).unwrap();

    assert_eq!(fude.cp(), 1495);
    assert_eq!(fude.scp(), 1279);
    assert_eq!(fude.dcp(), 1132);
}

/// 個体値(Individual Values)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct IVs {
    pub attack: i32,
    pub defense: i32,
    pub stamina: i32,
}

impl IVs {
    pub fn new(attack: i32, defense: i32, stamina: i32) -> Result<Self, PokemonError> {
        if !(0..16).contains(&attack)  {
            let message = format!("[IVs::new] 攻撃の個体値(attack_iv)が範囲外(0～15が正常): {}", attack);
            return Err(PokemonError { message });
        }

        if !(0..16).contains(&defense)  {
            let message = format!("[IVs::new] 防御の個体値(defense_iv)が範囲外(0～15が正常): {}", defense);
            return Err(PokemonError { message });
        }

        if !(0..16).contains(&stamina)  {
            let message = format!("[IVs::new] 耐久の個体値(stamina_iv)が範囲外(0～15が正常): {}", stamina);
            return Err(PokemonError { message });
        }

        Ok(Self { attack, defense, stamina })
    }

    pub fn to_tuple(self) -> (i32, i32, i32) {
        (self.attack, self.defense, self.stamina)
    }
}

#[derive(Debug, Clone)]
pub struct PokemonError {
    pub message: String
}

impl fmt::Display for PokemonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PokemonError { }

impl Pokemon {
    pub fn raw_new(dict: &'static Pokepedia, lv: f32, ivs: IVs,
                fast_move: &'static FastMove, charge_move1: &'static ChargeMove,
                charge_move2: Option<&'static ChargeMove>
                ) -> Self {

        Pokemon { dict, lv, ivs, fast_move, charge_move1, charge_move2 }
    }

    pub fn new_by_no(no: &str, pokemon_lv: Option<f32>, ivs_tuple: (i32, i32, i32),
               fast_move_no: &str, charge_move1_no: &str, charge_move2_no: Option<String>,
               cp: i32) -> Result<Self, PokemonError> {
        let dict = match pokepedia_by_no(no) {
            None => {
                let message = format!("存在しないポケモン: no = {}", no);
                return Err(PokemonError { message });
            },
            Some(dict) => dict,
        };

        let fast_move = match fast_move_by_no(fast_move_no) {
            None => {
                let message = format!("{}: 存在しないノーマルアタック: no = {}", dict.name(), fast_move_no);
                return Err(PokemonError { message });
            },
            Some(mv) => mv,
        };

        let charge_move1 = match charge_move_by_no(charge_move1_no) {
            None => {
                let message = format!("{}: 存在しないスペシャルアタック1: no = {}", dict.name(), charge_move1_no);
                return Err(PokemonError { message });
            },
            Some(mv) => mv,
        };

        let charge_move2 = match charge_move2_no {
            None => None,
            Some(mv_no) => {
                match charge_move_by_no(&mv_no) {
                    None => {
                        let message = format!("{}: 存在しないスペシャルアタック2: no = {}", dict.name(), mv_no);
                        return Err(PokemonError { message });
                    },
                    Some(mv) => Some(mv.name().to_string()),
                }
            }
        };

        Self::new(dict.name(), pokemon_lv, ivs_tuple, fast_move.name(), charge_move1.name(),
                  charge_move2, cp)
    }

    pub fn new(name: &str, pokemon_lv: Option<f32>, ivs_tuple: (i32, i32, i32),
               fast_move: &str, charge_move1: &str, charge_move2: Option<String>,
               cp: i32) -> Result<Self, PokemonError> {
        let dict = match pokepedia_by_name(name) {
            None => {
                let message = format!("存在しないポケモン: {}", name);
                return Err(PokemonError { message });
            },
            Some(dict) => dict,
        };

        let ivs = IVs::new(ivs_tuple.0, ivs_tuple.1, ivs_tuple.2)?;

        let lv;

        if let Some(pl) = pokemon_lv {  // pokemon_lv引数があればそれをポケモンレベルとする。
            let int_pl = (pl * 2.0).floor() as usize;
            if !(2..=100).contains(&int_pl) {
                let message = format!("{}: pokemon_lvが1.0から50.0の間でない。: {}", dict.name(), pl);
                return Err(PokemonError { message });
            }

            lv = pl;
        } else {
            lv = match calc_lv(dict, cp, ivs) {
                None => {
                    let mut msgs = vec![];
                    msgs.push(format!("{}: ポケモンレベルの取得に失敗(CPか個体値が間違っている)", dict.name()));

                    let near_ivs = search_near_iv(dict, cp, ivs);
                    if !near_ivs.is_empty() {
                        msgs.push("もしかして、この値?".to_string());

                        for ivs in near_ivs {
                            msgs.push(format!("{:?}", ivs));
                        }
                    }

                    return Err(PokemonError { message: msgs.join("\n") });
                },
                Some(lv) => lv,
            };
        }

        let fast_move = match fast_move_by_name(fast_move) {
            None => {
                let mut message = format!("{}: 存在しないノーマルアタック: {}", dict.name(), fast_move);

                if fast_move == "ウェザーボール" {
                    message += "\n'ウェザーボール(ノーマル)', 'ウェザーボール(ほのお)'";
                    message += "\n'ウェザーボール(こおり)', 'ウェザーボール(いわ)'";
                    message += "\n'ウェザーボール(みず)' のどれか。";
                } else if fast_move == "テクノバスター" {
                    message += "\n'テクノバスター(ノーマル)', 'テクノバスター(ほのお)'";
                    message += "\n'テクノバスター(こおり)', 'テクノバスター(みず)'";
                    message += "\n'テクノバスター(でんき)' のどれか。";
                }

                return Err(PokemonError { message });
            },
            Some(mv) => mv,
        };

        let charge_move1 = match charge_move_by_name(charge_move1) {
            None => {
                let message = format!("{}: 存在しないスペシャルアタック1: {}", dict.name(), charge_move1);
                return Err(PokemonError { message });
            },
            Some(mv) => mv,
        };

        let charge_move2 = match charge_move2 {
            None => None,
            Some(mv_str) => {
                match charge_move_by_name(&mv_str) {
                    None => {
                        let message = format!("{}: 存在しないスペシャルアタック2: {}", dict.name(), mv_str);
                        return Err(PokemonError { message });
                    },
                    Some(mv) => Some(mv),
                }
            }
        };

        Ok(Pokemon { dict, lv, ivs, fast_move, charge_move1, charge_move2 })
    }

    pub fn dict(&self) -> &'static Pokepedia {
        self.dict
    }

    pub fn no(&self) -> &'static str {
        self.dict.no()
    }

    pub fn name(&self) -> &'static str {
        self.dict.name()
    }

    pub fn s_name(&self) -> &'static str {
        self.dict.s_name()
    }

    pub fn types(&self) -> Vec<Type> {
        self.dict.types()
    }

    pub fn base_stats(&self) -> Stats {
        self.dict.base_stats()
    }

    pub fn set_cp(&mut self, cp: i32) -> bool {
        match calc_lv(self.dict, cp, self.ivs) {
            None => false,
            Some(lv) => {
                self.lv = lv;
                true
            },
        }
    }

    pub fn lv(&self) -> f32 {
        self.lv
    }

    pub fn lv_mut(&mut self) -> &mut f32 {
        &mut self.lv
    }

    pub fn ivs(&self) -> IVs {
        self.ivs
    }

    pub fn ivs_mut(&mut self) -> &mut IVs {
        &mut self.ivs
    }

    pub fn fast_move(&self) -> &'static FastMove {
        self.fast_move
    }

    pub fn set_fast_move(&mut self, fast_move: &'static FastMove) {
        self.fast_move = fast_move;
    }

    pub fn charge_move1(&self) -> &'static ChargeMove {
        self.charge_move1
    }

    pub fn set_charge_move1(&mut self, charge_move1: &'static ChargeMove) {
        self.charge_move1 = charge_move1;
    }

    pub fn charge_move2(&self) -> Option<&'static ChargeMove> {
        self.charge_move2
    }

    pub fn set_charge_move2(&mut self, charge_move2: Option<&'static ChargeMove>) {
        self.charge_move2 = charge_move2;
    }

    pub fn cp(&self) -> i32 {
        self.stats().calc_cp()
    }

    pub fn scp(&self) -> i32 {
        self.stats().calc_scp()
    }

    pub fn dcp(&self) -> i32 {
        self.stats().calc_dcp()
    }

    pub fn cpm(&self) -> f64 {
        cpm(self.lv)
    }

    pub fn stats(&self) -> Stats {
        self.dict.base_stats().stats(self.lv, self.ivs)
    }

    pub fn hp(&self) -> i32 {
        self.stats().stamina.floor() as i32
    }

    pub fn fast_move_desc(&self) -> String {
        let mv = self.fast_move();
        format!("({:3.1}, {:3.1}, {}){}", mv.ppt(&self.types(), &Vec::new()), mv.ept(), mv.turns(), mv.name())
    }

    pub fn charge_move1_desc(&self) -> String {
        let mv = self.charge_move1();
        format!("({:3.1}, {:>2}){}", mv.ppe(&self.types(), &Vec::new()), mv.first_enable_turn(self.fast_move), mv.name())
    }

    pub fn charge_move2_desc(&self) -> String {
        if let Some(mv) = self.charge_move2() {
            format!("({:3.1}, {:>2}){}", mv.ppe(&self.types(), &Vec::new()), mv.first_enable_turn(self.fast_move), mv.name())
        } else {
            "None".to_string()
        }
    }

    pub fn format(&self, width: usize) -> String {
        let stats = self.stats();
        let name = jp_fixed_width_string(self.name(), width);

        let fm_desc = self.fast_move_desc();
        let cm1_desc = self.charge_move1_desc();
        let cm2_desc = self.charge_move2_desc();

        format!("{} CP {:>4} SCP {:>4} ECP1 {:>4} Lv {:>4.1} IVs({:>2}, {:>2}, {:>2}) Stats({:>5.1}, {:>5.1}, {:>3}) {} | {} | {}",
                 name, self.cp(), self.scp(), self.avg_ecp(1), self.lv, self.ivs.attack, self.ivs.defense, self.ivs.stamina,
                 stats.attack, stats.defense, self.hp(), fm_desc, cm1_desc, cm2_desc)
    }

    /// 1ターンあたりの平均的なわざの威力を計算する
    /// 戻り値は(PPT, 倒すまでのターン数)
    ///
    /// 攻撃をシミュレートするが、戦略は以下の通り。
    ///
    /// * 相手は引数で指定するが、Noneの場合は自分との対戦。
    ///   自分との対戦では相性の効果は無効とする。
    ///
    /// * シールドはあれば使われるとする。
    ///
    /// * 使用するスペシャルアタックは最初は必要エネルギーが低いほうを使う。
    ///   2回目以降は、PPE(Power Per Energy: エネルギー当たりの威力)が大きいほうを使う。
    ///
    /// * 相手のHPを0にしたときの威力は、そのままの値を威力として使う。
    ///   たとえば、最後の攻撃でダメージを50与えられるとすると、
    ///   相手のHPが1残っていようが、49残っていようが、計算に使用する威力の値は同じとする。
    ///   つまり、相手のHPは実際にはもともとのHPを初めて超えた総ダメージとする。
    ///   説明が下手で何を言っているかわかりにくいと思う。
    ///
    /// * 自分の防御のステータス変化は相手の防御への逆ステータス変化とする。
    ///   相手の攻撃のステータス変化は自分の攻撃への逆ステータス変化とする。
    pub fn calc_power_per_turn(&self, opponent: Option<&Pokemon>, custom_types: Option<Vec<Type>>, mut num_shields: i32) -> (f64, i32) {
        let types;
        let defender;

        if let Some(p) = opponent {
            types = p.types();
            defender = p;
        } else {
            if let Some(v) = custom_types {
                types = v;
            } else {
                types = Vec::new();  // 自分との対戦では相性を無視する
            }

            defender = self;
        }

        let min_hp = defender.hp();

        let mut total_damage = 0;
        let mut num_turns = 0;
        let mut sum_power = 0.0;
        let mut energy = 0;

        let shield_charge_move;  // 相手にシールドがある間に使うスペシャルアタック
        let charge_move;

        if let Some(mv2) = defender.charge_move2() {
            let mv1 = defender.charge_move1();

            shield_charge_move = if mv1.energy() < mv2.energy() { mv1 } else { mv2 };

            let ppe1 = mv1.ppe(&self.types(), &types);
            let ppe2 = mv2.ppe(&self.types(), &types);

            charge_move = if ppe1 > ppe2 { mv1 } else { mv2 };
        } else {
            shield_charge_move = defender.charge_move1();
            charge_move = defender.charge_move1();
        }

        let atk = self.stats().attack;
        let def = defender.stats().defense;
        let mut atk_buff: i8 = 0;
        let mut def_buff: i8 = 0;

        let seed = [0u8; 32];
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);

        while total_damage < min_hp {
            let mv = if num_shields > 0 { shield_charge_move } else { charge_move };

            let damage;

            if energy >= mv.energy() {
                energy = std::cmp::max(0, energy - mv.energy());
                let power = mv.real_power2(&self.types(), &types);

                if num_shields > 0 {
                    sum_power += 1.0;   // シールドを張られた時の威力は1とする
                    damage = 1;
                    num_shields -= 1;
                } else {
                    sum_power += power;
                    damage = calc_damage(power, atk * rank_mul(atk_buff as i32), def * rank_mul(def_buff as i32));
                }

                // ステータス変化
                if let Some(Buff(you_buff_atk, you_buff_def, opponent_buff_atk, opponent_buff_def)) = mv.buff() {
                    let rand_val = rng.gen::<f32>() * 100.0;

                    if rand_val < mv.buff_prob() {
                        if you_buff_atk != 0 {
                            atk_buff = std::cmp::max(-4, std::cmp::min(atk_buff + you_buff_atk, 4))
                        }

                        if you_buff_def != 0 {
                            def_buff = std::cmp::max(-4, std::cmp::min(def_buff - you_buff_def, 4))
                        }

                        if opponent_buff_atk != 0 {
                            atk_buff = std::cmp::max(-4, std::cmp::min(atk_buff - opponent_buff_atk, 4))
                        }

                        if opponent_buff_def != 0 {
                            def_buff = std::cmp::max(-4, std::cmp::min(def_buff + opponent_buff_def, 4))
                        }
                    }
                }

                num_turns += 1;
            } else {
                let fast_move = self.fast_move();
                energy = std::cmp::min(energy + fast_move.energy(), 100);
                let power = fast_move.real_power2(&self.types(), &types);
                sum_power += power;
                damage = calc_damage(power, atk * rank_mul(atk_buff as i32), def * rank_mul(def_buff as i32));
                num_turns += fast_move.turns();
            }

            total_damage += damage;
        }

        (sum_power / num_turns as f64, num_turns)
    }

    /// ECP(Extended Combat Power, 拡張戦闘力)を計算して返す。
    /// ECPは自分が考えた指標でゲームでは表示されることはない。
    /// ECPは攻撃力・防御力・耐久性に加えて、技の威力、ステータス変化、タイプ相性も考慮に入れる。
    pub fn ecp(&self, opponent: Option<&Pokemon>, custom_types: Option<Vec<Type>>, num_shields: i32) -> i32 {
        let (ppt, _) = self.calc_power_per_turn(opponent, custom_types, num_shields);
        let sc_ppt = ppt * PPT_SCALE;

        let stats = self.stats();
        let ecp = ((sc_ppt * stats.attack * stats.defense * stats.stamina.floor()).sqrt() / 10.0) as i32;

        if ecp < 10 {
            10
        } else {
            ecp
        }
    }

    /// すべてのタイプの平均ECP
    pub fn avg_ecp(&self, num_shields: i32) -> i32 {
        let mut v = Vec::new();

        for t in TYPES {
            let types = vec![t];

            v.push(self.ecp(None, Some(types), num_shields));
        }

        v.iter().sum::<i32>() / v.len() as i32
    }

    pub fn move_perm(&self) -> Vec<Pokemon> {
        let mut v = Vec::new();
        let mut set = HashSet::new();

        for fast_move in self.dict.fast_moves() {
            for charge_move1 in self.dict.charge_moves() {
                for charge_move2 in self.dict.charge_moves() {
                    if charge_move1.no() == charge_move2.no() {
                        continue;
                    }

                    // (A, B)と(B, A)のときは片方しか処理しない
                    if set.contains(&(charge_move1.no().to_string() + charge_move2.no())) {
                        continue;
                    }

                    v.push(Pokemon { dict: self.dict, lv: self.lv, ivs: self.ivs, fast_move, charge_move1, charge_move2: Some(charge_move2) });

                    set.insert(charge_move2.no().to_string() + charge_move1.no());
                }
            }
        }

        v
    }
}

const PPT_SCALE: f64 = 16.0;

fn calc_damage(power: f64, attack: f64, defense: f64) -> i32 {
    (0.5 * crate::battle::TRAINER_BATTLE_BONUS * power * (attack / defense)).floor() as i32 + 1
}

const MAX_ACP_TURNS: i32 = 128;

#[test]
fn test_calc_power_per_turn() {
    let kure = Pokemon::new("クレセリア", Some(20.0), (2, 15, 13), "ねんりき", "みらいよち", None, 0).unwrap();
    let fude = Pokemon::new("フーディン", Some(18.0), (1, 15, 15), "ねんりき", "みらいよち", None, 0).unwrap();

    let (ppt0, num_turns0) = kure.calc_power_per_turn(Some(&fude), None, 1);
    assert_eq!(ppt0, 4.8478260869565215);
    assert_eq!(num_turns0, 46);

    let (ppt1, num_turns1) = fude.calc_power_per_turn(Some(&kure), None, 1);
    assert_eq!(ppt1, 4.4655172413793105);
    assert_eq!(num_turns1, 58);
}

/// 引数として渡された種族値、CP、個体値からポケモンレベルを計算して返す。
pub fn calc_lv(poke: &Pokepedia, cp: i32, ivs: IVs) -> Option<f32> {
    for i in 2..=100 {
        let lv = i as f32 / 2.0;

        let stats = poke.base_stats().stats(lv, ivs);

        if cp == stats.calc_cp() {
            return Some(lv);
        }
    }

    None
}

#[test]
fn test_calc_lv() {
    let saza = pokepedia_by_name("サザンドラ").unwrap();
    let ivs = IVs::new(10, 14, 14).unwrap();
    assert_eq!(calc_lv(saza, 2276, ivs), Some(22.5));
    assert_eq!(calc_lv(saza, 2277, ivs), None);
}

#[derive(Debug, Serialize, Deserialize)]
struct PokemonsToml {
    pokemons: Vec<PokemonToml>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PokemonToml {
    no: String,
    cp: i32,
    hp: Option<i32>,

    // 個体値(0～15)
    ivs: IVs,

    // 技
    fast_move: String,
    charge_move1: String,
    charge_move2: Option<String>,
}

pub fn search_near_iv(poke: &Pokepedia, cp: i32, ivs: IVs) -> Vec<IVs> {
    let mut near_ivs = vec![];

    let near_attack_iv = vec![ivs.attack-1, ivs.attack, ivs.attack+1].into_iter().filter(|v| 0 <= *v && *v <= 15).collect::<Vec<_>>();
    let near_defense_iv = vec![ivs.defense-1, ivs.defense, ivs.defense+1].into_iter().filter(|v| 0 <= *v && *v <= 15).collect::<Vec<_>>();
    let near_stamina_iv = vec![ivs.stamina-1, ivs.stamina, ivs.stamina+1].into_iter().filter(|v| 0 <= *v && *v <= 15).collect::<Vec<_>>();

    for a in &near_attack_iv {
        for d in &near_defense_iv {
            for s in &near_stamina_iv {
                let v = IVs::new(*a, *d, *s).unwrap();

                if calc_lv(poke, cp, v).is_some() {
                    near_ivs.push(v);
                }
            }
        }
    }

    near_ivs
}

pub fn load_pokemons<R: Read>(reader: &mut R) -> Result<Vec<Pokemon>> {
    let mut pokemons = vec![];

    let mut contents = String::new();

    let _ = reader.read_to_string(&mut contents);

    let data: PokemonsToml = toml::from_str(&contents)?;

    for d in &data.pokemons {
        let poke = Pokemon::new_by_no(&d.no, None, (d.ivs.attack, d.ivs.defense, d.ivs.stamina),
                                &d.fast_move, &d.charge_move1, d.charge_move2.clone(), d.cp);

        match poke {
            Ok(poke) => {
                if let Some(hp) = d.hp {
                    if hp != poke.hp() {
                        eprintln!("{} HPが一致しない, データ: {}, 計算結果: {}", poke.name(), hp, poke.hp());
                        continue;
                    }
                }

                pokemons.push(poke)
            },
            Err(err) => println!("{}", err),
        }
    }

    Ok(pokemons)
}

#[test]
fn test_load_pokemon() {
    let poke_toml = r#"
[[pokemons]]
no = "0528"
cp = 1489
hp = 135
fast_move = "255"
charge_move1 = "353"
charge_move2 = "275"

[pokemons.ivs]
attack = 10
defense = 9
stamina = 12

[[pokemons]]
no = "0182"
cp = 1479
hp = 124
fast_move = "357"
charge_move1 = "117"

[pokemons.ivs]
attack = 2
defense = 15
stamina = 6

[[pokemons]]
no = "0340"
cp = 1474
hp = 174
fast_move = "230"
charge_move1 = "096"

[pokemons.ivs]
attack = 8
defense = 15
stamina = 14
    "#;

    use std::io::Cursor;

    let mut reader = Cursor::new(poke_toml);
    let pokemons = match load_pokemons(&mut reader) {
        Err(err) => panic!("{}", err),
        Ok(v) => v,
    };

    assert_eq!(pokemons.len(), 3);

    let p = &pokemons[0];

    assert_eq!(p.name(), "ココロモリ");
    assert_eq!(p.cp(), 1489);
    assert_eq!(p.lv(), 34.5);

    let ivs = p.ivs();
    assert_eq!(ivs.attack, 10);
    assert_eq!(ivs.defense, 9);
    assert_eq!(ivs.stamina, 12);

    assert_eq!(p.cpm(), 0.7586303702);

    let stats = p.stats();
    assert_eq!(stats.attack, 129.7257933042);
    assert_eq!(stats.defense, 97.1046873856);
    assert_eq!(stats.stamina, 135.7948362658);

    assert_eq!(p.hp(), 135);
    assert_eq!(p.fast_move().name(), "エアスラッシュ");
    assert_eq!(p.charge_move1().name(), "サイコファング");
    assert_eq!(p.charge_move2().unwrap().name(), "みらいよち");
}

pub fn save_pokemons<W: Write>(writer: &mut W, pokemons: &Vec<Pokemon>) -> Result<()> {

    let mut v = Vec::new();

    for p in pokemons {
        let no = p.no().to_string();
        let fast_move = p.fast_move().no().to_string();
        let charge_move1 = p.charge_move1().no().to_string();

        let charge_move2 = p.charge_move2.map(|mv| String::from(mv.no()));

        let ptoml = PokemonToml { no, cp: p.cp(), hp: Some(p.hp()), ivs: p.ivs,
                      fast_move, charge_move1, charge_move2 };

        v.push(ptoml);
    }

    let data = PokemonsToml { pokemons: v };

    let contents = toml::to_string(&data)?;

    writer.write_all(&contents.into_bytes())?;

    writer.flush()?;

    Ok(())
}

struct PokemonItem {
    display_str: String,
    output_index: String,
    search_str: String,
}

impl PokemonItem {
    fn new(p: &Pokemon, i: usize, width: usize) -> Self {
        Self {
            display_str: p.format(width),
            output_index: i.to_string(),
            search_str: (p.name().to_owned() + p.s_name()).to_string(),
        }
    }
}

impl SkimItem for PokemonItem {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.search_str)
    }

    fn display<'a>(&'a self, _context: DisplayContext<'a>) -> AnsiString<'a> {
        AnsiString::from(&*self.display_str)
    }

    fn output(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.output_index)
    }
}

pub fn skim_pokemons(pokemons: &[Pokemon], width: usize) -> Option<usize> {
    let options = SkimOptions::default();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

    for (i, p) in pokemons.iter().enumerate() {
        let _ = tx_item.send(Arc::new(PokemonItem::new(p, i, width)));
    }

    drop(tx_item);

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_default();

    if selected_items.len() == 1 {
        match selected_items[0].output().parse::<usize>() {
            Ok(i) => Some(i),
            Err(_err) => None,
        }
    } else {
        None
    }
}
