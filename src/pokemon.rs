//! ポケモンのデータを保持する。

use std::io::Read;
use std::fmt;

use serde::Deserialize;

use crate::pokepedia::*;
use crate::moves::*;
use crate::cpm::*;
use crate::index::*;

#[derive(Debug, Clone)]
pub struct Pokemon {
    pub poke: &'static Pokepedia,

    pub lv: f32,

    // 指標
    pub cp: i32,
    pub scp: i32,
    pub dcp: i32,

    // 個体値
    pub attack_iv: i32,
    pub defense_iv: i32,
    pub stamina_iv: i32,

    // ステータス
    pub cpm: f64,
    pub attack: f64,  // (種族値(攻撃) + attack_iv) * CP補正値
    pub defense: f64,  // (種族値(防御) + defense_iv) * CP補正値
    pub stamina: f64,  // (種族値(耐久) + stamina_iv) * CP補正値
    pub hp: i32,  // floor((種族値(耐久) + stamina_iv) * CP補正値)

    // 技
    pub fast_move: &'static FastMove,
    pub is_stab_fast_move: bool,  // STAB(Same Type Attack Bonus, タイプ一致ボーナス)

    pub charge_move1: &'static ChargeMove,
    pub is_stab_charge_move1: bool,

    pub charge_move2: Option<&'static ChargeMove>,
    pub is_stab_charge_move2: bool,
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
    /// cpがマイナスのときは-cpを超えない、CPとポケモンレベルの最大値を自動で計算する
    pub fn new(name: &str, fast_move: &str, charge_move1: &str, charge_move2: Option<String>,
           mut cp: i32, pokemon_lv: Option<f32>, attack_iv: i32, defense_iv: i32, stamina_iv: i32) -> Result<Self, PokemonError> {
        let poke = match get_pokepedia_by_name(name) {
            None => {
                let message = format!("存在しないポケモン: {}", name);
                return Err(PokemonError { message });
            },
            Some(poke) => poke,
        };

        if !(0..16).contains(&attack_iv)  {
            let message = format!("{}: 攻撃の個体値(attack_iv)が範囲外(0～15が正常): {}", poke.name, attack_iv);
            return Err(PokemonError { message });
        }

        if !(0..16).contains(&defense_iv)  {
            let message = format!("{}: 防御の個体値(defense_iv)が範囲外(0～15が正常): {}", poke.name, defense_iv);
            return Err(PokemonError { message });
        }

        if !(0..16).contains(&stamina_iv)  {
            let message = format!("{}: 耐久の個体値(stamina_iv)が範囲外(0～15が正常): {}", poke.name, stamina_iv);
            return Err(PokemonError { message });
        }

        let lv;

        if let Some(pl) = pokemon_lv {  // pokemon_lv引数があればそれをポケモンレベルとする。
            let int_pl = (pl * 2.0).floor() as usize;
            if !(2..=100).contains(&int_pl) {
                let message = format!("{}: pokemon_lvが1.0から50.0の間でない。: {}", poke.name, pl);
                return Err(PokemonError { message });
            }

            lv = pl;
            cp = calc_cp(poke, lv, attack_iv, defense_iv, stamina_iv);
        } else {
            if cp < 0 {
                cp = -cp;

                lv = match calc_pl_limited_by_cp(cp, 50.0, poke, attack_iv, defense_iv, stamina_iv) {
                    Some(lv) => lv,
                    None => {
                        let message = format!("{}: CP {} 以下は存在しない。", poke.name, cp);
                        return Err(PokemonError { message });
                    }
                }
            } else {
                lv = match calc_pokemon_lv(poke, cp, attack_iv, defense_iv, stamina_iv) {
                    None => {
                        let mut msgs = vec![];
                        msgs.push(format!("{}: ポケモンレベルの取得に失敗(CPか個体値が間違っている)", poke.name));

                        let near_ivs = search_near_iv(poke, cp, attack_iv, defense_iv, stamina_iv);
                        if !near_ivs.is_empty() {
                            msgs.push(format!("もしかして、この値?"));

                            for (a, d, s) in near_ivs {
                                msgs.push(format!("attack_iv = {}, defense_iv = {}, stamina_iv = {}", a, d, s));
                            }
                        }

                        return Err(PokemonError { message: msgs.join("\n") });
                    },
                    Some(lv) => lv,
                };
            }
        }

        let scp = calc_scp(poke, lv, attack_iv, defense_iv, stamina_iv);
        let dcp = calc_dcp(poke, lv, attack_iv, defense_iv, stamina_iv);

        let types = poke.get_types();

        let fast_move = match get_fast_move_by_name(fast_move) {
            None => {
                let message = format!("{}: 存在しないノーマルアタック(fast_move): {}", poke.name, fast_move);
                return Err(PokemonError { message });
            },
            Some(mv) => mv,
        };

        let is_stab_fast_move = types.iter().any(|t| t == &fast_move.mtype);

        let charge_move1 = match get_charge_move_by_name(charge_move1) {
            None => {
                let message = format!("{}: 存在しないスペシャルアタック(charge_move1): {}", poke.name, charge_move1);
                return Err(PokemonError { message });
            },
            Some(mv) => mv,
        };

        let is_stab_charge_move1 = types.iter().any(|t| t == &charge_move1.mtype);

        let charge_move2 = match charge_move2 {
            None => None,
            Some(mv_str) => {
                match get_charge_move_by_name(&mv_str) {
                    None => {
                        let message = format!("{}: 存在しないスペシャルアタック(charge_move2): {}", poke.name, mv_str);
                        return Err(PokemonError { message });
                    },
                    Some(mv) => Some(mv),
                }
            }
        };

        let is_stab_charge_move2 = if let Some(mv) = charge_move2 {
            types.iter().any(|t| t == &mv.mtype)
        } else {
            false
        };

        let cpm = get_cpm(lv);
        let attack = (poke.attack_st + attack_iv) as f64 * cpm;
        let defense = (poke.defense_st + defense_iv) as f64 * cpm;
        let stamina = (poke.stamina_st + stamina_iv) as f64 * cpm;
        let hp = stamina as i32;

        Ok(Pokemon { poke, lv, cp, scp, dcp, attack_iv, defense_iv, stamina_iv, cpm, attack, defense, stamina, hp, fast_move,
            is_stab_fast_move, charge_move1, is_stab_charge_move1, charge_move2, is_stab_charge_move2 })
    }

    pub fn new_limited_by_cp(limit_cp: i32, name: &str, fast_move: &str, charge_move1: &str, charge_move2: Option<String>,
                             attack_iv: i32, defense_iv: i32, stamina_iv: i32) -> Result<Self, PokemonError> {
        Pokemon::new(name, fast_move, charge_move1, charge_move2, -limit_cp, None, attack_iv, defense_iv, stamina_iv)
    }
}

#[derive(Debug, Deserialize)]
pub struct PokemonJson {
    pub name: String,
    pub cp: i32,

    // 個体値(0～15)
    pub attack_iv: i32,
    pub defense_iv: i32,
    pub stamina_iv: i32,

    // 技
    pub fast_move: String,
    pub charge_move1: String,
    pub charge_move2: Option<String>,
}

fn search_near_iv(poke: &Pokepedia, cp: i32, attack_iv: i32, defense_iv: i32, stamina_iv: i32) -> Vec<(i32, i32, i32)> {
    let mut near_ivs = vec![];

    let near_attack_iv = vec![attack_iv-1, attack_iv, attack_iv+1].into_iter().filter(|v| 0 <= *v && *v <= 15).collect::<Vec<_>>();
    let near_defense_iv = vec![defense_iv-1, defense_iv, defense_iv+1].into_iter().filter(|v| 0 <= *v && *v <= 15).collect::<Vec<_>>();
    let near_stamina_iv = vec![stamina_iv-1, stamina_iv, stamina_iv+1].into_iter().filter(|v| 0 <= *v && *v <= 15).collect::<Vec<_>>();

    for a in &near_attack_iv {
        for d in &near_defense_iv {
            for s in &near_stamina_iv {
                if calc_pokemon_lv(poke, cp, *a, *d, *s).is_some() {
                    near_ivs.push((*a, *d, *s));
                }
            }
        }
    }

    near_ivs
}

pub fn load_pokemon<R: Read>(reader: &mut R) -> Result<Vec<Pokemon>, std::io::Error> {
    let mut pokemons = vec![];

    let data: Vec<PokemonJson> = serde_json::from_reader(reader)?;

    for d in data {
        let poke = Pokemon::new(&d.name, &d.fast_move, &d.charge_move1, d.charge_move2,
                                d.cp, None, d.attack_iv, d.defense_iv, d.stamina_iv);

        match poke {
            Ok(poke) => pokemons.push(poke),
            Err(err) => println!("{}", err),
        }
    }

    Ok(pokemons)
}

#[test]
fn test_load_pokemon() {
    let poke_json = r#"
[
    {
        "name": "ココロモリ",
        "cp": 1489,
        "attack_iv": 10,
        "defense_iv": 9,
        "stamina_iv": 12,
        "fast_move": "エアスラッシュ",
        "charge_move1": "サイコファング",
        "charge_move2": null
    },
    {
        "name": "キレイハナ",
        "cp": 1479,
        "attack_iv": 2,
        "defense_iv": 15,
        "stamina_iv": 6,
        "fast_move": "マジカルリーフ",
        "charge_move1": "リーフブレード",
        "charge_move2": null
    },
    {
        "name": "ナマズン",
        "cp": 1474,
        "attack_iv": 8,
        "defense_iv": 15,
        "stamina_iv": 14,
        "fast_move": "みずでっぽう",
        "charge_move1": "どろばくだん",
        "charge_move2": null
    }
]
    "#;

    use std::io::Cursor;

    let mut reader = Cursor::new(poke_json);
    let pokemons = match load_pokemon(&mut reader) {
        Err(err) => panic!("{}", err),
        Ok(v) => v,
    };

    assert_eq!(pokemons.len(), 3);

    let p = &pokemons[0];

    assert_eq!(p.poke.name, "ココロモリ");
    assert_eq!(p.cp, 1489);
    assert_eq!(p.lv, 34.5);
    assert_eq!(p.attack_iv, 10);
    assert_eq!(p.defense_iv, 9);
    assert_eq!(p.stamina_iv, 12);
    assert_eq!(p.cpm, 0.7586303702);
    assert_eq!(p.attack, 129.7257933042);
    assert_eq!(p.defense, 97.1046873856);
    assert_eq!(p.stamina, 135.7948362658);
    assert_eq!(p.hp, 135);
    assert_eq!(p.fast_move.name, "エアスラッシュ");
    assert_eq!(p.charge_move1.name, "サイコファング");
    assert_eq!(p.charge_move2.is_none(), true);
}
