//! CP補正値

use super::pokepedia::*;

const NUM_CPM: usize = 101;

/// 引数で指定したポケモンレベルからCP補正値を取得する。
pub fn get_cpm(lv: f32) -> f64 {
    assert!(0.0 <= lv && lv <= 51.0);

    let i = ((lv - 1.0) * 2.0) as usize;
    CPM[i]
}

#[test]
fn test_get_cpm() {
    assert_eq!(get_cpm(1.0), 0.0939999967);
    assert_eq!(get_cpm(11.5), 0.4530599481);
    assert_eq!(get_cpm(51.0), 0.84529999);
}

/// 引数として渡された種族値、CP、個体値からポケモンレベルを計算して返す。
pub fn calc_pokemon_lv(poke: &Pokepedia, cp: i32, attack_iv: i32, defense_iv: i32, stamina_iv: i32) -> Option<f32> {
    assert!(0 <= attack_iv && attack_iv <= 15);
    assert!(0 <= defense_iv && defense_iv <= 15);
    assert!(0 <= stamina_iv && stamina_iv <= 15);

    for (i, cpm) in (0..NUM_CPM).zip(CPM) {
        let attack = (poke.attack_st + attack_iv) as f64 * cpm;
        let defense = (poke.defense_st + defense_iv) as f64 * cpm;
        let stamina = (poke.stamina_st + stamina_iv) as f64 * cpm;

        let mut ccp = (attack * defense.sqrt() * stamina.sqrt() / 10.0) as i32;

        if ccp < 10 {
            ccp = 10;
        }

        if cp == ccp {
            let pl = (i + 2) as f32 / 2.0;
            return Some(pl);
        }
    }

    None
}

#[test]
fn test_calc_pokemon_lv() {
    let m = get_pokepedia_by_name();

    let saza = m.get("サザンドラ").unwrap();

    assert_eq!(calc_pokemon_lv(saza, 2276, 10, 14, 14), Some(22.5));
    assert_eq!(calc_pokemon_lv(saza, 2277, 10, 14, 14), None);
}

/// CP補正値
static CPM: [f64; NUM_CPM] = [
    0.0939999967, 0.1351374320, 0.1663978695, 0.1926509131, 0.2157324701, 0.2365726514, 0.2557200491, 0.2735303721, 0.2902498841, 0.3060573813,
    0.3210875988, 0.3354450319, 0.3492126762, 0.3624577366, 0.3752355873, 0.3875924077, 0.3995672762, 0.4111935532, 0.4225000143, 0.4329264205,

    0.4431075453, 0.4530599481, 0.4627983868, 0.4723360853, 0.4816849529, 0.4908558071, 0.4998584389, 0.5087017489, 0.5173939466, 0.5259425161,
    0.5343543291, 0.5426357538, 0.5507926940, 0.5588305844, 0.5667545199, 0.5745691281, 0.5822789072, 0.5898879078, 0.5974000096, 0.6048236486,

    0.6121572852, 0.6194041079, 0.6265671253, 0.6336491787, 0.6406529545, 0.6475809713, 0.6544356346, 0.6612192658, 0.6679340004, 0.6745818856,
    0.6811649203, 0.6876849012, 0.6941436529, 0.7005429010, 0.7068842053, 0.7131690748, 0.7193990945, 0.7255755869, 0.7317000031, 0.7347410385,

    0.7377694845, 0.7407855797, 0.7437894344, 0.7467811972, 0.7497610449, 0.7527290997, 0.7556855082, 0.7586303702, 0.7615638375, 0.7644860495,
    0.7673971652, 0.7702972936, 0.7731865048, 0.7760649470, 0.7789327502, 0.7817900507, 0.7846369743, 0.7874736085, 0.7903000116, 0.792803968,

    0.7953000068, 0.797800015, 0.8003000020, 0.802799995, 0.8052999973, 0.8078, 0.8102999925, 0.812799985, 0.8152999877, 0.81779999,
    0.8202999830, 0.82279999, 0.8252999782, 0.82779999, 0.8302999734, 0.83279999, 0.8353000283, 0.83779999, 0.84029999, 0.84279999,

    0.84529999,
];
