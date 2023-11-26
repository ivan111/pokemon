//! ポケモンのランキングを作る

use crate::pokepedia::{Pokepedia, POKEPEDIA};
use crate::pokemon::Pokemon;
use crate::index::calc_max_scp_iv_limited_by_cp;

pub fn scp_ranking(limit_cp: i32, limit_lv: f32) -> Vec<Pokemon> {
    let mut v = vec![];

    for p in &POKEPEDIA {
        if let Some((_, lv, ivs)) = calc_max_scp_iv_limited_by_cp(limit_cp, limit_lv, p) {
            let poke = Pokemon::new(p.name(), p.fast_moves()[0].name(), p.charge_moves()[0].name(), None,
                                    0, Some(lv), ivs.to_tuple()).unwrap();
            v.push(poke);
        }
    }

     v.sort_by(|a, b| b.scp().cmp(&a.scp()));

    v
}
