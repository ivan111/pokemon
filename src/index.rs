//! CP, SCP, DCPなどのバトルでの強さの参考になる指標を計算する

use super::pokepedia::*;
use super::cpm::get_cpm;

/// CP(Combat Power, 戦闘力)を計算して返す。
pub fn calc_cp(poke: &Pokepedia, pokemon_lv: f32, attack_iv: i32, defense_iv: i32, stamina_iv: i32) -> i32 {
    assert!(0 <= attack_iv && attack_iv <= 15);
    assert!(0 <= defense_iv && defense_iv <= 15);
    assert!(0 <= stamina_iv && stamina_iv <= 15);

    let cpm = get_cpm(pokemon_lv);

    let attack = (poke.attack_st + attack_iv) as f64 * cpm;
    let defense = (poke.defense_st + defense_iv) as f64 * cpm;
    let stamina = (poke.stamina_st + stamina_iv) as f64 * cpm;

    let cp = (attack * defense.sqrt() * stamina.sqrt() / 10.0) as i32;

    if cp < 10 {
        10
    } else {
        cp
    }
}

#[test]
fn test_calc_cp() {
    let m = get_pokepedia_by_name();

    let kure = m.get("クレセリア").unwrap();
    assert_eq!(calc_cp(kure, 20.0, 2, 15, 13), 1500);

    let fude = m.get("フーディン").unwrap();
    assert_eq!(calc_cp(fude, 18.0, 1, 15, 15), 1495);
}

/// SCP(Standard Combat Power, 標準戦闘力)を計算して返す。
/// SCPは独自の指標でゲームでは表示されることはない。
/// SCPは攻撃力・防御力・耐久性をバランスよく表した指標とされているが、
/// 防御力と耐久性を合わせて守る力と考えれば、CPのほうが正しいことになる。
/// トレーナーバトルなど1対1の対戦で参考となる指標とされているが疑問。
#[allow(dead_code)]
pub fn calc_scp(poke: &Pokepedia, pokemon_lv: f32, attack_iv: i32, defense_iv: i32, stamina_iv: i32) -> i32 {
    assert!(0 <= attack_iv && attack_iv <= 15);
    assert!(0 <= defense_iv && defense_iv <= 15);
    assert!(0 <= stamina_iv && stamina_iv <= 15);

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
    let m = get_pokepedia_by_name();

    let kure = m.get("クレセリア").unwrap();
    assert_eq!(calc_scp(kure, 20.0, 2, 15, 13), 1815);

    let fude = m.get("フーディン").unwrap();
    assert_eq!(calc_scp(fude, 18.0, 1, 15, 15), 1281);
}

/// 耐久性のステータスを端数処理して計算したSCP。
#[allow(dead_code)]
pub fn calc_scp2(poke: &Pokepedia, pokemon_lv: f32, attack_iv: i32, defense_iv: i32, stamina_iv: i32) -> i32 {
    assert!(0 <= attack_iv && attack_iv <= 15);
    assert!(0 <= defense_iv && defense_iv <= 15);
    assert!(0 <= stamina_iv && stamina_iv <= 15);

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
    let m = get_pokepedia_by_name();

    let kure = m.get("クレセリア").unwrap();
    assert_eq!(calc_scp2(kure, 20.0, 2, 15, 13), 1815);

    let fude = m.get("フーディン").unwrap();
    assert_eq!(calc_scp2(fude, 18.0, 1, 15, 15), 1279);
}

/// DCP(Defensive Combat Power, 防御的戦闘力)を計算して返す。
/// DCPは独自の指標でゲームでは表示されることはない。
/// DCPは防御力と耐久性を重視した指標となる。
#[allow(dead_code)]
pub fn calc_dcp(poke: &Pokepedia, pokemon_lv: f32, attack_iv: i32, defense_iv: i32, stamina_iv: i32) -> i32 {
    assert!(0 <= attack_iv && attack_iv <= 15);
    assert!(0 <= defense_iv && defense_iv <= 15);
    assert!(0 <= stamina_iv && stamina_iv <= 15);

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
    let m = get_pokepedia_by_name();

    let hapi = m.get("ハピナス").unwrap();
    assert_eq!(calc_dcp(hapi, 40.0, 15, 15, 15), 4340);
}

/// 引数のlimit_cp以下のCPという条件で、一番高いポケモンレベルを返す。
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
    let m = get_pokepedia_by_name();

    let kure = m.get("クレセリア").unwrap();
    assert_eq!(calc_cp(kure, 20.0, 2, 15, 13), 1500);
    assert_eq!(calc_pl_limited_by_cp(1500, 50.0, kure, 2, 15, 13), Some(20.0));
    assert_eq!(calc_pl_limited_by_cp(5000, 50.0, kure, 2, 15, 13), Some(50.0));

    let hapi = m.get("ハピナス").unwrap();
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
    let m = get_pokepedia_by_name();

    let koko = m.get("ココロモリ").unwrap();
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
