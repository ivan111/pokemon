//! CP, SCP, DCPなどのバトルでの強さの参考になる指標を計算する

use crate::pokepedia::*;
use crate::pokemon::{Pokemon, IVs};

const MAX_ACP_TURNS: i32 = 1000;

/// 1ターンあたりの平均的なわざの威力を計算する
fn calc_power_per_turn(poke: &Pokemon) -> f64 {
    let mut num_turns = 0;
    let mut sum_power = 0.0;
    let mut energy = 0;

    let types = poke.types();

    while num_turns < MAX_ACP_TURNS {
        if energy >= poke.charge_move1().energy() {
            energy = std::cmp::max(0, energy - poke.charge_move1().energy());
            sum_power += poke.charge_move1().real_power(&types);
            num_turns += 1;
        } else {
            energy = std::cmp::min(energy + poke.fast_move().energy(), 100);
            sum_power += poke.fast_move().real_power(&types);
            num_turns += poke.fast_move().turns();
        }
    }

    sum_power / num_turns as f64
}

/// ACP(Advanced Combat Power, 発展型戦闘力)を計算して返す。
/// ACPは自分が指標でゲームでは表示されることはない。
/// ACPは攻撃力・防御力・耐久性に加えて、技の威力も考慮に入れる。
pub fn calc_acp(dict: &Pokepedia, lv: f32, ivs: IVs,
                fast_move: &str, charge_move1: &str, charge_move2: Option<String>) -> i32 {

    let stats = dict.base_stats().stats(lv, ivs);
    let scp = stats.calc_scp();

    let p = Pokemon::new(dict.name(), fast_move, charge_move1, charge_move2, 0, Some(lv),
                         (ivs.attack, ivs.defense, ivs.stamina)).unwrap();

    let ppt = calc_power_per_turn(&p);
    println!("ppt = {:.2}", ppt);

    (scp as f64 * ppt / 10.0).floor() as i32
}

/*

#[test]
fn test_calc_acp() {
    let kure = pokepedia_by_name("クレセリア").unwrap();
    let ivs = IVs::new(2, 15, 13).unwrap();
    assert_eq!(calc_acp(kure, 20.0, ivs, "ねんりき", "みらいよち", None), 1982);

    let fude = pokepedia_by_name("フーディン").unwrap();
    let ivs = IVs::new(1, 15, 15).unwrap();
    assert_eq!(calc_acp(fude, 18.0, ivs, "ねんりき", "みらいよち", None), 1399);
}

*/

/// 引数のlimit_cp以下のCPという条件で、一番高いポケモンレベルを返す。
/// ポケモンレベル1.0でもlimit_cpを超える場合は、Noneを返す。
pub fn calc_lv_limited_by_cp(limit_cp: i32, limit_lv: f32, dict: &Pokepedia, ivs: IVs) -> Option<f32> {
    let to = (limit_lv * 2.0) as usize;

    for i in 2..=to {
        let lv = i as f32 / 2.0;
        let stats = dict.base_stats().stats(lv, ivs);
        let cp = stats.calc_cp();

        if cp > limit_cp {
            if lv == 1.0 {
                return None;
            }

            return Some(lv - 0.5);
        }
    }

    Some(limit_lv)
}

#[test]
fn test_calc_lv_limited_by_cp() {
    let kure = pokepedia_by_name("クレセリア").unwrap();
    let ivs = IVs::new(2, 15, 13).unwrap();
    assert_eq!(calc_lv_limited_by_cp(1500, 50.0, kure, ivs), Some(20.0));
    assert_eq!(calc_lv_limited_by_cp(5000, 50.0, kure, ivs), Some(50.0));

    let hapi = pokepedia_by_name("ハピナス").unwrap();
    let ivs = IVs::new(15, 15, 15).unwrap();
    assert_eq!(calc_lv_limited_by_cp(39, 40.0, hapi, ivs), Some(1.0));
    assert_eq!(calc_lv_limited_by_cp(38, 40.0, hapi, ivs), None);
}

/// 一番SCPが高くなる個体値の組み合わせを計算する。
/// 戻り値はOption<(SCP, ポケモンレベル, 攻撃個体値, 防御個体値, 耐久個体値)>
pub fn calc_max_scp_iv_limited_by_cp(limit_cp: i32, limit_lv: f32, dict: &Pokepedia) -> Option<(i32, f32, IVs)> {
    let mut max_scp = 0;
    let mut max_scp_ivs = None;

    for ivs in (0..(16*16*16)).map(i2ivs) {

        let lv = calc_lv_limited_by_cp(limit_cp, limit_lv, dict, ivs);

        if let Some(lv) = lv {
            let stats = dict.base_stats().stats(lv, ivs);
            let scp = stats.calc_scp();

            if scp > max_scp {
                max_scp = scp;
                max_scp_ivs = Some((scp, lv, ivs));
            }
        }
    }

    max_scp_ivs
}

#[test]
fn test_calc_max_scp_iv_limited_by_cp() {
    let koko = pokepedia_by_name("ココロモリ").unwrap();
    let ivs = IVs::new(0, 15, 9).unwrap();
    assert_eq!(calc_max_scp_iv_limited_by_cp(1500, 40.0, koko), Some((1476, 38.0, ivs)));
}

/// 重複順列を作るための変換。
/// (0..(16*16*16)).map(i2ivs) で全組み合わせを生成できる
fn i2ivs(i: usize) -> IVs {
    let attack = ((i & 0xF00) >> 8) as i32;
    let defense = ((i & 0xF0) >> 4) as i32;
    let stamina = (i & 0xF) as i32;

    IVs::new(attack, defense, stamina).unwrap()
}
