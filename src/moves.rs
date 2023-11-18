//! ノーマル技やスペシャル技を保持する。

use std::collections::HashMap;

/// 技の属性
#[derive(Debug, Copy, Clone)]
pub enum MoveType {
    Normal,    // ノーマル
    Flare,     // ほのお
    Water,     // みず
    Electric,  // でんき
    Grass,     // くさ
    Ice,       // こおり
    Fighting,  // かくとう
    Poison,    // どく
    Ground,    // じめん
    Flying,    // ひこう
    Psychic,   // エスパー
    Bug,       // むし
    Rock,      // いわ
    Ghost,     // ゴースト
    Dragon,    // ドラゴン
    Dark,      // あく
    Steel,     // はがね
    Fairy,     // フェアリー
}

// ノーマル技構造体
#[derive(Debug)]
pub struct FastMove {
    pub no: &'static str,
    pub name: &'static str,
    pub mtype: MoveType,

    // ジム・レイドバトル
    pub gym_power: i32,
    pub gym_energy: i32,  // エネルギー充填
    pub gym_time: f32,

    // トレーナーバトル
    pub tb_power: i32,
    pub tb_energy: i32,  // エネルギー充填
    pub tb_turns: i32,
}

pub const NUM_FAST_MOVES: usize = 77;

/// ステータス変化構造体。
/// 値の意味は(自分の攻撃, 自分の防御, 相手の攻撃, 相手の防御)
/// 例えば相手の攻撃が-1になるならChangeStatus(0, 0, -1, 0)
#[derive(Debug)]
pub struct ChangeStatus(i8, i8, i8, i8);

/// スペシャル技構造体
#[derive(Debug)]
pub struct ChargeMove {
    pub no: &'static str,
    pub name: &'static str,
    pub mtype: MoveType,

    // ジム・レイドバトル
    pub gym_power: i32,
    pub gym_energy: i32,  // エネルギー消費
    pub gym_time: f32,

    // トレーナーバトル
    pub tb_power: i32,
    pub tb_energy: i32,  // エネルギー消費
    pub tb_status: Option<ChangeStatus>,  // ステータス変化
    pub tb_prob_st: f32,  // ステータス変化確率
}

pub const NUM_CHARGE_MOVES: usize = 201;

/// 技Noからノーマル技を取得するマップを返す。
pub fn get_fast_move_map_by_no() -> HashMap<String, &'static FastMove> {
    let mut m = HashMap::with_capacity(NUM_FAST_MOVES);

    for mv in &FAST_MOVES {
        m.insert(mv.no.to_string(), mv);
    }

    m
}

#[test]
fn test_get_fast_move_map_by_no() {
    let m = get_fast_move_map_by_no();
    assert_eq!(m["216"].name, "マッドショット");
}

/// 技名からノーマル技を取得するマップを返す。
pub fn get_fast_move_map_by_name() -> HashMap<String, &'static FastMove> {
    let mut m = HashMap::with_capacity(NUM_FAST_MOVES);

    for mv in &FAST_MOVES {
        m.insert(mv.name.to_string(), mv);
    }

    m
}

#[test]
fn test_get_fast_move_map_by_name() {
    let m = get_fast_move_map_by_name();
    assert_eq!(m["マッドショット"].no, "216");
}

/// 技Noからスペシャル技を取得するマップを返す。
pub fn get_charge_move_map_by_no() -> HashMap<String, &'static ChargeMove> {
    let mut m = HashMap::with_capacity(NUM_CHARGE_MOVES);

    for mv in &CHARGE_MOVES {
        m.insert(mv.no.to_string(), mv);
    }

    m
}

#[test]
fn test_get_charge_move_map_by_no() {
    let m = get_charge_move_map_by_no();
    assert_eq!(m["090"].name, "ヘドロばくだん");
}

/// 技名からスペシャル技を取得するマップを返す。
pub fn get_charge_move_map_by_name() -> HashMap<String, &'static ChargeMove> {
    let mut m = HashMap::with_capacity(NUM_CHARGE_MOVES);

    for mv in &CHARGE_MOVES {
        m.insert(mv.name.to_string(), mv);
    }

    m
}

#[test]
fn test_get_charge_move_map_by_name() {
    let m = get_charge_move_map_by_name();
    assert_eq!(m["ヘドロばくだん"].no, "090");
}

/// ノーマル技一覧
pub static FAST_MOVES: [FastMove; NUM_FAST_MOVES] = [
    FastMove { no: "200", name: "れんぞくぎり", mtype: MoveType::Bug, gym_power: 3, gym_energy: 6, gym_time: 0.4, tb_power: 2, tb_energy: 4, tb_turns: 1 },
    FastMove { no: "201", name: "むしくい", mtype: MoveType::Bug, gym_power: 5, gym_energy: 6, gym_time: 0.5, tb_power: 3, tb_energy: 3, tb_turns: 1 },
    FastMove { no: "202", name: "かみつく", mtype: MoveType::Dark, gym_power: 6, gym_energy: 4, gym_time: 0.5, tb_power: 4, tb_energy: 2, tb_turns: 1 },
    FastMove { no: "203", name: "ふいうち", mtype: MoveType::Dark, gym_power: 7, gym_energy: 8, gym_time: 0.7, tb_power: 5, tb_energy: 7, tb_turns: 2 },
    FastMove { no: "204", name: "りゅうのいぶき", mtype: MoveType::Dragon, gym_power: 6, gym_energy: 4, gym_time: 0.5, tb_power: 4, tb_energy: 3, tb_turns: 1 },
    FastMove { no: "205", name: "でんきショック", mtype: MoveType::Electric, gym_power: 5, gym_energy: 8, gym_time: 0.6, tb_power: 3, tb_energy: 9, tb_turns: 2 },
    FastMove { no: "206", name: "スパーク", mtype: MoveType::Electric, gym_power: 6, gym_energy: 9, gym_time: 0.7, tb_power: 6, tb_energy: 7, tb_turns: 2 },
    FastMove { no: "207", name: "けたぐり", mtype: MoveType::Fighting, gym_power: 6, gym_energy: 6, gym_time: 0.6, tb_power: 4, tb_energy: 5, tb_turns: 2 },
    FastMove { no: "208", name: "からてチョップ", mtype: MoveType::Fighting, gym_power: 8, gym_energy: 10, gym_time: 0.8, tb_power: 5, tb_energy: 8, tb_turns: 2 },
    FastMove { no: "209", name: "ひのこ", mtype: MoveType::Flare, gym_power: 10, gym_energy: 10, gym_time: 1.0, tb_power: 7, tb_energy: 6, tb_turns: 2 },
    FastMove { no: "210", name: "つばさでうつ", mtype: MoveType::Flying, gym_power: 8, gym_energy: 9, gym_time: 0.8, tb_power: 5, tb_energy: 8, tb_turns: 2 },
    FastMove { no: "211", name: "つつく", mtype: MoveType::Flying, gym_power: 10, gym_energy: 10, gym_time: 1.0, tb_power: 6, tb_energy: 5, tb_turns: 2 },
    FastMove { no: "212", name: "したでなめる", mtype: MoveType::Ghost, gym_power: 5, gym_energy: 6, gym_time: 0.5, tb_power: 3, tb_energy: 3, tb_turns: 1 },
    FastMove { no: "213", name: "シャドークロー", mtype: MoveType::Ghost, gym_power: 9, gym_energy: 6, gym_time: 0.7, tb_power: 6, tb_energy: 8, tb_turns: 2 },
    FastMove { no: "214", name: "つるのムチ", mtype: MoveType::Grass, gym_power: 7, gym_energy: 6, gym_time: 0.6, tb_power: 5, tb_energy: 8, tb_turns: 2 },
    FastMove { no: "215", name: "はっぱカッター", mtype: MoveType::Grass, gym_power: 13, gym_energy: 7, gym_time: 1.0, tb_power: 10, tb_energy: 4, tb_turns: 2 },
    FastMove { no: "216", name: "マッドショット", mtype: MoveType::Ground, gym_power: 5, gym_energy: 7, gym_time: 0.6, tb_power: 3, tb_energy: 9, tb_turns: 2 },
    FastMove { no: "217", name: "こおりのつぶて", mtype: MoveType::Ice, gym_power: 12, gym_energy: 12, gym_time: 1.2, tb_power: 9, tb_energy: 10, tb_turns: 3 },
    FastMove { no: "218", name: "こおりのいぶき", mtype: MoveType::Ice, gym_power: 10, gym_energy: 8, gym_time: 0.9, tb_power: 7, tb_energy: 5, tb_turns: 2 },
    FastMove { no: "219", name: "でんこうせっか", mtype: MoveType::Normal, gym_power: 8, gym_energy: 10, gym_time: 0.8, tb_power: 5, tb_energy: 8, tb_turns: 2 },
    FastMove { no: "220", name: "ひっかく", mtype: MoveType::Normal, gym_power: 6, gym_energy: 4, gym_time: 0.5, tb_power: 4, tb_energy: 2, tb_turns: 1 },
    FastMove { no: "221", name: "たいあたり", mtype: MoveType::Normal, gym_power: 5, gym_energy: 5, gym_time: 0.5, tb_power: 3, tb_energy: 3, tb_turns: 1 },
    FastMove { no: "222", name: "はたく", mtype: MoveType::Normal, gym_power: 7, gym_energy: 6, gym_time: 0.6, tb_power: 4, tb_energy: 4, tb_turns: 2 },
    FastMove { no: "223", name: "いあいぎり", mtype: MoveType::Normal, gym_power: 5, gym_energy: 5, gym_time: 0.5, tb_power: 3, tb_energy: 2, tb_turns: 1 },
    FastMove { no: "224", name: "どくづき", mtype: MoveType::Poison, gym_power: 10, gym_energy: 7, gym_time: 0.8, tb_power: 7, tb_energy: 7, tb_turns: 2 },
    FastMove { no: "225", name: "ようかいえき", mtype: MoveType::Poison, gym_power: 9, gym_energy: 8, gym_time: 0.8, tb_power: 6, tb_energy: 5, tb_turns: 2 },
    FastMove { no: "226", name: "サイコカッター", mtype: MoveType::Psychic, gym_power: 5, gym_energy: 8, gym_time: 0.6, tb_power: 3, tb_energy: 9, tb_turns: 2 },
    FastMove { no: "227", name: "いわおとし", mtype: MoveType::Rock, gym_power: 12, gym_energy: 7, gym_time: 0.9, tb_power: 8, tb_energy: 5, tb_turns: 2 },
    FastMove { no: "228", name: "メタルクロー", mtype: MoveType::Steel, gym_power: 8, gym_energy: 7, gym_time: 0.7, tb_power: 5, tb_energy: 6, tb_turns: 2 },
    FastMove { no: "229", name: "バレットパンチ", mtype: MoveType::Steel, gym_power: 9, gym_energy: 10, gym_time: 0.9, tb_power: 6, tb_energy: 7, tb_turns: 2 },
    FastMove { no: "230", name: "みずでっぽう", mtype: MoveType::Water, gym_power: 5, gym_energy: 5, gym_time: 0.5, tb_power: 3, tb_energy: 3, tb_turns: 1 },
    FastMove { no: "231", name: "はねる", mtype: MoveType::Water, gym_power: 0, gym_energy: 20, gym_time: 1.73, tb_power: 0, tb_energy: 12, tb_turns: 4 },
    FastMove { no: "232", name: "みずでっぽう(カメックス)", mtype: MoveType::Water, gym_power: 10, gym_energy: 6, gym_time: 1.0, tb_power: 6, tb_energy: 4, tb_turns: 2 },
    FastMove { no: "233", name: "どろかけ", mtype: MoveType::Ground, gym_power: 18, gym_energy: 12, gym_time: 1.4, tb_power: 11, tb_energy: 8, tb_turns: 3 },
    FastMove { no: "234", name: "しねんのずつき", mtype: MoveType::Psychic, gym_power: 12, gym_energy: 10, gym_time: 1.1, tb_power: 8, tb_energy: 6, tb_turns: 3 },
    FastMove { no: "235", name: "ねんりき", mtype: MoveType::Psychic, gym_power: 20, gym_energy: 15, gym_time: 1.6, tb_power: 16, tb_energy: 12, tb_turns: 4 },
    FastMove { no: "236", name: "どくばり", mtype: MoveType::Poison, gym_power: 5, gym_energy: 7, gym_time: 0.6, tb_power: 3, tb_energy: 9, tb_turns: 2 },
    FastMove { no: "237", name: "あわ", mtype: MoveType::Water, gym_power: 12, gym_energy: 14, gym_time: 1.2, tb_power: 7, tb_energy: 11, tb_turns: 3 },
    FastMove { no: "238", name: "だましうち", mtype: MoveType::Dark, gym_power: 10, gym_energy: 9, gym_time: 0.9, tb_power: 6, tb_energy: 6, tb_turns: 2 },
    FastMove { no: "239", name: "はがねのつばさ", mtype: MoveType::Steel, gym_power: 11, gym_energy: 6, gym_time: 0.8, tb_power: 7, tb_energy: 5, tb_turns: 2 },
    FastMove { no: "240", name: "ほのおのキバ", mtype: MoveType::Flare, gym_power: 12, gym_energy: 8, gym_time: 0.9, tb_power: 8, tb_energy: 5, tb_turns: 2 },
    FastMove { no: "241", name: "いわくだき", mtype: MoveType::Fighting, gym_power: 15, gym_energy: 10, gym_time: 1.3, tb_power: 9, tb_energy: 7, tb_turns: 3 },
    FastMove { no: "242", name: "へんしん", mtype: MoveType::Normal, gym_power: 0, gym_energy: 0, gym_time: 2.23, tb_power: 0, tb_energy: 0, tb_turns: 3 },
    FastMove { no: "243", name: "カウンター", mtype: MoveType::Fighting, gym_power: 12, gym_energy: 8, gym_time: 0.9, tb_power: 8, tb_energy: 7, tb_turns: 2 },
    FastMove { no: "244", name: "こなゆき", mtype: MoveType::Ice, gym_power: 6, gym_energy: 15, gym_time: 1.0, tb_power: 5, tb_energy: 8, tb_turns: 2 },
    FastMove { no: "249", name: "チャージビーム", mtype: MoveType::Electric, gym_power: 8, gym_energy: 15, gym_time: 1.1, tb_power: 5, tb_energy: 11, tb_turns: 3 },
    FastMove { no: "250", name: "ボルトチェンジ", mtype: MoveType::Electric, gym_power: 14, gym_energy: 21, gym_time: 1.6, tb_power: 12, tb_energy: 16, tb_turns: 4 },
    FastMove { no: "253", name: "ドラゴンテール", mtype: MoveType::Dragon, gym_power: 15, gym_energy: 9, gym_time: 1.1, tb_power: 13, tb_energy: 9, tb_turns: 3 },
    FastMove { no: "255", name: "エアスラッシュ", mtype: MoveType::Flying, gym_power: 14, gym_energy: 10, gym_time: 1.2, tb_power: 9, tb_energy: 9, tb_turns: 3 },
    FastMove { no: "260", name: "まとわりつく", mtype: MoveType::Bug, gym_power: 10, gym_energy: 14, gym_time: 1.1, tb_power: 6, tb_energy: 12, tb_turns: 3 },
    FastMove { no: "261", name: "むしのていこう", mtype: MoveType::Bug, gym_power: 15, gym_energy: 15, gym_time: 1.5, tb_power: 9, tb_energy: 8, tb_turns: 3 },
    FastMove { no: "263", name: "おどろかす", mtype: MoveType::Ghost, gym_power: 8, gym_energy: 14, gym_time: 1.1, tb_power: 5, tb_energy: 10, tb_turns: 3 },
    FastMove { no: "264", name: "たたりめ", mtype: MoveType::Ghost, gym_power: 10, gym_energy: 16, gym_time: 1.2, tb_power: 6, tb_energy: 12, tb_turns: 3 },
    FastMove { no: "266", name: "アイアンテール", mtype: MoveType::Steel, gym_power: 15, gym_energy: 7, gym_time: 1.1, tb_power: 9, tb_energy: 6, tb_turns: 3 },
    FastMove { no: "269", name: "ほのおのうず", mtype: MoveType::Flare, gym_power: 14, gym_energy: 10, gym_time: 1.1, tb_power: 9, tb_energy: 10, tb_turns: 3 },
    FastMove { no: "271", name: "タネマシンガン", mtype: MoveType::Grass, gym_power: 8, gym_energy: 14, gym_time: 1.1, tb_power: 5, tb_energy: 13, tb_turns: 3 },
    FastMove { no: "274", name: "じんつうりき", mtype: MoveType::Psychic, gym_power: 12, gym_energy: 12, gym_time: 1.1, tb_power: 8, tb_energy: 10, tb_turns: 3 },
    FastMove { no: "278", name: "バークアウト", mtype: MoveType::Dark, gym_power: 12, gym_energy: 14, gym_time: 1.1, tb_power: 5, tb_energy: 13, tb_turns: 3 },
    FastMove { no: "281", name: "めざめるパワー※", mtype: MoveType::Normal, gym_power: 15, gym_energy: 15, gym_time: 1.5, tb_power: 9, tb_energy: 8, tb_turns: 3 },
    FastMove { no: "282", name: "とっしん", mtype: MoveType::Normal, gym_power: 8, gym_energy: 10, gym_time: 1.2, tb_power: 5, tb_energy: 8, tb_turns: 3 },
    FastMove { no: "283", name: "たきのぼり", mtype: MoveType::Water, gym_power: 16, gym_energy: 8, gym_time: 1.2, tb_power: 12, tb_energy: 8, tb_turns: 3 },
    FastMove { no: "287", name: "あくび", mtype: MoveType::Normal, gym_power: 0, gym_energy: 15, gym_time: 1.7, tb_power: 0, tb_energy: 12, tb_turns: 4 },
    FastMove { no: "291", name: "プレゼント", mtype: MoveType::Normal, gym_power: 5, gym_energy: 20, gym_time: 1.3, tb_power: 3, tb_energy: 12, tb_turns: 3 },
    FastMove { no: "297", name: "うちおとす", mtype: MoveType::Rock, gym_power: 16, gym_energy: 8, gym_time: 1.2, tb_power: 12, tb_energy: 8, tb_turns: 3 },
    FastMove { no: "320", name: "あまえる", mtype: MoveType::Fairy, gym_power: 20, gym_energy: 11, gym_time: 1.5, tb_power: 15, tb_energy: 6, tb_turns: 3 },
    FastMove { no: "325", name: "ロックオン", mtype: MoveType::Normal, gym_power: 1, gym_energy: 6, gym_time: 0.3, tb_power: 1, tb_energy: 5, tb_turns: 1 },
    FastMove { no: "326", name: "かみなりのキバ", mtype: MoveType::Electric, gym_power: 12, gym_energy: 16, gym_time: 1.2, tb_power: 8, tb_energy: 5, tb_turns: 2 },
    FastMove { no: "327", name: "こおりのキバ", mtype: MoveType::Ice, gym_power: 12, gym_energy: 20, gym_time: 1.5, tb_power: 8, tb_energy: 5, tb_turns: 2 },
    FastMove { no: "345", name: "かぜおこし", mtype: MoveType::Flying, gym_power: 25, gym_energy: 20, gym_time: 2.0, tb_power: 16, tb_energy: 12, tb_turns: 4 },
    FastMove { no: "346", name: "やきつくす", mtype: MoveType::Flare, gym_power: 29, gym_energy: 20, gym_time: 2.3, tb_power: 15, tb_energy: 20, tb_turns: 5 },
    FastMove { no: "350", name: "ようせいのかぜ", mtype: MoveType::Fairy, gym_power: 9, gym_energy: 13, gym_time: 0.97, tb_power: 3, tb_energy: 9, tb_turns: 2 },
    FastMove { no: "356", name: "にどげり", mtype: MoveType::Fighting, gym_power: 10, gym_energy: 13, gym_time: 1.0, tb_power: 8, tb_energy: 12, tb_turns: 3 },
    FastMove { no: "357", name: "マジカルリーフ", mtype: MoveType::Grass, gym_power: 16, gym_energy: 16, gym_time: 1.4, tb_power: 10, tb_energy: 10, tb_turns: 3 },
    FastMove { no: "368", name: "ころがる", mtype: MoveType::Rock, gym_power: 14, gym_energy: 18, gym_time: 1.4, tb_power: 5, tb_energy: 13, tb_turns: 3 },
    FastMove { no: "373", name: "みずしゅりけん", mtype: MoveType::Water, gym_power: 10, gym_energy: 15, gym_time: 1.1, tb_power: 6, tb_energy: 14, tb_turns: 3 },
    FastMove { no: "385", name: "このは", mtype: MoveType::Grass, gym_power: 9, gym_energy: 6, gym_time: 0.7, tb_power: 6, tb_energy: 7, tb_turns: 2 },
    FastMove { no: "387", name: "ジオコントロール", mtype: MoveType::Fairy, gym_power: 20, gym_energy: 14, gym_time: 1.5, tb_power: 4, tb_energy: 13, tb_turns: 3 },
];

/// スペシャル技一覧
pub static CHARGE_MOVES: [ChargeMove; NUM_CHARGE_MOVES] = [
    ChargeMove { no: "013", name: "まきつく", mtype: MoveType::Normal, gym_power: 60, gym_energy: 33, gym_time: 2.9, tb_power: 60, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "014", name: "はかいこうせん", mtype: MoveType::Normal, gym_power: 150, gym_energy: 100, gym_time: 3.8, tb_power: 150, tb_energy: 80, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "016", name: "あくのはどう", mtype: MoveType::Dark, gym_power: 80, gym_energy: 50, gym_time: 3.0, tb_power: 80, tb_energy: 50, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "018", name: "ヘドロこうげき", mtype: MoveType::Poison, gym_power: 50, gym_energy: 33, gym_time: 2.1, tb_power: 50, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "020", name: "はさむ", mtype: MoveType::Normal, gym_power: 35, gym_energy: 33, gym_time: 1.9, tb_power: 40, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "021", name: "かえんぐるま", mtype: MoveType::Flare, gym_power: 60, gym_energy: 50, gym_time: 2.7, tb_power: 60, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "022", name: "メガホーン", mtype: MoveType::Bug, gym_power: 110, gym_energy: 100, gym_time: 2.2, tb_power: 110, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "024", name: "かえんほうしゃ", mtype: MoveType::Flare, gym_power: 70, gym_energy: 50, gym_time: 2.2, tb_power: 90, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "026", name: "あなをほる", mtype: MoveType::Ground, gym_power: 100, gym_energy: 50, gym_time: 4.7, tb_power: 80, tb_energy: 50, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "028", name: "クロスチョップ", mtype: MoveType::Fighting, gym_power: 50, gym_energy: 50, gym_time: 1.5, tb_power: 50, tb_energy: 35, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "030", name: "サイケこうせん", mtype: MoveType::Psychic, gym_power: 70, gym_energy: 50, gym_time: 3.2, tb_power: 70, tb_energy: 60, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "031", name: "じしん", mtype: MoveType::Ground, gym_power: 140, gym_energy: 100, gym_time: 3.6, tb_power: 110, tb_energy: 65, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "032", name: "ストーンエッジ", mtype: MoveType::Rock, gym_power: 100, gym_energy: 100, gym_time: 2.3, tb_power: 100, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "033", name: "れいとうパンチ", mtype: MoveType::Ice, gym_power: 50, gym_energy: 33, gym_time: 1.9, tb_power: 55, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "034", name: "ハートスタンプ", mtype: MoveType::Psychic, gym_power: 40, gym_energy: 33, gym_time: 1.9, tb_power: 40, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "035", name: "ほうでん", mtype: MoveType::Electric, gym_power: 65, gym_energy: 33, gym_time: 2.5, tb_power: 65, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "036", name: "ラスターカノン", mtype: MoveType::Steel, gym_power: 100, gym_energy: 100, gym_time: 2.7, tb_power: 110, tb_energy: 70, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "038", name: "ドリルくちばし", mtype: MoveType::Flying, gym_power: 65, gym_energy: 33, gym_time: 2.3, tb_power: 65, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "039", name: "れいとうビーム", mtype: MoveType::Ice, gym_power: 90, gym_energy: 50, gym_time: 3.3, tb_power: 90, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "040", name: "ふぶき", mtype: MoveType::Ice, gym_power: 130, gym_energy: 100, gym_time: 3.1, tb_power: 140, tb_energy: 75, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "042", name: "ねっぷう", mtype: MoveType::Flare, gym_power: 95, gym_energy: 100, gym_time: 3.0, tb_power: 95, tb_energy: 75, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "045", name: "つばめがえし", mtype: MoveType::Flying, gym_power: 55, gym_energy: 33, gym_time: 2.4, tb_power: 55, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "046", name: "ドリルライナー", mtype: MoveType::Ground, gym_power: 80, gym_energy: 50, gym_time: 2.8, tb_power: 80, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "047", name: "はなふぶき", mtype: MoveType::Grass, gym_power: 110, gym_energy: 100, gym_time: 2.6, tb_power: 110, tb_energy: 65, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "048", name: "メガドレイン", mtype: MoveType::Grass, gym_power: 25, gym_energy: 50, gym_time: 2.6, tb_power: 25, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "049", name: "むしのさざめき", mtype: MoveType::Bug, gym_power: 100, gym_energy: 50, gym_time: 3.7, tb_power: 100, tb_energy: 60, tb_status: Some(ChangeStatus(0, 0, 0, -1)), tb_prob_st: 30.0 },
    ChargeMove { no: "050", name: "どくどくのキバ", mtype: MoveType::Poison, gym_power: 35, gym_energy: 33, gym_time: 1.7, tb_power: 45, tb_energy: 40, tb_status: Some(ChangeStatus(0, 0, 0, -1)), tb_prob_st: 100.0 },
    ChargeMove { no: "051", name: "つじぎり", mtype: MoveType::Dark, gym_power: 50, gym_energy: 33, gym_time: 2.2, tb_power: 50, tb_energy: 35, tb_status: Some(ChangeStatus(2, 0, 0, 0)), tb_prob_st: 12.5 },
    ChargeMove { no: "053", name: "バブルこうせん", mtype: MoveType::Water, gym_power: 45, gym_energy: 33, gym_time: 1.9, tb_power: 25, tb_energy: 40, tb_status: Some(ChangeStatus(0, 0, -1, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "054", name: "じごくぐるま", mtype: MoveType::Fighting, gym_power: 60, gym_energy: 50, gym_time: 2.2, tb_power: 60, tb_energy: 50, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "056", name: "ローキック", mtype: MoveType::Fighting, gym_power: 40, gym_energy: 33, gym_time: 1.9, tb_power: 40, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "057", name: "アクアジェット", mtype: MoveType::Water, gym_power: 45, gym_energy: 33, gym_time: 2.6, tb_power: 45, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "058", name: "アクアテール", mtype: MoveType::Water, gym_power: 50, gym_energy: 33, gym_time: 1.9, tb_power: 50, tb_energy: 35, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "059", name: "タネばくだん", mtype: MoveType::Grass, gym_power: 55, gym_energy: 33, gym_time: 2.1, tb_power: 60, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "060", name: "サイコショック", mtype: MoveType::Psychic, gym_power: 65, gym_energy: 33, gym_time: 2.7, tb_power: 70, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "062", name: "げんしのちから", mtype: MoveType::Rock, gym_power: 70, gym_energy: 33, gym_time: 3.5, tb_power: 60, tb_energy: 45, tb_status: Some(ChangeStatus(1, 1, 0, 0)), tb_prob_st: 10.0 },
    ChargeMove { no: "063", name: "がんせきふうじ", mtype: MoveType::Rock, gym_power: 70, gym_energy: 50, gym_time: 3.2, tb_power: 70, tb_energy: 60, tb_status: Some(ChangeStatus(0, 0, -1, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "064", name: "いわなだれ", mtype: MoveType::Rock, gym_power: 80, gym_energy: 50, gym_time: 2.7, tb_power: 75, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "065", name: "パワージェム", mtype: MoveType::Rock, gym_power: 80, gym_energy: 50, gym_time: 2.9, tb_power: 80, tb_energy: 60, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "066", name: "かげうち", mtype: MoveType::Ghost, gym_power: 50, gym_energy: 33, gym_time: 2.9, tb_power: 50, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "067", name: "シャドーパンチ", mtype: MoveType::Ghost, gym_power: 40, gym_energy: 33, gym_time: 1.7, tb_power: 40, tb_energy: 35, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "069", name: "あやしいかぜ", mtype: MoveType::Ghost, gym_power: 50, gym_energy: 33, gym_time: 2.3, tb_power: 45, tb_energy: 45, tb_status: Some(ChangeStatus(1, 1, 0, 0)), tb_prob_st: 10.0 },
    ChargeMove { no: "070", name: "シャドーボール", mtype: MoveType::Ghost, gym_power: 100, gym_energy: 50, gym_time: 3.0, tb_power: 100, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "072", name: "マグネットボム", mtype: MoveType::Steel, gym_power: 70, gym_energy: 33, gym_time: 2.8, tb_power: 70, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "074", name: "アイアンヘッド", mtype: MoveType::Steel, gym_power: 60, gym_energy: 50, gym_time: 1.9, tb_power: 70, tb_energy: 50, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "075", name: "パラボラチャージ", mtype: MoveType::Electric, gym_power: 65, gym_energy: 50, gym_time: 2.8, tb_power: 65, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "077", name: "かみなりパンチ", mtype: MoveType::Electric, gym_power: 45, gym_energy: 33, gym_time: 1.8, tb_power: 55, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "078", name: "かみなり", mtype: MoveType::Electric, gym_power: 100, gym_energy: 100, gym_time: 2.4, tb_power: 100, tb_energy: 60, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "079", name: "10まんボルト", mtype: MoveType::Electric, gym_power: 80, gym_energy: 50, gym_time: 2.5, tb_power: 90, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "080", name: "たつまき", mtype: MoveType::Dragon, gym_power: 45, gym_energy: 33, gym_time: 2.8, tb_power: 45, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "082", name: "りゅうのはどう", mtype: MoveType::Dragon, gym_power: 90, gym_energy: 50, gym_time: 3.6, tb_power: 90, tb_energy: 60, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "083", name: "ドラゴンクロー", mtype: MoveType::Dragon, gym_power: 50, gym_energy: 33, gym_time: 1.7, tb_power: 50, tb_energy: 35, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "084", name: "チャームボイス", mtype: MoveType::Fairy, gym_power: 70, gym_energy: 33, gym_time: 3.9, tb_power: 70, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "085", name: "ドレインキッス", mtype: MoveType::Fairy, gym_power: 60, gym_energy: 50, gym_time: 2.6, tb_power: 60, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "086", name: "マジカルシャイン", mtype: MoveType::Fairy, gym_power: 100, gym_energy: 50, gym_time: 3.5, tb_power: 110, tb_energy: 70, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "087", name: "ムーンフォース", mtype: MoveType::Fairy, gym_power: 130, gym_energy: 100, gym_time: 3.9, tb_power: 110, tb_energy: 60, tb_status: Some(ChangeStatus(0, 0, -1, 0)), tb_prob_st: 10.0 },
    ChargeMove { no: "088", name: "じゃれつく", mtype: MoveType::Fairy, gym_power: 90, gym_energy: 50, gym_time: 2.9, tb_power: 90, tb_energy: 60, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "089", name: "クロスポイズン", mtype: MoveType::Poison, gym_power: 40, gym_energy: 33, gym_time: 1.5, tb_power: 50, tb_energy: 35, tb_status: Some(ChangeStatus(2, 0, 0, 0)), tb_prob_st: 12.5 },
    ChargeMove { no: "090", name: "ヘドロばくだん", mtype: MoveType::Poison, gym_power: 80, gym_energy: 50, gym_time: 2.3, tb_power: 80, tb_energy: 50, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "091", name: "ヘドロウェーブ", mtype: MoveType::Poison, gym_power: 110, gym_energy: 100, gym_time: 3.2, tb_power: 110, tb_energy: 65, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "092", name: "ダストシュート", mtype: MoveType::Poison, gym_power: 130, gym_energy: 100, gym_time: 3.1, tb_power: 130, tb_energy: 75, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "094", name: "ホネこんぼう", mtype: MoveType::Ground, gym_power: 40, gym_energy: 33, gym_time: 1.6, tb_power: 40, tb_energy: 35, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "095", name: "じならし", mtype: MoveType::Ground, gym_power: 80, gym_energy: 50, gym_time: 3.5, tb_power: 80, tb_energy: 60, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "096", name: "どろばくだん", mtype: MoveType::Ground, gym_power: 55, gym_energy: 33, gym_time: 2.3, tb_power: 60, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "099", name: "シグナルビーム", mtype: MoveType::Bug, gym_power: 75, gym_energy: 50, gym_time: 2.9, tb_power: 75, tb_energy: 55, tb_status: Some(ChangeStatus(0, 0, -1, -1)), tb_prob_st: 20.0 },
    ChargeMove { no: "100", name: "シザークロス", mtype: MoveType::Bug, gym_power: 45, gym_energy: 33, gym_time: 1.6, tb_power: 65, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "101", name: "ニトロチャージ", mtype: MoveType::Flare, gym_power: 70, gym_energy: 33, gym_time: 3.8, tb_power: 65, tb_energy: 50, tb_status: Some(ChangeStatus(1, 0, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "102", name: "はじけるほのお", mtype: MoveType::Flare, gym_power: 70, gym_energy: 50, gym_time: 2.6, tb_power: 70, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "103", name: "だいもんじ", mtype: MoveType::Flare, gym_power: 140, gym_energy: 100, gym_time: 4.2, tb_power: 140, tb_energy: 80, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "104", name: "しおみず", mtype: MoveType::Water, gym_power: 60, gym_energy: 50, gym_time: 2.3, tb_power: 60, tb_energy: 50, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "105", name: "みずのはどう", mtype: MoveType::Water, gym_power: 70, gym_energy: 50, gym_time: 3.2, tb_power: 70, tb_energy: 60, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "106", name: "ねっとう", mtype: MoveType::Water, gym_power: 80, gym_energy: 50, gym_time: 3.7, tb_power: 80, tb_energy: 50, tb_status: Some(ChangeStatus(0, 0, 0, -1)), tb_prob_st: 30.0 },
    ChargeMove { no: "107", name: "ハイドロポンプ", mtype: MoveType::Water, gym_power: 130, gym_energy: 100, gym_time: 3.3, tb_power: 130, tb_energy: 75, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "108", name: "サイコキネシス", mtype: MoveType::Psychic, gym_power: 90, gym_energy: 50, gym_time: 2.8, tb_power: 85, tb_energy: 55, tb_status: Some(ChangeStatus(0, 0, 0, -1)), tb_prob_st: 10.0 },
    ChargeMove { no: "109", name: "サイコブレイク", mtype: MoveType::Psychic, gym_power: 90, gym_energy: 50, gym_time: 2.3, tb_power: 90, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "111", name: "こごえるかぜ", mtype: MoveType::Ice, gym_power: 60, gym_energy: 33, gym_time: 3.3, tb_power: 60, tb_energy: 45, tb_status: Some(ChangeStatus(0, 0, -1, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "114", name: "ギガドレイン", mtype: MoveType::Grass, gym_power: 50, gym_energy: 100, gym_time: 3.9, tb_power: 50, tb_energy: 80, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "115", name: "ほのおのパンチ", mtype: MoveType::Flare, gym_power: 55, gym_energy: 33, gym_time: 2.2, tb_power: 55, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "116", name: "ソーラービーム", mtype: MoveType::Grass, gym_power: 180, gym_energy: 100, gym_time: 4.9, tb_power: 150, tb_energy: 80, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "117", name: "リーフブレード", mtype: MoveType::Grass, gym_power: 70, gym_energy: 33, gym_time: 2.4, tb_power: 70, tb_energy: 35, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "118", name: "パワーウィップ", mtype: MoveType::Grass, gym_power: 90, gym_energy: 50, gym_time: 2.6, tb_power: 90, tb_energy: 50, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "121", name: "エアカッター", mtype: MoveType::Flying, gym_power: 60, gym_energy: 50, gym_time: 2.7, tb_power: 60, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "122", name: "ぼうふう", mtype: MoveType::Flying, gym_power: 110, gym_energy: 100, gym_time: 2.7, tb_power: 110, tb_energy: 65, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "123", name: "かわらわり", mtype: MoveType::Fighting, gym_power: 40, gym_energy: 33, gym_time: 1.6, tb_power: 40, tb_energy: 35, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "125", name: "スピードスター", mtype: MoveType::Normal, gym_power: 60, gym_energy: 50, gym_time: 2.8, tb_power: 60, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "126", name: "つのでつく", mtype: MoveType::Normal, gym_power: 40, gym_energy: 33, gym_time: 1.85, tb_power: 40, tb_energy: 35, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "127", name: "ふみつけ", mtype: MoveType::Normal, gym_power: 55, gym_energy: 50, gym_time: 1.7, tb_power: 55, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "129", name: "ひっさつまえば", mtype: MoveType::Normal, gym_power: 80, gym_energy: 50, gym_time: 2.5, tb_power: 80, tb_energy: 50, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "131", name: "のしかかり", mtype: MoveType::Normal, gym_power: 50, gym_energy: 33, gym_time: 1.9, tb_power: 60, tb_energy: 35, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "132", name: "ねむる", mtype: MoveType::Normal, gym_power: 50, gym_energy: 33, gym_time: 1.9, tb_power: 50, tb_energy: 35, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "133", name: "わるあがき", mtype: MoveType::Normal, gym_power: 35, gym_energy: 0, gym_time: 2.2, tb_power: 35, tb_energy: 100, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "134", name: "ねっとう(カメックス)", mtype: MoveType::Water, gym_power: 50, gym_energy: 100, gym_time: 4.7, tb_power: 50, tb_energy: 80, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "135", name: "ハイドロポンプ(カメックス)", mtype: MoveType::Water, gym_power: 90, gym_energy: 100, gym_time: 4.5, tb_power: 90, tb_energy: 80, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "136", name: "まきつく(緑)", mtype: MoveType::Normal, gym_power: 25, gym_energy: 33, gym_time: 2.9, tb_power: 25, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "137", name: "まきつく(ピンク)", mtype: MoveType::Normal, gym_power: 25, gym_energy: 33, gym_time: 2.9, tb_power: 25, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "245", name: "インファイト", mtype: MoveType::Fighting, gym_power: 100, gym_energy: 100, gym_time: 2.3, tb_power: 100, tb_energy: 45, tb_status: Some(ChangeStatus(0, -2, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "246", name: "ばくれつパンチ", mtype: MoveType::Fighting, gym_power: 90, gym_energy: 50, gym_time: 2.7, tb_power: 90, tb_energy: 50, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "247", name: "きあいだま", mtype: MoveType::Fighting, gym_power: 140, gym_energy: 100, gym_time: 3.5, tb_power: 150, tb_energy: 75, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "248", name: "オーロラビーム", mtype: MoveType::Ice, gym_power: 80, gym_energy: 50, gym_time: 3.55, tb_power: 80, tb_energy: 60, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "251", name: "ワイルドボルト", mtype: MoveType::Electric, gym_power: 90, gym_energy: 50, gym_time: 2.6, tb_power: 100, tb_energy: 45, tb_status: Some(ChangeStatus(0, -2, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "252", name: "でんじほう", mtype: MoveType::Electric, gym_power: 140, gym_energy: 100, gym_time: 3.7, tb_power: 150, tb_energy: 80, tb_status: Some(ChangeStatus(0, 0, -1, 0)), tb_prob_st: 66.0 },
    ChargeMove { no: "254", name: "ゆきなだれ", mtype: MoveType::Ice, gym_power: 90, gym_energy: 50, gym_time: 2.7, tb_power: 90, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "256", name: "ブレイブバード", mtype: MoveType::Flying, gym_power: 130, gym_energy: 100, gym_time: 2.0, tb_power: 130, tb_energy: 55, tb_status: Some(ChangeStatus(0, -3, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "257", name: "ゴッドバード", mtype: MoveType::Flying, gym_power: 80, gym_energy: 50, gym_time: 2.0, tb_power: 75, tb_energy: 50, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "258", name: "すなじごく", mtype: MoveType::Ground, gym_power: 60, gym_energy: 33, gym_time: 4.0, tb_power: 25, tb_energy: 40, tb_status: Some(ChangeStatus(0, 0, 0, -1)), tb_prob_st: 100.0 },
    ChargeMove { no: "259", name: "ロックブラスト", mtype: MoveType::Rock, gym_power: 50, gym_energy: 33, gym_time: 2.1, tb_power: 50, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "262", name: "ぎんいろのかぜ", mtype: MoveType::Bug, gym_power: 70, gym_energy: 33, gym_time: 3.7, tb_power: 60, tb_energy: 45, tb_status: Some(ChangeStatus(1, 1, 0, 0)), tb_prob_st: 10.0 },
    ChargeMove { no: "265", name: "ナイトヘッド", mtype: MoveType::Ghost, gym_power: 60, gym_energy: 50, gym_time: 2.6, tb_power: 60, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "267", name: "ジャイロボール", mtype: MoveType::Steel, gym_power: 80, gym_energy: 50, gym_time: 3.3, tb_power: 80, tb_energy: 60, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "268", name: "ヘビーボンバー", mtype: MoveType::Steel, gym_power: 70, gym_energy: 50, gym_time: 2.1, tb_power: 70, tb_energy: 50, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "270", name: "オーバーヒート", mtype: MoveType::Flare, gym_power: 160, gym_energy: 100, gym_time: 4.0, tb_power: 130, tb_energy: 55, tb_status: Some(ChangeStatus(-2, 0, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "272", name: "くさむすび", mtype: MoveType::Grass, gym_power: 90, gym_energy: 50, gym_time: 2.6, tb_power: 90, tb_energy: 50, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "273", name: "エナジーボール", mtype: MoveType::Grass, gym_power: 90, gym_energy: 50, gym_time: 3.9, tb_power: 90, tb_energy: 55, tb_status: Some(ChangeStatus(0, 0, 0, -1)), tb_prob_st: 10.0 },
    ChargeMove { no: "275", name: "みらいよち", mtype: MoveType::Psychic, gym_power: 120, gym_energy: 100, gym_time: 2.7, tb_power: 120, tb_energy: 65, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "276", name: "ミラーコート", mtype: MoveType::Psychic, gym_power: 60, gym_energy: 50, gym_time: 2.6, tb_power: 60, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "277", name: "げきりん", mtype: MoveType::Dragon, gym_power: 110, gym_energy: 50, gym_time: 3.9, tb_power: 110, tb_energy: 60, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "279", name: "かみくだく", mtype: MoveType::Dark, gym_power: 70, gym_energy: 33, gym_time: 3.2, tb_power: 70, tb_energy: 45, tb_status: Some(ChangeStatus(0, 0, 0, -1)), tb_prob_st: 30.0 },
    ChargeMove { no: "280", name: "イカサマ", mtype: MoveType::Dark, gym_power: 70, gym_energy: 50, gym_time: 2.0, tb_power: 70, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "284", name: "なみのり", mtype: MoveType::Water, gym_power: 65, gym_energy: 50, gym_time: 1.7, tb_power: 65, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "285", name: "りゅうせいぐん", mtype: MoveType::Dragon, gym_power: 150, gym_energy: 100, gym_time: 3.6, tb_power: 150, tb_energy: 65, tb_status: Some(ChangeStatus(-2, 0, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "286", name: "はめつのねがい", mtype: MoveType::Steel, gym_power: 70, gym_energy: 33, gym_time: 1.7, tb_power: 75, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "288", name: "サイコブースト", mtype: MoveType::Psychic, gym_power: 70, gym_energy: 50, gym_time: 4.0, tb_power: 70, tb_energy: 35, tb_status: Some(ChangeStatus(-2, 0, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "289", name: "こんげんのはどう", mtype: MoveType::Water, gym_power: 130, gym_energy: 100, gym_time: 1.7, tb_power: 130, tb_energy: 60, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "290", name: "だんがいのつるぎ", mtype: MoveType::Ground, gym_power: 130, gym_energy: 100, gym_time: 1.7, tb_power: 130, tb_energy: 60, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "292", name: "ウェザーボール", mtype: MoveType::Flare, gym_power: 55, gym_energy: 33, gym_time: 1.6, tb_power: 55, tb_energy: 35, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "293", name: "ウェザーボール", mtype: MoveType::Ice, gym_power: 55, gym_energy: 33, gym_time: 1.6, tb_power: 55, tb_energy: 35, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "294", name: "ウェザーボール", mtype: MoveType::Rock, gym_power: 55, gym_energy: 33, gym_time: 1.6, tb_power: 55, tb_energy: 35, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "295", name: "ウェザーボール", mtype: MoveType::Water, gym_power: 55, gym_energy: 33, gym_time: 1.6, tb_power: 55, tb_energy: 35, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "296", name: "ハードプラント", mtype: MoveType::Grass, gym_power: 100, gym_energy: 50, gym_time: 2.6, tb_power: 100, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "298", name: "ブラストバーン", mtype: MoveType::Flare, gym_power: 110, gym_energy: 50, gym_time: 3.3, tb_power: 110, tb_energy: 50, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "299", name: "ハイドロカノン", mtype: MoveType::Water, gym_power: 90, gym_energy: 50, gym_time: 1.9, tb_power: 80, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "300", name: "とっておき", mtype: MoveType::Normal, gym_power: 90, gym_energy: 50, gym_time: 2.9, tb_power: 90, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "301", name: "コメットパンチ", mtype: MoveType::Steel, gym_power: 100, gym_energy: 50, gym_time: 2.6, tb_power: 100, tb_energy: 50, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "302", name: "ロケットずつき", mtype: MoveType::Normal, gym_power: 130, gym_energy: 100, gym_time: 3.1, tb_power: 130, tb_energy: 75, tb_status: Some(ChangeStatus(0, 1, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "303", name: "アシッドボム", mtype: MoveType::Poison, gym_power: 20, gym_energy: 50, gym_time: 3.0, tb_power: 20, tb_energy: 45, tb_status: Some(ChangeStatus(0, 0, 0, -2)), tb_prob_st: 100.0 },
    ChargeMove { no: "304", name: "だいちのちから", mtype: MoveType::Ground, gym_power: 100, gym_energy: 50, gym_time: 3.6, tb_power: 90, tb_energy: 55, tb_status: Some(ChangeStatus(0, 0, 0, -1)), tb_prob_st: 10.0 },
    ChargeMove { no: "305", name: "クラブハンマー", mtype: MoveType::Water, gym_power: 85, gym_energy: 50, gym_time: 1.9, tb_power: 85, tb_energy: 50, tb_status: Some(ChangeStatus(2, 0, 0, 0)), tb_prob_st: 12.5 },
    ChargeMove { no: "306", name: "とびかかる", mtype: MoveType::Bug, gym_power: 55, gym_energy: 33, gym_time: 2.9, tb_power: 60, tb_energy: 45, tb_status: Some(ChangeStatus(0, 0, -1, 0)), tb_prob_st: 100.0 },
    //ChargeMove { no: "307", name: "ブレイククロー", mtype: MoveType::Normal, gym_power: , gym_energy: , gym_time: 1.9, tb_power: , tb_energy: , tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "308", name: "オクタンほう", mtype: MoveType::Water, gym_power: 50, gym_energy: 50, gym_time: 2.3, tb_power: 50, tb_energy: 50, tb_status: Some(ChangeStatus(0, 0, -2, 0)), tb_prob_st: 50.0 },
    ChargeMove { no: "309", name: "ミラーショット", mtype: MoveType::Steel, gym_power: 50, gym_energy: 33, gym_time: 2.4, tb_power: 35, tb_energy: 35, tb_status: Some(ChangeStatus(0, 0, -1, 0)), tb_prob_st: 30.0 },
    ChargeMove { no: "310", name: "ばかぢから", mtype: MoveType::Fighting, gym_power: 85, gym_energy: 50, gym_time: 3.0, tb_power: 85, tb_energy: 40, tb_status: Some(ChangeStatus(-1, -1, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "311", name: "とどめばり", mtype: MoveType::Bug, gym_power: 50, gym_energy: 33, gym_time: 2.2, tb_power: 20, tb_energy: 35, tb_status: Some(ChangeStatus(1, 0, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "312", name: "グラスミキサー", mtype: MoveType::Grass, gym_power: 45, gym_energy: 33, gym_time: 3.1, tb_power: 45, tb_energy: 40, tb_status: Some(ChangeStatus(0, 0, -2, 0)), tb_prob_st: 50.0 },
    //ChargeMove { no: "313", name: "きゅうけつ", mtype: MoveType::Bug, gym_power: , gym_energy: , gym_time: 2.5, tb_power: , tb_energy: , tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "314", name: "ドレインパンチ", mtype: MoveType::Fighting, gym_power: 50, gym_energy: 33, gym_time: 2.4, tb_power: 20, tb_energy: 40, tb_status: Some(ChangeStatus(0, 1, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "315", name: "シャドーボーン", mtype: MoveType::Ghost, gym_power: 80, gym_energy: 50, gym_time: 2.8, tb_power: 75, tb_energy: 45, tb_status: Some(ChangeStatus(0, 0, 0, -1)), tb_prob_st: 20.0 },
    ChargeMove { no: "316", name: "だくりゅう", mtype: MoveType::Water, gym_power: 50, gym_energy: 33, gym_time: 2.2, tb_power: 35, tb_energy: 35, tb_status: Some(ChangeStatus(0, 0, -1, 0)), tb_prob_st: 30.0 },
    ChargeMove { no: "317", name: "ブレイズキック", mtype: MoveType::Flare, gym_power: 45, gym_energy: 33, gym_time: 1.2, tb_power: 55, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "318", name: "シェルブレード", mtype: MoveType::Water, gym_power: 45, gym_energy: 33, gym_time: 1.3, tb_power: 35, tb_energy: 35, tb_status: Some(ChangeStatus(0, 0, 0, -1)), tb_prob_st: 50.0 },
    ChargeMove { no: "319", name: "グロウパンチ", mtype: MoveType::Fighting, gym_power: 50, gym_energy: 33, gym_time: 2.0, tb_power: 20, tb_energy: 35, tb_status: Some(ChangeStatus(1, 0, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "321", name: "ギガインパクト", mtype: MoveType::Normal, gym_power: 200, gym_energy: 100, gym_time: 4.7, tb_power: 150, tb_energy: 80, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "322", name: "やつあたり", mtype: MoveType::Normal, gym_power: 10, gym_energy: 33, gym_time: 2.0, tb_power: 10, tb_energy: 70, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "323", name: "おんがえし", mtype: MoveType::Normal, gym_power: 35, gym_energy: 33, gym_time: 0.7, tb_power: 130, tb_energy: 70, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "324", name: "シンクロノイズ", mtype: MoveType::Psychic, gym_power: 80, gym_energy: 50, gym_time: 2.6, tb_power: 80, tb_energy: 50, tb_status: None, tb_prob_st: 0.0 },
    //ChargeMove { no: "328", name: "つのドリル", mtype: MoveType::Normal, gym_power: , gym_energy: , gym_time: 1.9, tb_power: , tb_energy: , tb_status: None, tb_prob_st: 0.0 },
    //ChargeMove { no: "329", name: "じわれ", mtype: MoveType::Ground, gym_power: , gym_energy: , gym_time: 2.8, tb_power: , tb_energy: , tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "330", name: "せいなるつるぎ", mtype: MoveType::Fighting, gym_power: 55, gym_energy: 33, gym_time: 1.2, tb_power: 60, tb_energy: 35, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "331", name: "フライングプレス", mtype: MoveType::Fighting, gym_power: 110, gym_energy: 50, gym_time: 2.3, tb_power: 90, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "332", name: "はどうだん", mtype: MoveType::Fighting, gym_power: 90, gym_energy: 50, gym_time: 1.8, tb_power: 100, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "333", name: "しっぺがえし", mtype: MoveType::Dark, gym_power: 100, gym_energy: 100, gym_time: 2.2, tb_power: 110, tb_energy: 60, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "334", name: "がんせきほう", mtype: MoveType::Rock, gym_power: 110, gym_energy: 50, gym_time: 3.6, tb_power: 110, tb_energy: 50, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "335", name: "エアロブラスト", mtype: MoveType::Flying, gym_power: 180, gym_energy: 100, gym_time: 3.4, tb_power: 170, tb_energy: 75, tb_status: Some(ChangeStatus(2, 0, 0, 0)), tb_prob_st: 12.5 },
    ChargeMove { no: "336", name: "テクノバスター", mtype: MoveType::Normal, gym_power: 120, gym_energy: 100, gym_time: 2.0, tb_power: 120, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "337", name: "テクノバスター", mtype: MoveType::Flare, gym_power: 120, gym_energy: 100, gym_time: 2.0, tb_power: 120, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "338", name: "テクノバスター", mtype: MoveType::Ice, gym_power: 120, gym_energy: 100, gym_time: 2.0, tb_power: 120, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "339", name: "テクノバスター", mtype: MoveType::Water, gym_power: 120, gym_energy: 100, gym_time: 2.0, tb_power: 120, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "340", name: "テクノバスター", mtype: MoveType::Electric, gym_power: 120, gym_energy: 100, gym_time: 2.0, tb_power: 120, tb_energy: 55, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "341", name: "そらをとぶ", mtype: MoveType::Flying, gym_power: 80, gym_energy: 50, gym_time: 1.8, tb_power: 80, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "342", name: "Vジェネレート", mtype: MoveType::Flare, gym_power: 95, gym_energy: 33, gym_time: 2.8, tb_power: 95, tb_energy: 40, tb_status: Some(ChangeStatus(0, -3, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "343", name: "リーフストーム", mtype: MoveType::Grass, gym_power: 130, gym_energy: 100, gym_time: 2.5, tb_power: 130, tb_energy: 55, tb_status: Some(ChangeStatus(-2, 0, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "344", name: "トライアタック", mtype: MoveType::Normal, gym_power: 75, gym_energy: 50, gym_time: 2.5, tb_power: 65, tb_energy: 50, tb_status: Some(ChangeStatus(0, 0, -1, -1)), tb_prob_st: 50.0 },
    ChargeMove { no: "348", name: "フェザーダンス", mtype: MoveType::Flying, gym_power: 35, gym_energy: 50, gym_time: 2.8, tb_power: 35, tb_energy: 50, tb_status: Some(ChangeStatus(0, 0, -2, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "352", name: "ウェザーボール", mtype: MoveType::Normal, gym_power: 55, gym_energy: 33, gym_time: 1.6, tb_power: 55, tb_energy: 35, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "353", name: "サイコファング", mtype: MoveType::Psychic, gym_power: 30, gym_energy: 33, gym_time: 1.2, tb_power: 40, tb_energy: 35, tb_status: Some(ChangeStatus(0, 0, 0, -1)), tb_prob_st: 100.0 },
    ChargeMove { no: "358", name: "せいなるほのお", mtype: MoveType::Flare, gym_power: 120, gym_energy: 100, gym_time: 2.6, tb_power: 130, tb_energy: 65, tb_status: Some(ChangeStatus(0, 0, -1, 0)), tb_prob_st: 50.0 },
    ChargeMove { no: "359", name: "つららばり", mtype: MoveType::Ice, gym_power: 60, gym_energy: 33, gym_time: 2.2, tb_power: 65, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "360", name: "エアロブラスト+", mtype: MoveType::Flying, gym_power: 200, gym_energy: 100, gym_time: 3.4, tb_power: 170, tb_energy: 75, tb_status: Some(ChangeStatus(2, 0, 0, 0)), tb_prob_st: 12.5 },
    ChargeMove { no: "361", name: "エアロブラスト++", mtype: MoveType::Flying, gym_power: 225, gym_energy: 100, gym_time: 3.4, tb_power: 170, tb_energy: 75, tb_status: Some(ChangeStatus(2, 0, 0, 0)), tb_prob_st: 12.5 },
    ChargeMove { no: "362", name: "せいなるほのお+", mtype: MoveType::Flare, gym_power: 135, gym_energy: 100, gym_time: 2.6, tb_power: 130, tb_energy: 65, tb_status: Some(ChangeStatus(0, 0, -1, 0)), tb_prob_st: 50.0 },
    ChargeMove { no: "363", name: "せいなるほのお++", mtype: MoveType::Flare, gym_power: 155, gym_energy: 100, gym_time: 2.6, tb_power: 130, tb_energy: 65, tb_status: Some(ChangeStatus(0, 0, -1, 0)), tb_prob_st: 50.0 },
    ChargeMove { no: "364", name: "アクロバット", mtype: MoveType::Flying, gym_power: 100, gym_energy: 100, gym_time: 2.0, tb_power: 110, tb_energy: 60, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "365", name: "ラスターパージ", mtype: MoveType::Psychic, gym_power: 100, gym_energy: 100, gym_time: 1.5, tb_power: 120, tb_energy: 60, tb_status: Some(ChangeStatus(0, 0, 0, -1)), tb_prob_st: 50.0 },
    ChargeMove { no: "366", name: "ミストボール", mtype: MoveType::Psychic, gym_power: 105, gym_energy: 100, gym_time: 2.0, tb_power: 120, tb_energy: 60, tb_status: Some(ChangeStatus(0, 0, -1, 0)), tb_prob_st: 50.0 },
    ChargeMove { no: "367", name: "ぶんまわす", mtype: MoveType::Dark, gym_power: 65, gym_energy: 33, gym_time: 1.9, tb_power: 65, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "369", name: "シードフレア", mtype: MoveType::Grass, gym_power: 120, gym_energy: 100, gym_time: 2.7, tb_power: 130, tb_energy: 75, tb_status: Some(ChangeStatus(0, 0, 0, -2)), tb_prob_st: 40.0 },
    ChargeMove { no: "370", name: "ブロッキング", mtype: MoveType::Dark, gym_power: 20, gym_energy: 33, gym_time: 1.7, tb_power: 15, tb_energy: 40, tb_status: Some(ChangeStatus(0, 1, 0, -1)), tb_prob_st: 100.0 },
    ChargeMove { no: "371", name: "シャドーダイブ", mtype: MoveType::Ghost, gym_power: 140, gym_energy: 100, gym_time: 1.9, tb_power: 120, tb_energy: 90, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "372", name: "メテオビーム", mtype: MoveType::Rock, gym_power: 140, gym_energy: 100, gym_time: 1.9, tb_power: 120, tb_energy: 60, tb_status: Some(ChangeStatus(1, 0, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "374", name: "クロスサンダー", mtype: MoveType::Electric, gym_power: 140, gym_energy: 100, gym_time: 2.0, tb_power: 90, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "375", name: "クロスフレイム", mtype: MoveType::Flare, gym_power: 140, gym_energy: 100, gym_time: 2.2, tb_power: 90, tb_energy: 45, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "376", name: "ポルターガイスト", mtype: MoveType::Ghost, gym_power: 140, gym_energy: 100, gym_time: 3.6, tb_power: 150, tb_energy: 75, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "377", name: "10まんばりき", mtype: MoveType::Ground, gym_power: 110, gym_energy: 100, gym_time: 1.6, tb_power: 100, tb_energy: 60, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "378", name: "こごえるせかい", mtype: MoveType::Ice, gym_power: 160, gym_energy: 100, gym_time: 2.5, tb_power: 60, tb_energy: 40, tb_status: Some(ChangeStatus(0, 0, -1, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "379", name: "ワイドブレイカー", mtype: MoveType::Dragon, gym_power: 35, gym_energy: 33, gym_time: 0.8, tb_power: 50, tb_energy: 35, tb_status: Some(ChangeStatus(0, 0, -1, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "380", name: "ばくおんぱ", mtype: MoveType::Normal, gym_power: 140, gym_energy: 100, gym_time: 2.3, tb_power: 150, tb_energy: 70, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "381", name: "ダブルパンツァー", mtype: MoveType::Steel, gym_power: 70, gym_energy: 33, gym_time: 2.0, tb_power: 50, tb_energy: 35, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "382", name: "マジカルフレイム", mtype: MoveType::Flare, gym_power: 60, gym_energy: 33, gym_time: 2.0, tb_power: 60, tb_energy: 45, tb_status: Some(ChangeStatus(0, 0, -1, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "383", name: "アクアブレイク", mtype: MoveType::Water, gym_power: 70, gym_energy: 33, gym_time: 3.0, tb_power: 70, tb_energy: 45, tb_status: Some(ChangeStatus(0, 0, 0, -1)), tb_prob_st: 30.0 },
    ChargeMove { no: "384", name: "ガリョウテンセイ", mtype: MoveType::Flying, gym_power: 140, gym_energy: 50, gym_time: 3.5, tb_power: 150, tb_energy: 70, tb_status: Some(ChangeStatus(0, -1, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "386", name: "マグマストーム", mtype: MoveType::Flare, gym_power: 75, gym_energy: 33, gym_time: 2.5, tb_power: 65, tb_energy: 40, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "389", name: "デスウイング", mtype: MoveType::Flying, gym_power: 85, gym_energy: 50, gym_time: 2.0, tb_power: 85, tb_energy: 50, tb_status: None, tb_prob_st: 0.0 },
    ChargeMove { no: "391", name: "トリプルアクセル", mtype: MoveType::Ice, gym_power: 60, gym_energy: 33, gym_time: 2.0, tb_power: 60, tb_energy: 45, tb_status: Some(ChangeStatus(1, 0, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "392", name: "くさわけ", mtype: MoveType::Grass, gym_power: 65, gym_energy: 50, gym_time: 2.0, tb_power: 65, tb_energy: 50, tb_status: Some(ChangeStatus(1, 0, 0, 0)), tb_prob_st: 100.0 },
    ChargeMove { no: "393", name: "ねっさのだいち", mtype: MoveType::Ground, gym_power: 95, gym_energy: 50, gym_time: 3.2, tb_power: 80, tb_energy: 50, tb_status: Some(ChangeStatus(0, 0, -1, 0)), tb_prob_st: 30.0 },
];
