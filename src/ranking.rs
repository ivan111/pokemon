//! ポケモンのランキングを作る

use crate::pokepedia::POKEPEDIA;
use crate::pokemon::Pokemon;
use crate::index::calc_top_scp_iv_limited_by_cp;

/*
pub fn get_high_ecp(dict: &'static Pokepedia, limit_cp: i32, limit_lv: f32)
    -> (&'static FastMove, &'static ChargeMove, Option<&'static ChargeMove>) {

    if let Some((_, lv, ivs)) = calc_top_scp_iv_limited_by_cp(limit_cp, limit_lv, dict) {
        for fm in dict.fast_moves() {
            for cm1 in dict.charge_moves() {
            }
        }
        let poke = Pokemon::raw_new(p, lv, ivs, p.fast_moves()[0], p.charge_moves()[0], None);
        v.push(poke);
    }

    v
}

pub fn get_top_pokemons(limit_cp: i32, limit_lv: f32) -> Vec<Pokemon> {
    let mut v = vec![];

    for p in &POKEPEDIA {
        if let Some((_, lv, ivs)) = calc_top_scp_iv_limited_by_cp(limit_cp, limit_lv, p) {
            let poke = Pokemon::raw_new(p, lv, ivs, p.fast_moves()[0], p.charge_moves()[0], None);
            v.push(poke);
        }
    }

    v
}
*/

pub fn scp_ranking(limit_cp: i32, limit_lv: f32) -> Vec<Pokemon> {
    let mut v = vec![];

    for p in &POKEPEDIA {
        if let Some((_, lv, ivs)) = calc_top_scp_iv_limited_by_cp(limit_cp, limit_lv, p) {
            let poke = Pokemon::raw_new(p, lv, ivs, p.fast_moves()[0], p.charge_moves()[0], None);
            v.push(poke);
        }
    }

     v.sort_by_key(|p| std::cmp::Reverse(p.scp()));

    v
}
