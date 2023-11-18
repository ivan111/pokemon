//! CP, SCP, DCPなどのバトルでの強さの参考になる指標を計算する

use super::base_stats::*;
use super::pokemon_lv::get_pokemon_lv;

/// CP(Combat Power, 戦闘力)を計算して返す。
/// CPは攻撃力を重視した指標となる。
/// ジムやレイドの攻撃側で参考となる指標。
pub fn calc_cp(bs: &BaseStats, pokemon_lv: f32, iv_attack: i32, iv_defense: i32, iv_stamina: i32) -> i32 {
    assert!(0 <= iv_attack && iv_attack <= 15);
    assert!(0 <= iv_defense && iv_defense <= 15);
    assert!(0 <= iv_stamina && iv_stamina <= 15);

    let pl = get_pokemon_lv(pokemon_lv);

    let attack = (bs.attack + iv_attack) as f64 * pl.cpm;
    let defense = (bs.defense + iv_defense) as f64 * pl.cpm;
    let stamina = (bs.stamina + iv_stamina) as f64 * pl.cpm;

    let cp = (attack * defense.sqrt() * stamina.sqrt() / 10.0) as i32;

    if cp < 10 {
        10
    } else {
        cp
    }
}

#[test]
fn test_calc_cp() {
    let bs_name_map = get_base_stats_map_by_name();

    let kure = bs_name_map.get("クレセリア").unwrap();
    assert_eq!(calc_cp(kure, 20.0, 2, 15, 13), 1500);

    let fude = bs_name_map.get("フーディン").unwrap();
    assert_eq!(calc_cp(fude, 18.0, 1, 15, 15), 1495);
}

/// SCP(Standard Combat Power, 標準戦闘力)を計算して返す。
/// SCPは独自の指標でゲームでは表示されることはない。
/// SCPは攻撃力・防御力・耐久性をバランスよく表した指標となる。
/// トレーナーバトルなど1対1の対戦で参考となる指標。
pub fn calc_scp(bs: &BaseStats, pokemon_lv: f32, iv_attack: i32, iv_defense: i32, iv_stamina: i32) -> i32 {
    assert!(0 <= iv_attack && iv_attack <= 15);
    assert!(0 <= iv_defense && iv_defense <= 15);
    assert!(0 <= iv_stamina && iv_stamina <= 15);

    let pl = get_pokemon_lv(pokemon_lv);

    let attack = (bs.attack + iv_attack) as f64 * pl.cpm;
    let defense = (bs.defense + iv_defense) as f64 * pl.cpm;
    let stamina = (bs.stamina + iv_stamina) as f64 * pl.cpm;

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
    let bs_name_map = get_base_stats_map_by_name();

    let kure = bs_name_map.get("クレセリア").unwrap();
    assert_eq!(calc_scp(kure, 20.0, 2, 15, 13), 1815);

    let fude = bs_name_map.get("フーディン").unwrap();
    assert_eq!(calc_scp(fude, 18.0, 1, 15, 15), 1281);
}

/// 耐久性のステータスを端数処理して計算したSCP。
pub fn calc_scp2(bs: &BaseStats, pokemon_lv: f32, iv_attack: i32, iv_defense: i32, iv_stamina: i32) -> i32 {
    assert!(0 <= iv_attack && iv_attack <= 15);
    assert!(0 <= iv_defense && iv_defense <= 15);
    assert!(0 <= iv_stamina && iv_stamina <= 15);

    let pl = get_pokemon_lv(pokemon_lv);

    let attack = (bs.attack + iv_attack) as f64 * pl.cpm;
    let defense = (bs.defense + iv_defense) as f64 * pl.cpm;
    let stamina = (bs.stamina + iv_stamina) as f64 * pl.cpm;

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
    let bs_name_map = get_base_stats_map_by_name();

    let kure = bs_name_map.get("クレセリア").unwrap();
    assert_eq!(calc_scp2(kure, 20.0, 2, 15, 13), 1815);

    let fude = bs_name_map.get("フーディン").unwrap();
    assert_eq!(calc_scp2(fude, 18.0, 1, 15, 15), 1279);
}

/// DCP(Defensive Combat Power, 防御的戦闘力)を計算して返す。
/// DCPは独自の指標でゲームでは表示されることはない。
/// DCPは防御力と耐久性を重視した指標となる。
/// ジムやレイドの防衛側で参考となる指標。
pub fn calc_dcp(bs: &BaseStats, pokemon_lv: f32, iv_attack: i32, iv_defense: i32, iv_stamina: i32) -> i32 {
    assert!(0 <= iv_attack && iv_attack <= 15);
    assert!(0 <= iv_defense && iv_defense <= 15);
    assert!(0 <= iv_stamina && iv_stamina <= 15);

    let pl = get_pokemon_lv(pokemon_lv);

    let attack = (bs.attack + iv_attack) as f64 * pl.cpm;
    let defense = (bs.defense + iv_defense) as f64 * pl.cpm;
    let stamina = (bs.stamina + iv_stamina) as f64 * pl.cpm;

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
    let bs_name_map = get_base_stats_map_by_name();

    let hapi = bs_name_map.get("ハピナス").unwrap();
    assert_eq!(calc_dcp(hapi, 40.0, 15, 15, 15), 4340);
}
