//! ポケモンのデータを保持する。

use std::io::Read;
use std::fmt;

use anyhow::Result;
use serde::Deserialize;

use crate::pokepedia::{Pokepedia, pokepedia_by_name};
use crate::types::Type;
use crate::moves::{FastMove, ChargeMove, fast_move_by_name, charge_move_by_name};
use crate::cpm::cpm;

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
    let kure = Pokemon::new("クレセリア", "ねんりき", "みらいよち", None, 0, Some(20.0), (2, 15, 13)).unwrap();

    assert_eq!(kure.cp(), 1500);
    assert_eq!(kure.scp(), 1815);
    assert_eq!(kure.dcp(), 2115);

    let fude = Pokemon::new("フーディン", "ねんりき", "みらいよち", None, 0, Some(18.0), (1, 15, 15)).unwrap();

    assert_eq!(fude.cp(), 1495);
    assert_eq!(fude.scp(), 1279);
    assert_eq!(fude.dcp(), 1132);
}

/// 個体値(Individual Values)
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
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
                ) -> Result<Self, PokemonError> {

        Ok(Pokemon { dict, lv, ivs, fast_move, charge_move1, charge_move2 })
    }

    /// cpがマイナスのときは-cpを超えない、CPとポケモンレベルの最大値を自動で計算する
    pub fn new(name: &str, fast_move: &str, charge_move1: &str, charge_move2: Option<String>,
           mut cp: i32, pokemon_lv: Option<f32>, ivs_tuple: (i32, i32, i32)) -> Result<Self, PokemonError> {
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
                let mut message = format!("{}: 存在しないノーマルアタック(fast_move): {}", dict.name(), fast_move);

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
                let message = format!("{}: 存在しないスペシャルアタック(charge_move1): {}", dict.name(), charge_move1);
                return Err(PokemonError { message });
            },
            Some(mv) => mv,
        };

        let charge_move2 = match charge_move2 {
            None => None,
            Some(mv_str) => {
                match charge_move_by_name(&mv_str) {
                    None => {
                        let message = format!("{}: 存在しないスペシャルアタック(charge_move2): {}", dict.name(), mv_str);
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

    pub fn name(&self) -> &'static str {
        self.dict.name()
    }

    pub fn types(&self) -> Vec<Type> {
        self.dict.types()
    }

    pub fn base_stats(&self) -> Stats {
        self.dict.base_stats()
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

    pub fn charge_move1(&self) -> &'static ChargeMove {
        self.charge_move1
    }

    pub fn charge_move2(&self) -> Option<&'static ChargeMove> {
        self.charge_move2
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

    pub fn print(&self) {
        let stats = self.stats();
        println!("{} CP {} SCP {} Lv {} HP {} Atk {:.1} Def {:.1}",
                 self.name(), self.cp(), self.scp(), self.lv, self.hp(), stats.attack, stats.defense);
    }
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

#[derive(Debug, Deserialize)]
struct PokemonsToml {
    pokemons: Vec<PokemonToml>,
}

#[derive(Debug, Deserialize)]
struct PokemonToml {
    name: String,
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

pub fn load_pokemon<R: Read>(reader: &mut R) -> Result<Vec<Pokemon>> {
    let mut pokemons = vec![];

    let mut contents = String::new();

    let _ = reader.read_to_string(&mut contents);

    let data: PokemonsToml = toml::from_str(&contents)?;

    for d in &data.pokemons {
        let poke = Pokemon::new(&d.name, &d.fast_move, &d.charge_move1, d.charge_move2.clone(), d.cp, None,
                                (d.ivs.attack, d.ivs.defense, d.ivs.stamina));

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
name = "ココロモリ"
cp = 1489
hp = 135
ivs.attack = 10
ivs.defense = 9
ivs.stamina = 12
fast_move = "エアスラッシュ"
charge_move1 = "サイコファング"

[[pokemons]]
name = "キレイハナ"
cp = 1479
#hp = 
ivs.attack = 2
ivs.defense = 15
ivs.stamina = 6
fast_move = "マジカルリーフ"
charge_move1 = "リーフブレード"

[[pokemons]]
name = "ナマズン"
cp = 1474
#hp = 
ivs.attack = 8
ivs.defense = 15
ivs.stamina = 14
fast_move = "みずでっぽう"
charge_move1 = "どろばくだん"
    "#;

    use std::io::Cursor;

    let mut reader = Cursor::new(poke_toml);
    let pokemons = match load_pokemon(&mut reader) {
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
    assert_eq!(p.charge_move2().is_none(), true);
}
