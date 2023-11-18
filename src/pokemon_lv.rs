//! 強化によって変化するポケモンレベルに関するデータや関数がある

use super::base_stats::*;

/// CP補正値と強化するときに必要な星の砂とアメの個数を持つポケモンレベル構造体。
#[derive(Debug)]
pub struct PokemonLv {
    pub lv: f32,
    pub cpm: f64,
    pub powerup_stardust: i32,
    pub powerup_candies: i32,
}

/// 引数で指定したポケモンレベルからポケモンレベル構造体を取得する。
pub fn get_pokemon_lv(lv: f32) -> &'static PokemonLv {
    let i = ((lv - 1.0) * 2.0) as usize;
    &POKEMON_LV[i]
}

/// 引数として渡された種族値、CP、個体値からポケモンレベルを計算して返す。
pub fn calc_pokemon_lv(bs: &BaseStats, cp: i32, iv_attack: i32, iv_defense: i32, iv_stamina: i32) -> f32 {
    assert!(0 <= iv_attack && iv_attack <= 15);
    assert!(0 <= iv_defense && iv_defense <= 15);
    assert!(0 <= iv_stamina && iv_stamina <= 15);

    for pl in &POKEMON_LV {
        let attack = (bs.attack + iv_attack) as f64 * pl.cpm;
        let defense = (bs.defense + iv_defense) as f64 * pl.cpm;
        let stamina = (bs.stamina + iv_stamina) as f64 * pl.cpm;

        let mut ccp = (attack * defense.sqrt() * stamina.sqrt() / 10.0) as i32;

        if ccp < 10 {
            ccp = 10;
        }

        if cp == ccp {
            return pl.lv;
        }
    }

    panic!("calc_pokemon_lv({}, {}, {}, {}, {}): not found", bs.name, cp, iv_attack, iv_defense, iv_stamina);
}

#[test]
fn test_calc_pokemon_lv() {
    let bs_name_map = get_base_stats_map_by_name();

    let saza = bs_name_map.get("サザンドラ").unwrap();

    assert_eq!(calc_pokemon_lv(saza, 2276, 10, 14, 14), 22.5);
}

/// ポケモンレベル一覧
pub static POKEMON_LV: [PokemonLv; 101] = [
    PokemonLv { lv: 1.0, cpm: 0.0939999967, powerup_stardust: 200, powerup_candies: 1 },
    PokemonLv { lv: 1.5, cpm: 0.1351374320, powerup_stardust: 200, powerup_candies: 1 },
    PokemonLv { lv: 2.0, cpm: 0.1663978695, powerup_stardust: 200, powerup_candies: 1 },
    PokemonLv { lv: 2.5, cpm: 0.1926509131, powerup_stardust: 200, powerup_candies: 1 },
    PokemonLv { lv: 3.0, cpm: 0.2157324701, powerup_stardust: 400, powerup_candies: 1 },
    PokemonLv { lv: 3.5, cpm: 0.2365726514, powerup_stardust: 400, powerup_candies: 1 },
    PokemonLv { lv: 4.0, cpm: 0.2557200491, powerup_stardust: 400, powerup_candies: 1 },
    PokemonLv { lv: 4.5, cpm: 0.2735303721, powerup_stardust: 400, powerup_candies: 1 },
    PokemonLv { lv: 5.0, cpm: 0.2902498841, powerup_stardust: 600, powerup_candies: 1 },
    PokemonLv { lv: 5.5, cpm: 0.3060573813, powerup_stardust: 600, powerup_candies: 1 },
    PokemonLv { lv: 6.0, cpm: 0.3210875988, powerup_stardust: 600, powerup_candies: 1 },
    PokemonLv { lv: 6.5, cpm: 0.3354450319, powerup_stardust: 600, powerup_candies: 1 },
    PokemonLv { lv: 7.0, cpm: 0.3492126762, powerup_stardust: 800, powerup_candies: 1 },
    PokemonLv { lv: 7.5, cpm: 0.3624577366, powerup_stardust: 800, powerup_candies: 1 },
    PokemonLv { lv: 8.0, cpm: 0.3752355873, powerup_stardust: 800, powerup_candies: 1 },
    PokemonLv { lv: 8.5, cpm: 0.3875924077, powerup_stardust: 800, powerup_candies: 1 },
    PokemonLv { lv: 9.0, cpm: 0.3995672762, powerup_stardust: 1000, powerup_candies: 1 },
    PokemonLv { lv: 9.5, cpm: 0.4111935532, powerup_stardust: 1000, powerup_candies: 1 },
    PokemonLv { lv: 10.0, cpm: 0.4225000143, powerup_stardust: 1000, powerup_candies: 1 },
    PokemonLv { lv: 10.5, cpm: 0.4329264205, powerup_stardust: 1000, powerup_candies: 1 },

    PokemonLv { lv: 11.0, cpm: 0.4431075453, powerup_stardust: 1300, powerup_candies: 2 },
    PokemonLv { lv: 11.5, cpm: 0.4530599481, powerup_stardust: 1300, powerup_candies: 2 },
    PokemonLv { lv: 12.0, cpm: 0.4627983868, powerup_stardust: 1300, powerup_candies: 2 },
    PokemonLv { lv: 12.5, cpm: 0.4723360853, powerup_stardust: 1300, powerup_candies: 2 },
    PokemonLv { lv: 13.0, cpm: 0.4816849529, powerup_stardust: 1600, powerup_candies: 2 },
    PokemonLv { lv: 13.5, cpm: 0.4908558071, powerup_stardust: 1600, powerup_candies: 2 },
    PokemonLv { lv: 14.0, cpm: 0.4998584389, powerup_stardust: 1600, powerup_candies: 2 },
    PokemonLv { lv: 14.5, cpm: 0.5087017489, powerup_stardust: 1600, powerup_candies: 2 },
    PokemonLv { lv: 15.0, cpm: 0.5173939466, powerup_stardust: 1900, powerup_candies: 2 },
    PokemonLv { lv: 15.5, cpm: 0.5259425161, powerup_stardust: 1900, powerup_candies: 2 },
    PokemonLv { lv: 16.0, cpm: 0.5343543291, powerup_stardust: 1900, powerup_candies: 2 },
    PokemonLv { lv: 16.5, cpm: 0.5426357538, powerup_stardust: 1900, powerup_candies: 2 },
    PokemonLv { lv: 17.0, cpm: 0.5507926940, powerup_stardust: 2200, powerup_candies: 2 },
    PokemonLv { lv: 17.5, cpm: 0.5588305844, powerup_stardust: 2200, powerup_candies: 2 },
    PokemonLv { lv: 18.0, cpm: 0.5667545199, powerup_stardust: 2200, powerup_candies: 2 },
    PokemonLv { lv: 18.5, cpm: 0.5745691281, powerup_stardust: 2200, powerup_candies: 2 },
    PokemonLv { lv: 19.0, cpm: 0.5822789072, powerup_stardust: 2500, powerup_candies: 2 },
    PokemonLv { lv: 19.5, cpm: 0.5898879078, powerup_stardust: 2500, powerup_candies: 2 },
    PokemonLv { lv: 20.0, cpm: 0.5974000096, powerup_stardust: 2500, powerup_candies: 2 },
    PokemonLv { lv: 20.5, cpm: 0.6048236486, powerup_stardust: 2500, powerup_candies: 2 },

    PokemonLv { lv: 21.0, cpm: 0.6121572852, powerup_stardust: 3000, powerup_candies: 3 },
    PokemonLv { lv: 21.5, cpm: 0.6194041079, powerup_stardust: 3000, powerup_candies: 3 },
    PokemonLv { lv: 22.0, cpm: 0.6265671253, powerup_stardust: 3000, powerup_candies: 3 },
    PokemonLv { lv: 22.5, cpm: 0.6336491787, powerup_stardust: 3000, powerup_candies: 3 },
    PokemonLv { lv: 23.0, cpm: 0.6406529545, powerup_stardust: 3500, powerup_candies: 3 },
    PokemonLv { lv: 23.5, cpm: 0.6475809713, powerup_stardust: 3500, powerup_candies: 3 },
    PokemonLv { lv: 24.0, cpm: 0.6544356346, powerup_stardust: 3500, powerup_candies: 3 },
    PokemonLv { lv: 24.5, cpm: 0.6612192658, powerup_stardust: 3500, powerup_candies: 3 },
    PokemonLv { lv: 25.0, cpm: 0.6679340004, powerup_stardust: 4000, powerup_candies: 3 },
    PokemonLv { lv: 25.5, cpm: 0.6745818856, powerup_stardust: 4000, powerup_candies: 3 },
    PokemonLv { lv: 26.0, cpm: 0.6811649203, powerup_stardust: 4000, powerup_candies: 4 },
    PokemonLv { lv: 26.5, cpm: 0.6876849012, powerup_stardust: 4000, powerup_candies: 4 },
    PokemonLv { lv: 27.0, cpm: 0.6941436529, powerup_stardust: 4500, powerup_candies: 4 },
    PokemonLv { lv: 27.5, cpm: 0.7005429010, powerup_stardust: 4500, powerup_candies: 4 },
    PokemonLv { lv: 28.0, cpm: 0.7068842053, powerup_stardust: 4500, powerup_candies: 4 },
    PokemonLv { lv: 28.5, cpm: 0.7131690748, powerup_stardust: 4500, powerup_candies: 4 },
    PokemonLv { lv: 29.0, cpm: 0.7193990945, powerup_stardust: 5000, powerup_candies: 4 },
    PokemonLv { lv: 29.5, cpm: 0.7255755869, powerup_stardust: 5000, powerup_candies: 4 },
    PokemonLv { lv: 30.0, cpm: 0.7317000031, powerup_stardust: 5000, powerup_candies: 4 },
    PokemonLv { lv: 30.5, cpm: 0.7347410385, powerup_stardust: 5000, powerup_candies: 4 },

    PokemonLv { lv: 31.0, cpm: 0.7377694845, powerup_stardust: 6000, powerup_candies: 6 },
    PokemonLv { lv: 31.5, cpm: 0.7407855797, powerup_stardust: 6000, powerup_candies: 6 },
    PokemonLv { lv: 32.0, cpm: 0.7437894344, powerup_stardust: 6000, powerup_candies: 6 },
    PokemonLv { lv: 32.5, cpm: 0.7467811972, powerup_stardust: 6000, powerup_candies: 6 },
    PokemonLv { lv: 33.0, cpm: 0.7497610449, powerup_stardust: 7000, powerup_candies: 8 },
    PokemonLv { lv: 33.5, cpm: 0.7527290997, powerup_stardust: 7000, powerup_candies: 8 },
    PokemonLv { lv: 34.0, cpm: 0.7556855082, powerup_stardust: 7000, powerup_candies: 8 },
    PokemonLv { lv: 34.5, cpm: 0.7586303702, powerup_stardust: 7000, powerup_candies: 8 },
    PokemonLv { lv: 35.0, cpm: 0.7615638375, powerup_stardust: 8000, powerup_candies: 10 },
    PokemonLv { lv: 35.5, cpm: 0.7644860495, powerup_stardust: 8000, powerup_candies: 10 },
    PokemonLv { lv: 36.0, cpm: 0.7673971652, powerup_stardust: 8000, powerup_candies: 10 },
    PokemonLv { lv: 36.5, cpm: 0.7702972936, powerup_stardust: 8000, powerup_candies: 10 },
    PokemonLv { lv: 37.0, cpm: 0.7731865048, powerup_stardust: 9000, powerup_candies: 12 },
    PokemonLv { lv: 37.5, cpm: 0.7760649470, powerup_stardust: 9000, powerup_candies: 12 },
    PokemonLv { lv: 38.0, cpm: 0.7789327502, powerup_stardust: 9000, powerup_candies: 12 },
    PokemonLv { lv: 38.5, cpm: 0.7817900507, powerup_stardust: 9000, powerup_candies: 12 },
    PokemonLv { lv: 39.0, cpm: 0.7846369743, powerup_stardust: 10000, powerup_candies: 15 },
    PokemonLv { lv: 39.5, cpm: 0.7874736085, powerup_stardust: 10000, powerup_candies: 15 },
    PokemonLv { lv: 40.0, cpm: 0.7903000116, powerup_stardust: 10000, powerup_candies: 10 },
    PokemonLv { lv: 40.5, cpm: 0.792803968, powerup_stardust: 10000, powerup_candies: 10 },

    PokemonLv { lv: 41.0, cpm: 0.7953000068, powerup_stardust: 11000, powerup_candies: 10 },
    PokemonLv { lv: 41.5, cpm: 0.797800015, powerup_stardust: 11000, powerup_candies: 10 },
    PokemonLv { lv: 42.0, cpm: 0.8003000020, powerup_stardust: 11000, powerup_candies: 12 },
    PokemonLv { lv: 42.5, cpm: 0.802799995, powerup_stardust: 11000, powerup_candies: 12 },
    PokemonLv { lv: 43.0, cpm: 0.8052999973, powerup_stardust: 12000, powerup_candies: 12 },
    PokemonLv { lv: 43.5, cpm: 0.8078, powerup_stardust: 12000, powerup_candies: 12 },
    PokemonLv { lv: 44.0, cpm: 0.8102999925, powerup_stardust: 12000, powerup_candies: 15 },
    PokemonLv { lv: 44.5, cpm: 0.812799985, powerup_stardust: 12000, powerup_candies: 15 },
    PokemonLv { lv: 45.0, cpm: 0.8152999877, powerup_stardust: 13000, powerup_candies: 15 },
    PokemonLv { lv: 45.5, cpm: 0.81779999, powerup_stardust: 13000, powerup_candies: 15 },
    PokemonLv { lv: 46.0, cpm: 0.8202999830, powerup_stardust: 13000, powerup_candies: 17 },
    PokemonLv { lv: 46.5, cpm: 0.82279999, powerup_stardust: 13000, powerup_candies: 17 },
    PokemonLv { lv: 47.0, cpm: 0.8252999782, powerup_stardust: 14000, powerup_candies: 17 },
    PokemonLv { lv: 47.5, cpm: 0.82779999, powerup_stardust: 14000, powerup_candies: 17 },
    PokemonLv { lv: 48.0, cpm: 0.8302999734, powerup_stardust: 14000, powerup_candies: 20 },
    PokemonLv { lv: 48.5, cpm: 0.83279999, powerup_stardust: 14000, powerup_candies: 20 },
    PokemonLv { lv: 49.0, cpm: 0.8353000283, powerup_stardust: 15000, powerup_candies: 20 },
    PokemonLv { lv: 49.5, cpm: 0.83779999, powerup_stardust: 15000, powerup_candies: 20 },
    PokemonLv { lv: 50.0, cpm: 0.84029999, powerup_stardust: 0, powerup_candies: 0 },
    PokemonLv { lv: 50.5, cpm: 0.84279999, powerup_stardust: 0, powerup_candies: 0 },

    PokemonLv { lv: 51.0, cpm: 0.84529999, powerup_stardust: 0, powerup_candies: 0 },
];
