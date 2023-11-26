//! ノーマル技やスペシャル技を保持する。

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::types::Type;

pub const STAB: f64 = 1.2;  // STAB(Same Type Attack Bonus, タイプ一致ボーナス)

// ノーマル技構造体
#[derive(Debug)]
pub struct FastMove {
    no: &'static str,
    name: &'static str,
    mtype: Type,

    power: i32,  // 威力
    energy: i32,  // エネルギー充填
    turns: i32,  // 硬直ターン数。わざ開始から次の動作を開始できるまでのターン数。
}

impl FastMove {
    pub fn no(&self) -> &'static str {
        self.no
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn mtype(&self) -> Type {
        self.mtype
    }

    pub fn power(&self) -> i32 {
        self.power
    }

    pub fn energy(&self) -> i32 {
        self.energy
    }

    pub fn turns(&self) -> i32 {
        self.turns
    }

    pub fn real_power(&self, types: &[Type]) -> f64 {
        if types.iter().any(|t| t == &self.mtype) {
            self.power as f64 * STAB
        } else {
            self.power as f64
        }
    }
}

#[test]
fn test_real_power() {
    let m = fast_move_by_name("マッドショット").unwrap();
    assert_eq!(m.mtype(), Type::Ground);
    assert_eq!(m.power(), 3);
    assert_eq!(m.real_power(&vec![Type::Normal, Type::Flare]), 3.0);
    assert_eq!(m.real_power(&vec![Type::Ground]), 3.0 * STAB);
}

/// ステータス変化構造体。
/// 値の意味は(自分の攻撃, 自分の防御, 相手の攻撃, 相手の防御)
/// 例えば相手の攻撃が-1になるならBuff(0, 0, -1, 0)
#[derive(Debug, Clone, Copy)]
pub struct Buff(pub i8, pub i8, pub i8, pub i8);

/// スペシャル技構造体
#[derive(Debug)]
pub struct ChargeMove {
    no: &'static str,
    name: &'static str,
    mtype: Type,

    // トレーナーバトル
    power: i32,
    energy: i32,  // エネルギー消費
    buff: Option<Buff>,  // ステータス変化
    buff_prob: f32,  // ステータス変化確率
}

impl ChargeMove {
    pub fn no(&self) -> &'static str {
        self.no
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn mtype(&self) -> Type {
        self.mtype
    }

    pub fn power(&self) -> i32 {
        self.power
    }

    pub fn energy(&self) -> i32 {
        self.energy
    }

    pub fn buff(&self) -> Option<Buff> {
        self.buff
    }

    pub fn buff_prob(&self) -> f32 {
        self.buff_prob
    }

    pub fn real_power(&self, types: &[Type]) -> f64 {
        if types.iter().any(|t| t == &self.mtype) {
            self.power as f64 * STAB
        } else {
            self.power as f64
        }
    }
}

/// 技Noからノーマル技を取得するマップを返す。
pub fn fast_move_by_no(no: &str) -> Option<&'static FastMove> {
    let m = FAST_MOVE_NO_MAP.get_or_init(|| {
        let mut m = HashMap::new();

        for mv in &FAST_MOVES {
            if mv.no() != "000" {
                m.insert(mv.no().to_string(), mv);
            }
        }

        m
    });

    match m.get(no) {
        None => None,
        Some(p) => Some(*p)
    }
}

#[test]
fn test_fast_move_by_no() {
    let m = fast_move_by_no("216").unwrap();
    assert_eq!(m.name, "マッドショット");
}

/// 技名からノーマル技を取得するマップを返す。
pub fn fast_move_by_name(name: &str) -> Option<&'static FastMove> {
    let m = FAST_MOVE_NAME_MAP.get_or_init(|| {
        let mut m = HashMap::new();

        for mv in &FAST_MOVES {
            if mv.no() != "000" {
                m.insert(mv.name().to_string(), mv);
            }
        }

        m
    });

    match m.get(name) {
        None => None,
        Some(p) => Some(*p)
    }
}

#[test]
fn test_fast_move_by_name() {
    let m = fast_move_by_name("マッドショット").unwrap();
    assert_eq!(m.no, "216");
}

/// 技Noからスペシャル技を取得するマップを返す。
pub fn charge_move_by_no(no: &str) -> Option<&'static ChargeMove> {
    let m = CHARGE_MOVE_NO_MAP.get_or_init(|| {
        let mut m = HashMap::new();

        for mv in &CHARGE_MOVES {
            if mv.no() != "000" {
                m.insert(mv.no().to_string(), mv);
            }
        }

        m
    });

    match m.get(no) {
        None => None,
        Some(p) => Some(*p)
    }
}

#[test]
fn test_charge_move_by_no() {
    let m = charge_move_by_no("090").unwrap();
    assert_eq!(m.name, "ヘドロばくだん");
}

/// 技名からスペシャル技を取得するマップを返す。
pub fn charge_move_by_name(name: &str) -> Option<&'static ChargeMove> {
    let m = CHARGE_MOVE_NAME_MAP.get_or_init(|| {
        let mut m = HashMap::new();

        for mv in &CHARGE_MOVES {
            if mv.no() != "000" {
                m.insert(mv.name().to_string(), mv);
            }
        }

        m
    });

    match m.get(name) {
        None => None,
        Some(p) => Some(*p)
    }
}

#[test]
fn test_charge_move_by_name() {
    let m = charge_move_by_name("ヘドロばくだん").unwrap();
    assert_eq!(m.no, "090");
}

static FAST_MOVE_NO_MAP: OnceLock<HashMap<String, &'static FastMove>> = OnceLock::new();
static FAST_MOVE_NAME_MAP: OnceLock<HashMap<String, &'static FastMove>> = OnceLock::new();
static CHARGE_MOVE_NO_MAP: OnceLock<HashMap<String, &'static ChargeMove>> = OnceLock::new();
static CHARGE_MOVE_NAME_MAP: OnceLock<HashMap<String, &'static ChargeMove>> = OnceLock::new();

pub const NUM_FAST_MOVES: usize = 188;

const DUMMY_FM: FastMove = FastMove { no: "000", name: "dummy fast move", mtype: Type::Normal, power: 0, energy: 0, turns: 0 };

/// ノーマル技一覧
pub static FAST_MOVES: [FastMove; NUM_FAST_MOVES] = [
    FastMove { no: "200", name: "れんぞくぎり", mtype: Type::Bug, power: 2, energy: 4, turns: 1 },
    FastMove { no: "201", name: "むしくい", mtype: Type::Bug, power: 3, energy: 3, turns: 1 },
    FastMove { no: "202", name: "かみつく", mtype: Type::Dark, power: 4, energy: 2, turns: 1 },
    FastMove { no: "203", name: "ふいうち", mtype: Type::Dark, power: 5, energy: 7, turns: 2 },
    FastMove { no: "204", name: "りゅうのいぶき", mtype: Type::Dragon, power: 4, energy: 3, turns: 1 },
    FastMove { no: "205", name: "でんきショック", mtype: Type::Electric, power: 3, energy: 9, turns: 2 },
    FastMove { no: "206", name: "スパーク", mtype: Type::Electric, power: 6, energy: 7, turns: 2 },
    FastMove { no: "207", name: "けたぐり", mtype: Type::Fighting, power: 4, energy: 5, turns: 2 },
    FastMove { no: "208", name: "からてチョップ", mtype: Type::Fighting, power: 5, energy: 8, turns: 2 },
    FastMove { no: "209", name: "ひのこ", mtype: Type::Flare, power: 7, energy: 6, turns: 2 },

    FastMove { no: "210", name: "つばさでうつ", mtype: Type::Flying, power: 5, energy: 8, turns: 2 },
    FastMove { no: "211", name: "つつく", mtype: Type::Flying, power: 6, energy: 5, turns: 2 },
    FastMove { no: "212", name: "したでなめる", mtype: Type::Ghost, power: 3, energy: 3, turns: 1 },
    FastMove { no: "213", name: "シャドークロー", mtype: Type::Ghost, power: 6, energy: 8, turns: 2 },
    FastMove { no: "214", name: "つるのムチ", mtype: Type::Grass, power: 5, energy: 8, turns: 2 },
    FastMove { no: "215", name: "はっぱカッター", mtype: Type::Grass, power: 10, energy: 4, turns: 2 },
    FastMove { no: "216", name: "マッドショット", mtype: Type::Ground, power: 3, energy: 9, turns: 2 },
    FastMove { no: "217", name: "こおりのつぶて", mtype: Type::Ice, power: 9, energy: 10, turns: 3 },
    FastMove { no: "218", name: "こおりのいぶき", mtype: Type::Ice, power: 7, energy: 5, turns: 2 },
    FastMove { no: "219", name: "でんこうせっか", mtype: Type::Normal, power: 5, energy: 8, turns: 2 },

    FastMove { no: "220", name: "ひっかく", mtype: Type::Normal, power: 4, energy: 2, turns: 1 },
    FastMove { no: "221", name: "たいあたり", mtype: Type::Normal, power: 3, energy: 3, turns: 1 },
    FastMove { no: "222", name: "はたく", mtype: Type::Normal, power: 4, energy: 4, turns: 2 },
    FastMove { no: "223", name: "いあいぎり", mtype: Type::Normal, power: 3, energy: 2, turns: 1 },
    FastMove { no: "224", name: "どくづき", mtype: Type::Poison, power: 7, energy: 7, turns: 2 },
    FastMove { no: "225", name: "ようかいえき", mtype: Type::Poison, power: 6, energy: 5, turns: 2 },
    FastMove { no: "226", name: "サイコカッター", mtype: Type::Psychic, power: 3, energy: 9, turns: 2 },
    FastMove { no: "227", name: "いわおとし", mtype: Type::Rock, power: 8, energy: 5, turns: 2 },
    FastMove { no: "228", name: "メタルクロー", mtype: Type::Steel, power: 5, energy: 6, turns: 2 },
    FastMove { no: "229", name: "バレットパンチ", mtype: Type::Steel, power: 6, energy: 7, turns: 2 },

    FastMove { no: "230", name: "みずでっぽう", mtype: Type::Water, power: 3, energy: 3, turns: 1 },
    FastMove { no: "231", name: "はねる", mtype: Type::Water, power: 0, energy: 12, turns: 4 },
    FastMove { no: "232", name: "みずでっぽう(カメックス)", mtype: Type::Water, power: 6, energy: 4, turns: 2 },
    FastMove { no: "233", name: "どろかけ", mtype: Type::Ground, power: 11, energy: 8, turns: 3 },
    FastMove { no: "234", name: "しねんのずつき", mtype: Type::Psychic, power: 8, energy: 6, turns: 3 },
    FastMove { no: "235", name: "ねんりき", mtype: Type::Psychic, power: 16, energy: 12, turns: 4 },
    FastMove { no: "236", name: "どくばり", mtype: Type::Poison, power: 3, energy: 9, turns: 2 },
    FastMove { no: "237", name: "あわ", mtype: Type::Water, power: 7, energy: 11, turns: 3 },
    FastMove { no: "238", name: "だましうち", mtype: Type::Dark, power: 6, energy: 6, turns: 2 },
    FastMove { no: "239", name: "はがねのつばさ", mtype: Type::Steel, power: 7, energy: 5, turns: 2 },

    FastMove { no: "240", name: "ほのおのキバ", mtype: Type::Flare, power: 8, energy: 5, turns: 2 },
    FastMove { no: "241", name: "いわくだき", mtype: Type::Fighting, power: 9, energy: 7, turns: 3 },
    FastMove { no: "242", name: "へんしん", mtype: Type::Normal, power: 0, energy: 0, turns: 3 },
    FastMove { no: "243", name: "カウンター", mtype: Type::Fighting, power: 8, energy: 7, turns: 2 },
    FastMove { no: "244", name: "こなゆき", mtype: Type::Ice, power: 5, energy: 8, turns: 2 },
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "249", name: "チャージビーム", mtype: Type::Electric, power: 5, energy: 11, turns: 3 },

    FastMove { no: "250", name: "ボルトチェンジ", mtype: Type::Electric, power: 12, energy: 16, turns: 4 },
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "253", name: "ドラゴンテール", mtype: Type::Dragon, power: 13, energy: 9, turns: 3 },
    DUMMY_FM,
    FastMove { no: "255", name: "エアスラッシュ", mtype: Type::Flying, power: 9, energy: 9, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,

    FastMove { no: "260", name: "まとわりつく", mtype: Type::Bug, power: 6, energy: 12, turns: 3 },
    FastMove { no: "261", name: "むしのていこう", mtype: Type::Bug, power: 9, energy: 8, turns: 3 },
    DUMMY_FM,
    FastMove { no: "263", name: "おどろかす", mtype: Type::Ghost, power: 5, energy: 10, turns: 3 },
    FastMove { no: "264", name: "たたりめ", mtype: Type::Ghost, power: 6, energy: 12, turns: 3 },
    DUMMY_FM,
    FastMove { no: "266", name: "アイアンテール", mtype: Type::Steel, power: 9, energy: 6, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "269", name: "ほのおのうず", mtype: Type::Flare, power: 9, energy: 10, turns: 3 },

    DUMMY_FM,
    FastMove { no: "271", name: "タネマシンガン", mtype: Type::Grass, power: 5, energy: 13, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "274", name: "じんつうりき", mtype: Type::Psychic, power: 8, energy: 10, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "278", name: "バークアウト", mtype: Type::Dark, power: 5, energy: 13, turns: 3 },
    DUMMY_FM,

    DUMMY_FM,
    FastMove { no: "281", name: "めざめるパワー※", mtype: Type::Normal, power: 9, energy: 8, turns: 3 },
    FastMove { no: "282", name: "とっしん", mtype: Type::Normal, power: 5, energy: 8, turns: 3 },
    FastMove { no: "283", name: "たきのぼり", mtype: Type::Water, power: 12, energy: 8, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "287", name: "あくび", mtype: Type::Normal, power: 0, energy: 12, turns: 4 },
    DUMMY_FM,
    DUMMY_FM,

    DUMMY_FM,
    FastMove { no: "291", name: "プレゼント", mtype: Type::Normal, power: 3, energy: 12, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "297", name: "うちおとす", mtype: Type::Rock, power: 12, energy: 8, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,

    DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM,
    DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM,

    FastMove { no: "320", name: "あまえる", mtype: Type::Fairy, power: 15, energy: 6, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "325", name: "ロックオン", mtype: Type::Normal, power: 1, energy: 5, turns: 1 },
    FastMove { no: "326", name: "かみなりのキバ", mtype: Type::Electric, power: 8, energy: 5, turns: 2 },
    FastMove { no: "327", name: "こおりのキバ", mtype: Type::Ice, power: 8, energy: 5, turns: 2 },
    DUMMY_FM,
    DUMMY_FM,

    DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM,

    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "345", name: "かぜおこし", mtype: Type::Flying, power: 16, energy: 12, turns: 4 },
    FastMove { no: "346", name: "やきつくす", mtype: Type::Flare, power: 15, energy: 20, turns: 5 },
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,

    FastMove { no: "350", name: "ようせいのかぜ", mtype: Type::Fairy, power: 3, energy: 9, turns: 2 },
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "356", name: "にどげり", mtype: Type::Fighting, power: 8, energy: 12, turns: 3 },
    FastMove { no: "357", name: "マジカルリーフ", mtype: Type::Grass, power: 10, energy: 10, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,

    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "368", name: "ころがる", mtype: Type::Rock, power: 5, energy: 13, turns: 3 },
    DUMMY_FM,

    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "373", name: "みずしゅりけん", mtype: Type::Water, power: 6, energy: 14, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,

    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "385", name: "このは", mtype: Type::Grass, power: 6, energy: 7, turns: 2 },
    DUMMY_FM,
    FastMove { no: "387", name: "ジオコントロール", mtype: Type::Fairy, power: 4, energy: 13, turns: 3 },
];

pub const NUM_CHARGE_MOVES: usize = 394;

const DUMMY_CM: ChargeMove = ChargeMove { no: "000", name: "dummy charge move", mtype: Type::Normal, power: 0, energy: 0, buff: None, buff_prob: 0.0 };

/// スペシャル技一覧
pub static CHARGE_MOVES: [ChargeMove; NUM_CHARGE_MOVES] = [
    DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM,

    DUMMY_CM, DUMMY_CM, DUMMY_CM,
    ChargeMove { no: "013", name: "まきつく", mtype: Type::Normal, power: 60, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "014", name: "はかいこうせん", mtype: Type::Normal, power: 150, energy: 80, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "016", name: "あくのはどう", mtype: Type::Dark, power: 80, energy: 50, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "018", name: "ヘドロこうげき", mtype: Type::Poison, power: 50, energy: 40, buff: None, buff_prob: 0.0 },
    DUMMY_CM,

    ChargeMove { no: "020", name: "はさむ", mtype: Type::Normal, power: 40, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "021", name: "かえんぐるま", mtype: Type::Flare, power: 60, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "022", name: "メガホーン", mtype: Type::Bug, power: 110, energy: 55, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "024", name: "かえんほうしゃ", mtype: Type::Flare, power: 90, energy: 55, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "026", name: "あなをほる", mtype: Type::Ground, power: 80, energy: 50, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "028", name: "クロスチョップ", mtype: Type::Fighting, power: 50, energy: 35, buff: None, buff_prob: 0.0 },
    DUMMY_CM,

    ChargeMove { no: "030", name: "サイケこうせん", mtype: Type::Psychic, power: 70, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "031", name: "じしん", mtype: Type::Ground, power: 110, energy: 65, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "032", name: "ストーンエッジ", mtype: Type::Rock, power: 100, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "033", name: "れいとうパンチ", mtype: Type::Ice, power: 55, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "034", name: "ハートスタンプ", mtype: Type::Psychic, power: 40, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "035", name: "ほうでん", mtype: Type::Electric, power: 65, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "036", name: "ラスターカノン", mtype: Type::Steel, power: 110, energy: 70, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "038", name: "ドリルくちばし", mtype: Type::Flying, power: 65, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "039", name: "れいとうビーム", mtype: Type::Ice, power: 90, energy: 55, buff: None, buff_prob: 0.0 },

    ChargeMove { no: "040", name: "ふぶき", mtype: Type::Ice, power: 140, energy: 75, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "042", name: "ねっぷう", mtype: Type::Flare, power: 95, energy: 75, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "045", name: "つばめがえし", mtype: Type::Flying, power: 55, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "046", name: "ドリルライナー", mtype: Type::Ground, power: 80, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "047", name: "はなふぶき", mtype: Type::Grass, power: 110, energy: 65, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "048", name: "メガドレイン", mtype: Type::Grass, power: 25, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "049", name: "むしのさざめき", mtype: Type::Bug, power: 100, energy: 60, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 30.0 },

    ChargeMove { no: "050", name: "どくどくのキバ", mtype: Type::Poison, power: 45, energy: 40, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 100.0 },
    ChargeMove { no: "051", name: "つじぎり", mtype: Type::Dark, power: 50, energy: 35, buff: Some(Buff(2, 0, 0, 0)), buff_prob: 12.5 },
    DUMMY_CM,
    ChargeMove { no: "053", name: "バブルこうせん", mtype: Type::Water, power: 25, energy: 40, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 100.0 },
    ChargeMove { no: "054", name: "じごくぐるま", mtype: Type::Fighting, power: 60, energy: 50, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "056", name: "ローキック", mtype: Type::Fighting, power: 40, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "057", name: "アクアジェット", mtype: Type::Water, power: 45, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "058", name: "アクアテール", mtype: Type::Water, power: 50, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "059", name: "タネばくだん", mtype: Type::Grass, power: 60, energy: 45, buff: None, buff_prob: 0.0 },

    ChargeMove { no: "060", name: "サイコショック", mtype: Type::Psychic, power: 70, energy: 45, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "062", name: "げんしのちから", mtype: Type::Rock, power: 60, energy: 45, buff: Some(Buff(1, 1, 0, 0)), buff_prob: 10.0 },
    ChargeMove { no: "063", name: "がんせきふうじ", mtype: Type::Rock, power: 70, energy: 60, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 100.0 },
    ChargeMove { no: "064", name: "いわなだれ", mtype: Type::Rock, power: 75, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "065", name: "パワージェム", mtype: Type::Rock, power: 80, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "066", name: "かげうち", mtype: Type::Ghost, power: 50, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "067", name: "シャドーパンチ", mtype: Type::Ghost, power: 40, energy: 35, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "069", name: "あやしいかぜ", mtype: Type::Ghost, power: 45, energy: 45, buff: Some(Buff(1, 1, 0, 0)), buff_prob: 10.0 },

    ChargeMove { no: "070", name: "シャドーボール", mtype: Type::Ghost, power: 100, energy: 55, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "072", name: "マグネットボム", mtype: Type::Steel, power: 70, energy: 45, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "074", name: "アイアンヘッド", mtype: Type::Steel, power: 70, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "075", name: "パラボラチャージ", mtype: Type::Electric, power: 65, energy: 55, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "077", name: "かみなりパンチ", mtype: Type::Electric, power: 55, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "078", name: "かみなり", mtype: Type::Electric, power: 100, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "079", name: "10まんボルト", mtype: Type::Electric, power: 90, energy: 55, buff: None, buff_prob: 0.0 },

    ChargeMove { no: "080", name: "たつまき", mtype: Type::Dragon, power: 45, energy: 45, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "082", name: "りゅうのはどう", mtype: Type::Dragon, power: 90, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "083", name: "ドラゴンクロー", mtype: Type::Dragon, power: 50, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "084", name: "チャームボイス", mtype: Type::Fairy, power: 70, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "085", name: "ドレインキッス", mtype: Type::Fairy, power: 60, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "086", name: "マジカルシャイン", mtype: Type::Fairy, power: 110, energy: 70, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "087", name: "ムーンフォース", mtype: Type::Fairy, power: 110, energy: 60, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 10.0 },
    ChargeMove { no: "088", name: "じゃれつく", mtype: Type::Fairy, power: 90, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "089", name: "クロスポイズン", mtype: Type::Poison, power: 50, energy: 35, buff: Some(Buff(2, 0, 0, 0)), buff_prob: 12.5 },

    ChargeMove { no: "090", name: "ヘドロばくだん", mtype: Type::Poison, power: 80, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "091", name: "ヘドロウェーブ", mtype: Type::Poison, power: 110, energy: 65, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "092", name: "ダストシュート", mtype: Type::Poison, power: 130, energy: 75, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "094", name: "ホネこんぼう", mtype: Type::Ground, power: 40, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "095", name: "じならし", mtype: Type::Ground, power: 80, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "096", name: "どろばくだん", mtype: Type::Ground, power: 60, energy: 40, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "099", name: "シグナルビーム", mtype: Type::Bug, power: 75, energy: 55, buff: Some(Buff(0, 0, -1, -1)), buff_prob: 20.0 },

    ChargeMove { no: "100", name: "シザークロス", mtype: Type::Bug, power: 65, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "101", name: "ニトロチャージ", mtype: Type::Flare, power: 65, energy: 50, buff: Some(Buff(1, 0, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "102", name: "はじけるほのお", mtype: Type::Flare, power: 70, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "103", name: "だいもんじ", mtype: Type::Flare, power: 140, energy: 80, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "104", name: "しおみず", mtype: Type::Water, power: 60, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "105", name: "みずのはどう", mtype: Type::Water, power: 70, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "106", name: "ねっとう", mtype: Type::Water, power: 80, energy: 50, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 30.0 },
    ChargeMove { no: "107", name: "ハイドロポンプ", mtype: Type::Water, power: 130, energy: 75, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "108", name: "サイコキネシス", mtype: Type::Psychic, power: 85, energy: 55, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 10.0 },
    ChargeMove { no: "109", name: "サイコブレイク", mtype: Type::Psychic, power: 90, energy: 45, buff: None, buff_prob: 0.0 },

    DUMMY_CM,
    ChargeMove { no: "111", name: "こごえるかぜ", mtype: Type::Ice, power: 60, energy: 45, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 100.0 },
    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "114", name: "ギガドレイン", mtype: Type::Grass, power: 50, energy: 80, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "115", name: "ほのおのパンチ", mtype: Type::Flare, power: 55, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "116", name: "ソーラービーム", mtype: Type::Grass, power: 150, energy: 80, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "117", name: "リーフブレード", mtype: Type::Grass, power: 70, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "118", name: "パワーウィップ", mtype: Type::Grass, power: 90, energy: 50, buff: None, buff_prob: 0.0 },
    DUMMY_CM,

    DUMMY_CM,
    ChargeMove { no: "121", name: "エアカッター", mtype: Type::Flying, power: 60, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "122", name: "ぼうふう", mtype: Type::Flying, power: 110, energy: 65, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "123", name: "かわらわり", mtype: Type::Fighting, power: 40, energy: 35, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "125", name: "スピードスター", mtype: Type::Normal, power: 60, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "126", name: "つのでつく", mtype: Type::Normal, power: 40, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "127", name: "ふみつけ", mtype: Type::Normal, power: 55, energy: 40, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "129", name: "ひっさつまえば", mtype: Type::Normal, power: 80, energy: 50, buff: None, buff_prob: 0.0 },

    DUMMY_CM,
    ChargeMove { no: "131", name: "のしかかり", mtype: Type::Normal, power: 60, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "132", name: "ねむる", mtype: Type::Normal, power: 50, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "133", name: "わるあがき", mtype: Type::Normal, power: 35, energy: 100, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "134", name: "ねっとう(カメックス)", mtype: Type::Water, power: 50, energy: 80, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "135", name: "ハイドロポンプ(カメックス)", mtype: Type::Water, power: 90, energy: 80, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "136", name: "まきつく(緑)", mtype: Type::Normal, power: 25, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "137", name: "まきつく(ピンク)", mtype: Type::Normal, power: 25, energy: 45, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    DUMMY_CM,

    DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, // 140-149
    DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, // 150-159
    DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, // 160-169
    DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, // 170-179
    DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, // 180-189
    DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, // 190-199
    DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, // 200-209
    DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, // 210-219
    DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, // 220-229
    DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, // 230-239

    DUMMY_CM,
    DUMMY_CM,
    DUMMY_CM,
    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "245", name: "インファイト", mtype: Type::Fighting, power: 100, energy: 45, buff: Some(Buff(0, -2, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "246", name: "ばくれつパンチ", mtype: Type::Fighting, power: 90, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "247", name: "きあいだま", mtype: Type::Fighting, power: 150, energy: 75, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "248", name: "オーロラビーム", mtype: Type::Ice, power: 80, energy: 60, buff: None, buff_prob: 0.0 },
    DUMMY_CM,

    DUMMY_CM,
    ChargeMove { no: "251", name: "ワイルドボルト", mtype: Type::Electric, power: 100, energy: 45, buff: Some(Buff(0, -2, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "252", name: "でんじほう", mtype: Type::Electric, power: 150, energy: 80, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 66.0 },
    DUMMY_CM,
    ChargeMove { no: "254", name: "ゆきなだれ", mtype: Type::Ice, power: 90, energy: 45, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "256", name: "ブレイブバード", mtype: Type::Flying, power: 130, energy: 55, buff: Some(Buff(0, -3, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "257", name: "ゴッドバード", mtype: Type::Flying, power: 75, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "258", name: "すなじごく", mtype: Type::Ground, power: 25, energy: 40, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 100.0 },
    ChargeMove { no: "259", name: "ロックブラスト", mtype: Type::Rock, power: 50, energy: 40, buff: None, buff_prob: 0.0 },

    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "262", name: "ぎんいろのかぜ", mtype: Type::Bug, power: 60, energy: 45, buff: Some(Buff(1, 1, 0, 0)), buff_prob: 10.0 },
    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "265", name: "ナイトヘッド", mtype: Type::Ghost, power: 60, energy: 55, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "267", name: "ジャイロボール", mtype: Type::Steel, power: 80, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "268", name: "ヘビーボンバー", mtype: Type::Steel, power: 70, energy: 50, buff: None, buff_prob: 0.0 },
    DUMMY_CM,

    ChargeMove { no: "270", name: "オーバーヒート", mtype: Type::Flare, power: 130, energy: 55, buff: Some(Buff(-2, 0, 0, 0)), buff_prob: 100.0 },
    DUMMY_CM,
    ChargeMove { no: "272", name: "くさむすび", mtype: Type::Grass, power: 90, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "273", name: "エナジーボール", mtype: Type::Grass, power: 90, energy: 55, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 10.0 },
    DUMMY_CM,
    ChargeMove { no: "275", name: "みらいよち", mtype: Type::Psychic, power: 120, energy: 65, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "276", name: "ミラーコート", mtype: Type::Psychic, power: 60, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "277", name: "げきりん", mtype: Type::Dragon, power: 110, energy: 60, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "279", name: "かみくだく", mtype: Type::Dark, power: 70, energy: 45, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 30.0 },

    ChargeMove { no: "280", name: "イカサマ", mtype: Type::Dark, power: 70, energy: 45, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "284", name: "なみのり", mtype: Type::Water, power: 65, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "285", name: "りゅうせいぐん", mtype: Type::Dragon, power: 150, energy: 65, buff: Some(Buff(-2, 0, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "286", name: "はめつのねがい", mtype: Type::Steel, power: 75, energy: 40, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "288", name: "サイコブースト", mtype: Type::Psychic, power: 70, energy: 35, buff: Some(Buff(-2, 0, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "289", name: "こんげんのはどう", mtype: Type::Water, power: 130, energy: 60, buff: None, buff_prob: 0.0 },

    ChargeMove { no: "290", name: "だんがいのつるぎ", mtype: Type::Ground, power: 130, energy: 60, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "292", name: "ウェザーボール", mtype: Type::Flare, power: 55, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "293", name: "ウェザーボール", mtype: Type::Ice, power: 55, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "294", name: "ウェザーボール", mtype: Type::Rock, power: 55, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "295", name: "ウェザーボール", mtype: Type::Water, power: 55, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "296", name: "ハードプラント", mtype: Type::Grass, power: 100, energy: 45, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "298", name: "ブラストバーン", mtype: Type::Flare, power: 110, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "299", name: "ハイドロカノン", mtype: Type::Water, power: 80, energy: 40, buff: None, buff_prob: 0.0 },

    ChargeMove { no: "300", name: "とっておき", mtype: Type::Normal, power: 90, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "301", name: "コメットパンチ", mtype: Type::Steel, power: 100, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "302", name: "ロケットずつき", mtype: Type::Normal, power: 130, energy: 75, buff: Some(Buff(0, 1, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "303", name: "アシッドボム", mtype: Type::Poison, power: 20, energy: 45, buff: Some(Buff(0, 0, 0, -2)), buff_prob: 100.0 },
    ChargeMove { no: "304", name: "だいちのちから", mtype: Type::Ground, power: 90, energy: 55, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 10.0 },
    ChargeMove { no: "305", name: "クラブハンマー", mtype: Type::Water, power: 85, energy: 50, buff: Some(Buff(2, 0, 0, 0)), buff_prob: 12.5 },
    ChargeMove { no: "306", name: "とびかかる", mtype: Type::Bug, power: 60, energy: 45, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 100.0 },
    DUMMY_CM, //ChargeMove { no: "307", name: "ブレイククロー", mtype: Type::Normal, gym_power: , gym_energy: , gym_time: 1.9, power: , energy: , buff: None, buff_prob: 0.0 },
    ChargeMove { no: "308", name: "オクタンほう", mtype: Type::Water, power: 50, energy: 50, buff: Some(Buff(0, 0, -2, 0)), buff_prob: 50.0 },
    ChargeMove { no: "309", name: "ミラーショット", mtype: Type::Steel, power: 35, energy: 35, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 30.0 },

    ChargeMove { no: "310", name: "ばかぢから", mtype: Type::Fighting, power: 85, energy: 40, buff: Some(Buff(-1, -1, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "311", name: "とどめばり", mtype: Type::Bug, power: 20, energy: 35, buff: Some(Buff(1, 0, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "312", name: "グラスミキサー", mtype: Type::Grass, power: 45, energy: 40, buff: Some(Buff(0, 0, -2, 0)), buff_prob: 50.0 },
    DUMMY_CM, //ChargeMove { no: "313", name: "きゅうけつ", mtype: Type::Bug, gym_power: , gym_energy: , gym_time: 2.5, power: , energy: , buff: None, buff_prob: 0.0 },
    ChargeMove { no: "314", name: "ドレインパンチ", mtype: Type::Fighting, power: 20, energy: 40, buff: Some(Buff(0, 1, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "315", name: "シャドーボーン", mtype: Type::Ghost, power: 75, energy: 45, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 20.0 },
    ChargeMove { no: "316", name: "だくりゅう", mtype: Type::Water, power: 35, energy: 35, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 30.0 },
    ChargeMove { no: "317", name: "ブレイズキック", mtype: Type::Flare, power: 55, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "318", name: "シェルブレード", mtype: Type::Water, power: 35, energy: 35, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 50.0 },
    ChargeMove { no: "319", name: "グロウパンチ", mtype: Type::Fighting, power: 20, energy: 35, buff: Some(Buff(1, 0, 0, 0)), buff_prob: 100.0 },

    DUMMY_CM,
    ChargeMove { no: "321", name: "ギガインパクト", mtype: Type::Normal, power: 150, energy: 80, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "322", name: "やつあたり", mtype: Type::Normal, power: 10, energy: 70, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "323", name: "おんがえし", mtype: Type::Normal, power: 130, energy: 70, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "324", name: "シンクロノイズ", mtype: Type::Psychic, power: 80, energy: 50, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    DUMMY_CM,
    DUMMY_CM,
    DUMMY_CM, //ChargeMove { no: "328", name: "つのドリル", mtype: Type::Normal, gym_power: , gym_energy: , gym_time: 1.9, power: , energy: , buff: None, buff_prob: 0.0 },
    DUMMY_CM, //ChargeMove { no: "329", name: "じわれ", mtype: Type::Ground, gym_power: , gym_energy: , gym_time: 2.8, power: , energy: , buff: None, buff_prob: 0.0 },

    ChargeMove { no: "330", name: "せいなるつるぎ", mtype: Type::Fighting, power: 60, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "331", name: "フライングプレス", mtype: Type::Fighting, power: 90, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "332", name: "はどうだん", mtype: Type::Fighting, power: 100, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "333", name: "しっぺがえし", mtype: Type::Dark, power: 110, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "334", name: "がんせきほう", mtype: Type::Rock, power: 110, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "335", name: "エアロブラスト", mtype: Type::Flying, power: 170, energy: 75, buff: Some(Buff(2, 0, 0, 0)), buff_prob: 12.5 },
    ChargeMove { no: "336", name: "テクノバスター", mtype: Type::Normal, power: 120, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "337", name: "テクノバスター", mtype: Type::Flare, power: 120, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "338", name: "テクノバスター", mtype: Type::Ice, power: 120, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "339", name: "テクノバスター", mtype: Type::Water, power: 120, energy: 55, buff: None, buff_prob: 0.0 },

    ChargeMove { no: "340", name: "テクノバスター", mtype: Type::Electric, power: 120, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "341", name: "そらをとぶ", mtype: Type::Flying, power: 80, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "342", name: "Vジェネレート", mtype: Type::Flare, power: 95, energy: 40, buff: Some(Buff(0, -3, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "343", name: "リーフストーム", mtype: Type::Grass, power: 130, energy: 55, buff: Some(Buff(-2, 0, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "344", name: "トライアタック", mtype: Type::Normal, power: 65, energy: 50, buff: Some(Buff(0, 0, -1, -1)), buff_prob: 50.0 },
    DUMMY_CM,
    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "348", name: "フェザーダンス", mtype: Type::Flying, power: 35, energy: 50, buff: Some(Buff(0, 0, -2, 0)), buff_prob: 100.0 },
    DUMMY_CM,

    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "352", name: "ウェザーボール", mtype: Type::Normal, power: 55, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "353", name: "サイコファング", mtype: Type::Psychic, power: 40, energy: 35, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 100.0 },
    DUMMY_CM,
    DUMMY_CM,
    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "358", name: "せいなるほのお", mtype: Type::Flare, power: 130, energy: 65, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 50.0 },
    ChargeMove { no: "359", name: "つららばり", mtype: Type::Ice, power: 65, energy: 40, buff: None, buff_prob: 0.0 },

    ChargeMove { no: "360", name: "エアロブラスト+", mtype: Type::Flying, power: 170, energy: 75, buff: Some(Buff(2, 0, 0, 0)), buff_prob: 12.5 },
    ChargeMove { no: "361", name: "エアロブラスト++", mtype: Type::Flying, power: 170, energy: 75, buff: Some(Buff(2, 0, 0, 0)), buff_prob: 12.5 },
    ChargeMove { no: "362", name: "せいなるほのお+", mtype: Type::Flare, power: 130, energy: 65, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 50.0 },
    ChargeMove { no: "363", name: "せいなるほのお++", mtype: Type::Flare, power: 130, energy: 65, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 50.0 },
    ChargeMove { no: "364", name: "アクロバット", mtype: Type::Flying, power: 110, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "365", name: "ラスターパージ", mtype: Type::Psychic, power: 120, energy: 60, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 50.0 },
    ChargeMove { no: "366", name: "ミストボール", mtype: Type::Psychic, power: 120, energy: 60, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 50.0 },
    ChargeMove { no: "367", name: "ぶんまわす", mtype: Type::Dark, power: 65, energy: 40, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "369", name: "シードフレア", mtype: Type::Grass, power: 130, energy: 75, buff: Some(Buff(0, 0, 0, -2)), buff_prob: 40.0 },

    ChargeMove { no: "370", name: "ブロッキング", mtype: Type::Dark, power: 15, energy: 40, buff: Some(Buff(0, 1, 0, -1)), buff_prob: 100.0 },
    ChargeMove { no: "371", name: "シャドーダイブ", mtype: Type::Ghost, power: 120, energy: 90, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "372", name: "メテオビーム", mtype: Type::Rock, power: 120, energy: 60, buff: Some(Buff(1, 0, 0, 0)), buff_prob: 100.0 },
    DUMMY_CM,
    ChargeMove { no: "374", name: "クロスサンダー", mtype: Type::Electric, power: 90, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "375", name: "クロスフレイム", mtype: Type::Flare, power: 90, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "376", name: "ポルターガイスト", mtype: Type::Ghost, power: 150, energy: 75, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "377", name: "10まんばりき", mtype: Type::Ground, power: 100, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "378", name: "こごえるせかい", mtype: Type::Ice, power: 60, energy: 40, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 100.0 },
    ChargeMove { no: "379", name: "ワイドブレイカー", mtype: Type::Dragon, power: 50, energy: 35, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 100.0 },

    ChargeMove { no: "380", name: "ばくおんぱ", mtype: Type::Normal, power: 150, energy: 70, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "381", name: "ダブルパンツァー", mtype: Type::Steel, power: 50, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "382", name: "マジカルフレイム", mtype: Type::Flare, power: 60, energy: 45, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 100.0 },
    ChargeMove { no: "383", name: "アクアブレイク", mtype: Type::Water, power: 70, energy: 45, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 30.0 },
    ChargeMove { no: "384", name: "ガリョウテンセイ", mtype: Type::Flying, power: 150, energy: 70, buff: Some(Buff(0, -1, 0, 0)), buff_prob: 100.0 },
    DUMMY_CM,
    ChargeMove { no: "386", name: "マグマストーム", mtype: Type::Flare, power: 65, energy: 40, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "389", name: "デスウイング", mtype: Type::Flying, power: 85, energy: 50, buff: None, buff_prob: 0.0 },

    DUMMY_CM,
    ChargeMove { no: "391", name: "トリプルアクセル", mtype: Type::Ice, power: 60, energy: 45, buff: Some(Buff(1, 0, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "392", name: "くさわけ", mtype: Type::Grass, power: 65, energy: 50, buff: Some(Buff(1, 0, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "393", name: "ねっさのだいち", mtype: Type::Ground, power: 80, energy: 50, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 30.0 },
];
