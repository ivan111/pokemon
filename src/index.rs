//! CP, SCP, DCPなどのバトルでの強さの参考になる指標を計算する

use crate::pokepedia::*;
use crate::pokemon::Pokemon;
use crate::cpm::get_cpm;

/// CP(Combat Power, 戦闘力)を計算して返す。
pub fn calc_cp(poke: &Pokepedia, pokemon_lv: f32, attack_iv: i32, defense_iv: i32, stamina_iv: i32) -> i32 {
    assert!((0..=15).contains(&attack_iv));
    assert!((0..=15).contains(&defense_iv));
    assert!((0..=15).contains(&stamina_iv));

    let cpm = get_cpm(pokemon_lv);

    let attack = (poke.attack_st + attack_iv) as f64 * cpm;
    let defense = (poke.defense_st + defense_iv) as f64 * cpm;
    let stamina = (poke.stamina_st + stamina_iv) as f64 * cpm;

    let cp = (attack * (defense * stamina).sqrt() / 10.0) as i32;

    if cp < 10 {
        10
    } else {
        cp
    }
}

#[test]
fn test_calc_cp() {
    let kure = get_pokepedia_by_name("クレセリア").unwrap();
    assert_eq!(calc_cp(kure, 20.0, 2, 15, 13), 1500);

    let fude = get_pokepedia_by_name("フーディン").unwrap();
    assert_eq!(calc_cp(fude, 18.0, 1, 15, 15), 1495);
}

const MAX_ACP_TURNS: i32 = 1000;

/// 1ターンあたりの平均的なわざの威力を計算する
fn calc_power_per_turn(poke: &Pokemon) -> f64 {
    let mut num_turns = 0;
    let mut sum_power = 0.0;
    let mut energy = 0;

    let types = poke.poke.get_types();

    while num_turns < MAX_ACP_TURNS {
        if energy >= poke.charge_move1.tb_energy {
            energy = std::cmp::max(0, energy - poke.charge_move1.tb_energy);
            sum_power += poke.charge_move1.get_real_tb_power(&types);
            num_turns += 1;
        } else {
            energy = std::cmp::min(energy + poke.fast_move.tb_energy, 100);
            sum_power += poke.fast_move.get_real_tb_power(&types);
            num_turns += poke.fast_move.tb_turns;
        }
    }

    sum_power / num_turns as f64
}

/// ACP(Advanced Combat Power, 発展型戦闘力)を計算して返す。
/// ACPは自分が指標でゲームでは表示されることはない。
/// ACPは攻撃力・防御力・耐久性に加えて、技の威力も考慮に入れる。
#[allow(dead_code)]
pub fn calc_acp(poke: &Pokepedia, pokemon_lv: f32, attack_iv: i32, defense_iv: i32, stamina_iv: i32,
                fast_move: &str, charge_move1: &str, charge_move2: Option<String>) -> i32 {

    let scp = calc_scp(poke, pokemon_lv, attack_iv, defense_iv, stamina_iv);

    let p = Pokemon::new(poke.name, fast_move, charge_move1, charge_move2, 0, Some(pokemon_lv),
                         attack_iv, defense_iv, stamina_iv).unwrap();

    let ppt = calc_power_per_turn(&p);
    println!("ppt = {:.2}", ppt);

    (scp as f64 * ppt / 10.0).floor() as i32
}

#[test]
fn test_calc_acp() {
    let kure = get_pokepedia_by_name("クレセリア").unwrap();
    assert_eq!(calc_acp(kure, 20.0, 2, 15, 13, "ねんりき", "みらいよち", None), 1982);

    let fude = get_pokepedia_by_name("フーディン").unwrap();
    assert_eq!(calc_acp(fude, 18.0, 1, 15, 15, "ねんりき", "みらいよち", None), 1399);
}

/// SCP(Standard Combat Power, 標準戦闘力)を計算して返す。
/// SCPは独自の指標でゲームでは表示されることはない。
/// SCPは攻撃力・防御力・耐久性をバランスよく表した指標。
/// トレーナーバトルなど1対1の対戦で参考となる。
#[allow(dead_code)]
pub fn calc_scp(poke: &Pokepedia, pokemon_lv: f32, attack_iv: i32, defense_iv: i32, stamina_iv: i32) -> i32 {
    assert!((0..=15).contains(&attack_iv));
    assert!((0..=15).contains(&defense_iv));
    assert!((0..=15).contains(&stamina_iv));

    let cpm = get_cpm(pokemon_lv);

    let attack = (poke.attack_st + attack_iv) as f64 * cpm;
    let defense = (poke.defense_st + defense_iv) as f64 * cpm;
    let stamina = (poke.stamina_st + stamina_iv) as f64 * cpm;

    let v = attack * defense * stamina;
    let scp = (v.powf(2.0/3.0) / 10.0) as i32;

    if scp < 10 {
        10
    } else {
        scp
    }
}

#[test]
fn test_calc_scp() {
    let kure = get_pokepedia_by_name("クレセリア").unwrap();
    assert_eq!(calc_scp(kure, 20.0, 2, 15, 13), 1815);

    let fude = get_pokepedia_by_name("フーディン").unwrap();
    assert_eq!(calc_scp(fude, 18.0, 1, 15, 15), 1281);
}

/// 耐久性のステータスを端数処理して計算したSCP。
#[allow(dead_code)]
pub fn calc_scp2(poke: &Pokepedia, pokemon_lv: f32, attack_iv: i32, defense_iv: i32, stamina_iv: i32) -> i32 {
    assert!((0..=15).contains(&attack_iv));
    assert!((0..=15).contains(&defense_iv));
    assert!((0..=15).contains(&stamina_iv));

    let cpm = get_cpm(pokemon_lv);

    let attack = (poke.attack_st + attack_iv) as f64 * cpm;
    let defense = (poke.defense_st + defense_iv) as f64 * cpm;
    let stamina = (poke.stamina_st + stamina_iv) as f64 * cpm;

    let v = attack * defense * stamina.floor();  // calc_scpとの違いはここのfloorだけ
    let scp = (v.powf(2.0/3.0) / 10.0) as i32;

    if scp < 10 {
        10
    } else {
        scp
    }
}

#[test]
fn test_calc_scp2() {
    let kure = get_pokepedia_by_name("クレセリア").unwrap();
    assert_eq!(calc_scp2(kure, 20.0, 2, 15, 13), 1815);

    let fude = get_pokepedia_by_name("フーディン").unwrap();
    assert_eq!(calc_scp2(fude, 18.0, 1, 15, 15), 1279);
}

/// DCP(Defensive Combat Power, 防御的戦闘力)を計算して返す。
/// DCPは独自の指標でゲームでは表示されることはない。
/// DCPは防御力と耐久性を重視した指標となる。
#[allow(dead_code)]
pub fn calc_dcp(poke: &Pokepedia, pokemon_lv: f32, attack_iv: i32, defense_iv: i32, stamina_iv: i32) -> i32 {
    assert!((0..=15).contains(&attack_iv));
    assert!((0..=15).contains(&defense_iv));
    assert!((0..=15).contains(&stamina_iv));

    let cpm = get_cpm(pokemon_lv);

    let attack = (poke.attack_st + attack_iv) as f64 * cpm;
    let defense = (poke.defense_st + defense_iv) as f64 * cpm;
    let stamina = (poke.stamina_st + stamina_iv) as f64 * cpm;

    let v = attack * defense * defense * stamina * stamina;
    let dcp = (v.powf(2.0/5.0) / 10.0) as i32;

    if dcp < 10 {
        10
    } else {
        dcp
    }
}

#[test]
fn test_calc_dcp() {
    let hapi = get_pokepedia_by_name("ハピナス").unwrap();
    assert_eq!(calc_dcp(hapi, 40.0, 15, 15, 15), 4340);
}

/// 引数のlimit_cp以下のCPという条件で、一番PLの高いポケモンレベルを返す。
/// ポケモンレベル1.0でもlimit_cpを超える場合は、Noneを返す。
#[allow(dead_code)]
pub fn calc_pl_limited_by_cp(limit_cp: i32, limit_pl: f32, poke: &Pokepedia, attack_iv: i32, defense_iv: i32, stamina_iv: i32) -> Option<f32> {
    let to = (limit_pl * 2.0) as usize;

    for i in 2..=to {
        let pl = i as f32 / 2.0;
        let cp = calc_cp(poke, pl, attack_iv, defense_iv, stamina_iv);

        if cp > limit_cp {
            if pl == 1.0 {
                return None;
            }

            return Some(pl - 0.5);
        }
    }

    Some(limit_pl)
}

#[test]
fn test_calc_pl_limited_by_cp() {
    let kure = get_pokepedia_by_name("クレセリア").unwrap();
    assert_eq!(calc_cp(kure, 20.0, 2, 15, 13), 1500);
    assert_eq!(calc_pl_limited_by_cp(1500, 50.0, kure, 2, 15, 13), Some(20.0));
    assert_eq!(calc_pl_limited_by_cp(5000, 50.0, kure, 2, 15, 13), Some(50.0));

    let hapi = get_pokepedia_by_name("ハピナス").unwrap();
    assert_eq!(calc_cp(hapi, 1.0, 15, 15, 15), 39);
    assert_eq!(calc_pl_limited_by_cp(39, 40.0, hapi, 15, 15, 15), Some(1.0));
    assert_eq!(calc_pl_limited_by_cp(38, 40.0, hapi, 15, 15, 15), None);
}

/// 一番SCPが高くなる個体値の組み合わせを計算する。
/// 戻り値はOption<(SCP, ポケモンレベル, 攻撃個体値, 防御個体値, 耐久個体値)>
#[allow(dead_code)]
pub fn calc_max_scp_iv_limited_by_cp(limit_cp: i32, limit_pl: f32, poke: &Pokepedia) -> Option<(i32, f32, i32, i32, i32)> {
    let mut max_scp = 0;
    let mut max_scp_iv = None;

    for (attack_iv, defense_iv, stamina_iv) in (0..(16*16*16)).map(i2ivs) {

        let pl = calc_pl_limited_by_cp(limit_cp, limit_pl, poke, attack_iv, defense_iv, stamina_iv);

        if let Some(pl) = pl {
            let scp = calc_scp2(poke, pl, attack_iv, defense_iv, stamina_iv);

            if scp > max_scp {
                max_scp = scp;
                max_scp_iv = Some((scp, pl, attack_iv, defense_iv, stamina_iv));
            }
        }
    }

    max_scp_iv
}

#[test]
fn test_calc_max_scp_iv_limited_by_cp() {
    let koko = get_pokepedia_by_name("ココロモリ").unwrap();
    assert_eq!(calc_max_scp_iv_limited_by_cp(1500, 40.0, koko), Some((1476, 38.0, 0, 15, 9)));
}

/// 重複順列を作るための変換。
/// (0..(16*16*16)).map(i2ivs) で全組み合わせを生成できる
#[allow(dead_code)]
fn i2ivs(i: usize) -> (i32, i32, i32) {
    let attack_iv = ((i & 0xF00) >> 8) as i32;
    let defense_iv = ((i & 0xF0) >> 4) as i32;
    let stamina_iv = (i & 0xF) as i32;

    (attack_iv, defense_iv, stamina_iv)
}
