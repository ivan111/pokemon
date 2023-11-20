//! ポケモンのデータを保持する。

use std::io::Read;

use serde::Deserialize;

use crate::pokepedia::*;
use crate::moves::*;
use crate::cpm::*;

#[derive(Debug)]
pub struct Pokemon {
    pub poke: &'static Pokepedia,

    pub cp: i32,
    pub lv: f32,

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
    pub charge_move1: &'static ChargeMove,
    pub charge_move2: Option<&'static ChargeMove>,
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
    let pokepedia_map = get_pokepedia_by_name();
    let fast_move_map = get_fast_move_map_by_name();
    let charge_move_map = get_charge_move_map_by_name();

    let mut pokemons = vec![];

    let data: Vec<PokemonJson> = serde_json::from_reader(reader)?;

    for d in data {
        let poke = match pokepedia_map.get(&d.name) {
            None => {
                eprintln!("存在しないポケモン: {}", d.name);
                continue;
            },
            Some(poke) => poke,
        };

        if d.attack_iv < 0 || d.attack_iv > 15 {
            eprintln!("{}: 攻撃の個体値(attack_iv)が範囲外(0～15が正常): {}", poke.name, d.attack_iv);
            continue;
        }

        if d.defense_iv < 0 || d.defense_iv > 15 {
            eprintln!("{}: 防御の個体値(defense_iv)が範囲外(0～15が正常): {}", poke.name, d.defense_iv);
            continue;
        }

        if d.stamina_iv < 0 || d.stamina_iv > 15 {
            eprintln!("{}: 耐久の個体値(stamina_iv)が範囲外(0～15が正常): {}", poke.name, d.stamina_iv);
            continue;
        }

        let lv = match calc_pokemon_lv(poke, d.cp, d.attack_iv, d.defense_iv, d.stamina_iv) {
            None => {
                eprintln!("{}: ポケモンレベルの取得に失敗(CPか個体値が間違っている)", poke.name);

                let near_ivs = search_near_iv(poke, d.cp, d.attack_iv, d.defense_iv, d.stamina_iv);
                if !near_ivs.is_empty() {
                    eprintln!("もしかして、この値?");

                    for (a, d, s) in near_ivs {
                        eprintln!("attack_iv = {}, defense_iv = {}, stamina_iv = {}", a, d, s);
                    }
                }

                continue;
            },
            Some(lv) => lv,
        };

        let fast_move = match fast_move_map.get(&d.fast_move) {
            None => {
                eprintln!("{}: 存在しないノーマルアタック(fast_move): {}", poke.name, d.fast_move);
                continue;
            },
            Some(mv) => mv,
        };

        let charge_move1 = match charge_move_map.get(&d.charge_move1) {
            None => {
                eprintln!("{}: 存在しないスペシャルアタック(charge_move1): {}", poke.name, d.charge_move1);
                continue;
            },
            Some(mv) => mv,
        };

        let charge_move2 = match d.charge_move2 {
            None => None,
            Some(mv_str) => {
                match charge_move_map.get(&mv_str) {
                    None => {
                        eprintln!("{}: 存在しないスペシャルアタック(charge_move2): {}", poke.name, mv_str);
                        continue;
                    },
                    Some(mv) => Some(*mv),
                }
            }
        };

        let cpm = get_cpm(lv);
        let attack = (poke.attack_st + d.attack_iv) as f64 * cpm;
        let defense = (poke.defense_st + d.defense_iv) as f64 * cpm;
        let stamina = (poke.stamina_st + d.stamina_iv) as f64 * cpm;

        let p = Pokemon {
            poke,

            cp: d.cp,
            lv,

            attack_iv: d.attack_iv,
            defense_iv: d.defense_iv,
            stamina_iv: d.stamina_iv,

            cpm,
            attack,
            defense,
            stamina,
            hp: stamina as i32,

            fast_move,
            charge_move1,
            charge_move2
        };

        pokemons.push(p);
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
