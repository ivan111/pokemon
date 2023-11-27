//! ノーマル技やスペシャル技を保持する。

use std::collections::HashMap;
use std::sync::OnceLock;

use skim::prelude::*;

use crate::pokepedia::Pokepedia;
use crate::types::Type;
use crate::utils::NameItem;

pub const STAB: f64 = 1.2;  // STAB(Same Type Attack Bonus, タイプ一致ボーナス)

// ノーマル技構造体
#[derive(Debug)]
pub struct FastMove {
    no: &'static str,
    name: &'static str,
    s_name: &'static str,
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

    pub fn s_name(&self) -> &'static str {
        self.s_name
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

pub fn skim_fast_move() -> Option<&'static FastMove> {
    let options = SkimOptions::default();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

    for mv in &FAST_MOVES {
        if mv.no() == "000" {
            continue;
        }

        let _ = tx_item.send(Arc::new(NameItem {
            name: mv.name,
            search_text: mv.name.to_string() + mv.s_name,
        }));
    }

    drop(tx_item);

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_default();

    if selected_items.len() == 1 {
        fast_move_by_name(&selected_items[0].output())
    } else {
        None
    }
}

pub fn skim_charge_move() -> Option<&'static ChargeMove> {
    let options = SkimOptions::default();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

    for mv in &CHARGE_MOVES {
        if mv.no() == "000" {
            continue;
        }

        let _ = tx_item.send(Arc::new(NameItem {
            name: mv.name,
            search_text: mv.name.to_string() + mv.s_name,
        }));
    }

    drop(tx_item);

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_default();

    if selected_items.len() == 1 {
        charge_move_by_name(&selected_items[0].output())
    } else {
        None
    }
}

pub fn skim_fast_move_in_dict(dict: &'static Pokepedia) -> Option<&'static FastMove> {
    let options = SkimOptions::default();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

    for mv in dict.fast_moves() {
        let _ = tx_item.send(Arc::new(NameItem {
            name: mv.name,
            search_text: mv.name.to_string() + mv.s_name,
        }));
    }

    drop(tx_item);

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_default();

    if selected_items.len() == 1 {
        fast_move_by_name(&selected_items[0].output())
    } else {
        None
    }
}

pub fn skim_charge_move_in_dict(dict: &'static Pokepedia) -> Option<&'static ChargeMove> {
    let options = SkimOptions::default();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

    for mv in dict.charge_moves() {
        let _ = tx_item.send(Arc::new(NameItem {
            name: mv.name,
            search_text: mv.name.to_string() + mv.s_name,
        }));
    }

    drop(tx_item);

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_default();

    if selected_items.len() == 1 {
        charge_move_by_name(&selected_items[0].output())
    } else {
        None
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
    s_name: &'static str,
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

    pub fn s_name(&self) -> &'static str {
        self.s_name
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

    pub fn pte(&self) -> f64 {
        self.power as f64 / self.energy as f64
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

/// noとnameが一意(ユニーク)であるかをチェックする
#[test]
pub fn test_fast_move_uniq() {
    use std::collections::HashSet;

    let mut no_set = HashSet::with_capacity(NUM_FAST_MOVES);
    let mut name_set = HashSet::with_capacity(NUM_FAST_MOVES);

    for mv in &FAST_MOVES {
        if mv.no == "000" {
            continue;
        }

        if !no_set.insert(mv.no.to_string()) {
            panic!("FAST_MOVESの番号にダブりあり: {}", mv.no);
        }

        if !name_set.insert(mv.name.to_string()) {
            panic!("FAST_MOVESの名前にダブりあり: {}", mv.name);
        }
    }
}

/// noとnameが一意(ユニーク)であるかをチェックする
#[test]
pub fn test_charge_move_uniq() {
    use std::collections::HashSet;

    let mut no_set = HashSet::with_capacity(NUM_CHARGE_MOVES);
    let mut name_set = HashSet::with_capacity(NUM_CHARGE_MOVES);

    for mv in &CHARGE_MOVES {
        if mv.no == "000" {
            continue;
        }

        if !no_set.insert(mv.no.to_string()) {
            panic!("CHARGE_MOVESの番号にダブりあり: {}", mv.no);
        }

        if !name_set.insert(mv.name.to_string()) {
            panic!("CHARGE_MOVESの名前にダブりあり: {}", mv.name);
        }
    }
}

static FAST_MOVE_NO_MAP: OnceLock<HashMap<String, &'static FastMove>> = OnceLock::new();
static FAST_MOVE_NAME_MAP: OnceLock<HashMap<String, &'static FastMove>> = OnceLock::new();
static CHARGE_MOVE_NO_MAP: OnceLock<HashMap<String, &'static ChargeMove>> = OnceLock::new();
static CHARGE_MOVE_NAME_MAP: OnceLock<HashMap<String, &'static ChargeMove>> = OnceLock::new();

pub const NUM_FAST_MOVES: usize = 188;

const DUMMY_FM: FastMove = FastMove { no: "000", name: "dummy fast move", s_name: "", mtype: Type::Normal, power: 0, energy: 0, turns: 0 };

/// ノーマル技一覧
pub static FAST_MOVES: [FastMove; NUM_FAST_MOVES] = [
    FastMove { no: "200", name: "れんぞくぎり", s_name: "renzokugiri", mtype: Type::Bug, power: 2, energy: 4, turns: 1 },
    FastMove { no: "201", name: "むしくい", s_name: "musikui", mtype: Type::Bug, power: 3, energy: 3, turns: 1 },
    FastMove { no: "202", name: "かみつく", s_name: "kamituku", mtype: Type::Dark, power: 4, energy: 2, turns: 1 },
    FastMove { no: "203", name: "ふいうち", s_name: "fuiuti", mtype: Type::Dark, power: 5, energy: 7, turns: 2 },
    FastMove { no: "204", name: "りゅうのいぶき", s_name: "ryuunoibuki", mtype: Type::Dragon, power: 4, energy: 3, turns: 1 },
    FastMove { no: "205", name: "でんきショック", s_name: "denkisyokku", mtype: Type::Electric, power: 3, energy: 9, turns: 2 },
    FastMove { no: "206", name: "スパーク", s_name: "supa-ku", mtype: Type::Electric, power: 6, energy: 7, turns: 2 },
    FastMove { no: "207", name: "けたぐり", s_name: "ketaguri", mtype: Type::Fighting, power: 4, energy: 5, turns: 2 },
    FastMove { no: "208", name: "からてチョップ", s_name: "karatetyoppu", mtype: Type::Fighting, power: 5, energy: 8, turns: 2 },
    FastMove { no: "209", name: "ひのこ", s_name: "hinoko", mtype: Type::Flare, power: 7, energy: 6, turns: 2 },

    FastMove { no: "210", name: "つばさでうつ", s_name: "tubasadeutu", mtype: Type::Flying, power: 5, energy: 8, turns: 2 },
    FastMove { no: "211", name: "つつく", s_name: "tutuku", mtype: Type::Flying, power: 6, energy: 5, turns: 2 },
    FastMove { no: "212", name: "したでなめる", s_name: "sitadenameru", mtype: Type::Ghost, power: 3, energy: 3, turns: 1 },
    FastMove { no: "213", name: "シャドークロー", s_name: "syado-kuro-", mtype: Type::Ghost, power: 6, energy: 8, turns: 2 },
    FastMove { no: "214", name: "つるのムチ", s_name: "turunomuti", mtype: Type::Grass, power: 5, energy: 8, turns: 2 },
    FastMove { no: "215", name: "はっぱカッター", s_name: "happakatta-", mtype: Type::Grass, power: 10, energy: 4, turns: 2 },
    FastMove { no: "216", name: "マッドショット", s_name: "maddosyotto", mtype: Type::Ground, power: 3, energy: 9, turns: 2 },
    FastMove { no: "217", name: "こおりのつぶて", s_name: "koorinotubute", mtype: Type::Ice, power: 9, energy: 10, turns: 3 },
    FastMove { no: "218", name: "こおりのいぶき", s_name: "koorinoibuki", mtype: Type::Ice, power: 7, energy: 5, turns: 2 },
    FastMove { no: "219", name: "でんこうせっか", s_name: "denkousekka", mtype: Type::Normal, power: 5, energy: 8, turns: 2 },

    FastMove { no: "220", name: "ひっかく", s_name: "hikkaku", mtype: Type::Normal, power: 4, energy: 2, turns: 1 },
    FastMove { no: "221", name: "たいあたり", s_name: "taiatari", mtype: Type::Normal, power: 3, energy: 3, turns: 1 },
    FastMove { no: "222", name: "はたく", s_name: "hataku", mtype: Type::Normal, power: 4, energy: 4, turns: 2 },
    FastMove { no: "223", name: "いあいぎり", s_name: "iaigiri", mtype: Type::Normal, power: 3, energy: 2, turns: 1 },
    FastMove { no: "224", name: "どくづき", s_name: "dokuduki", mtype: Type::Poison, power: 7, energy: 7, turns: 2 },
    FastMove { no: "225", name: "ようかいえき", s_name: "youkaieki", mtype: Type::Poison, power: 6, energy: 5, turns: 2 },
    FastMove { no: "226", name: "サイコカッター", s_name: "saikokatta-", mtype: Type::Psychic, power: 3, energy: 9, turns: 2 },
    FastMove { no: "227", name: "いわおとし", s_name: "iwaotosi", mtype: Type::Rock, power: 8, energy: 5, turns: 2 },
    FastMove { no: "228", name: "メタルクロー", s_name: "metarukuro-", mtype: Type::Steel, power: 5, energy: 6, turns: 2 },
    FastMove { no: "229", name: "バレットパンチ", s_name: "barettopanti", mtype: Type::Steel, power: 6, energy: 7, turns: 2 },

    FastMove { no: "230", name: "みずでっぽう", s_name: "mizudeppou", mtype: Type::Water, power: 3, energy: 3, turns: 1 },
    FastMove { no: "231", name: "はねる", s_name: "haneru", mtype: Type::Water, power: 0, energy: 12, turns: 4 },
    FastMove { no: "232", name: "みずでっぽう(カメックス)", s_name: "mizudeppoukamekkusu", mtype: Type::Water, power: 6, energy: 4, turns: 2 },
    FastMove { no: "233", name: "どろかけ", s_name: "dorokake", mtype: Type::Ground, power: 11, energy: 8, turns: 3 },
    FastMove { no: "234", name: "しねんのずつき", s_name: "sinennnozutuki", mtype: Type::Psychic, power: 8, energy: 6, turns: 3 },
    FastMove { no: "235", name: "ねんりき", s_name: "nenriki", mtype: Type::Psychic, power: 16, energy: 12, turns: 4 },
    FastMove { no: "236", name: "どくばり", s_name: "dokubari", mtype: Type::Poison, power: 3, energy: 9, turns: 2 },
    FastMove { no: "237", name: "あわ", s_name: "awa", mtype: Type::Water, power: 7, energy: 11, turns: 3 },
    FastMove { no: "238", name: "だましうち", s_name: "damasiuti", mtype: Type::Dark, power: 6, energy: 6, turns: 2 },
    FastMove { no: "239", name: "はがねのつばさ", s_name: "haganenotubasa", mtype: Type::Steel, power: 7, energy: 5, turns: 2 },

    FastMove { no: "240", name: "ほのおのキバ", s_name: "honoonokiba", mtype: Type::Flare, power: 8, energy: 5, turns: 2 },
    FastMove { no: "241", name: "いわくだき", s_name: "iwakudaki", mtype: Type::Fighting, power: 9, energy: 7, turns: 3 },
    FastMove { no: "242", name: "へんしん", s_name: "hensinn", mtype: Type::Normal, power: 0, energy: 0, turns: 3 },
    FastMove { no: "243", name: "カウンター", s_name: "kaunta-", mtype: Type::Fighting, power: 8, energy: 7, turns: 2 },
    FastMove { no: "244", name: "こなゆき", s_name: "konayuki", mtype: Type::Ice, power: 5, energy: 8, turns: 2 },
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "249", name: "チャージビーム", s_name: "tya-jibi-mu", mtype: Type::Electric, power: 5, energy: 11, turns: 3 },

    FastMove { no: "250", name: "ボルトチェンジ", s_name: "borutotyenji", mtype: Type::Electric, power: 12, energy: 16, turns: 4 },
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "253", name: "ドラゴンテール", s_name: "doragonte-ru", mtype: Type::Dragon, power: 13, energy: 9, turns: 3 },
    DUMMY_FM,
    FastMove { no: "255", name: "エアスラッシュ", s_name: "easurassyu", mtype: Type::Flying, power: 9, energy: 9, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,

    FastMove { no: "260", name: "まとわりつく", s_name: "matowarituku", mtype: Type::Bug, power: 6, energy: 12, turns: 3 },
    FastMove { no: "261", name: "むしのていこう", s_name: "musinoteikou", mtype: Type::Bug, power: 9, energy: 8, turns: 3 },
    DUMMY_FM,
    FastMove { no: "263", name: "おどろかす", s_name: "odorokasu", mtype: Type::Ghost, power: 5, energy: 10, turns: 3 },
    FastMove { no: "264", name: "たたりめ", s_name: "tatarime", mtype: Type::Ghost, power: 6, energy: 12, turns: 3 },
    DUMMY_FM,
    FastMove { no: "266", name: "アイアンテール", s_name: "aiante-ru", mtype: Type::Steel, power: 9, energy: 6, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "269", name: "ほのおのうず", s_name: "honoonouzu", mtype: Type::Flare, power: 9, energy: 10, turns: 3 },

    DUMMY_FM,
    FastMove { no: "271", name: "タネマシンガン", s_name: "tanemasingann", mtype: Type::Grass, power: 5, energy: 13, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "274", name: "じんつうりき", s_name: "jintuuriki", mtype: Type::Psychic, power: 8, energy: 10, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "278", name: "バークアウト", s_name: "ba-kuauto", mtype: Type::Dark, power: 5, energy: 13, turns: 3 },
    DUMMY_FM,

    DUMMY_FM,
    FastMove { no: "281", name: "めざめるパワー※", s_name: "mezamerupawa-", mtype: Type::Normal, power: 9, energy: 8, turns: 3 },
    FastMove { no: "282", name: "とっしん", s_name: "tossinn", mtype: Type::Normal, power: 5, energy: 8, turns: 3 },
    FastMove { no: "283", name: "たきのぼり", s_name: "takinobori", mtype: Type::Water, power: 12, energy: 8, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "287", name: "あくび", s_name: "akubi", mtype: Type::Normal, power: 0, energy: 12, turns: 4 },
    DUMMY_FM,
    DUMMY_FM,

    DUMMY_FM,
    FastMove { no: "291", name: "プレゼント", s_name: "purezento", mtype: Type::Normal, power: 3, energy: 12, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "297", name: "うちおとす", s_name: "utiotosu", mtype: Type::Rock, power: 12, energy: 8, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,

    DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM,
    DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM,

    FastMove { no: "320", name: "あまえる", s_name: "amaeru", mtype: Type::Fairy, power: 15, energy: 6, turns: 3 },
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "325", name: "ロックオン", s_name: "rokkuonn", mtype: Type::Normal, power: 1, energy: 5, turns: 1 },
    FastMove { no: "326", name: "かみなりのキバ", s_name: "kaminarinokiba", mtype: Type::Electric, power: 8, energy: 5, turns: 2 },
    FastMove { no: "327", name: "こおりのキバ", s_name: "koorinokiba", mtype: Type::Ice, power: 8, energy: 5, turns: 2 },
    DUMMY_FM,
    DUMMY_FM,

    DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM, DUMMY_FM,

    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "345", name: "かぜおこし", s_name: "kazeokosi", mtype: Type::Flying, power: 16, energy: 12, turns: 4 },
    FastMove { no: "346", name: "やきつくす", s_name: "yakitukusu", mtype: Type::Flare, power: 15, energy: 20, turns: 5 },
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,

    FastMove { no: "350", name: "ようせいのかぜ", s_name: "youseinokaze", mtype: Type::Fairy, power: 3, energy: 9, turns: 2 },
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "356", name: "にどげり", s_name: "nidogeri", mtype: Type::Fighting, power: 8, energy: 12, turns: 3 },
    FastMove { no: "357", name: "マジカルリーフ", s_name: "majikaruri-fu", mtype: Type::Grass, power: 10, energy: 10, turns: 3 },
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
    FastMove { no: "368", name: "ころがる", s_name: "korogaru", mtype: Type::Rock, power: 5, energy: 13, turns: 3 },
    DUMMY_FM,

    DUMMY_FM,
    DUMMY_FM,
    DUMMY_FM,
    FastMove { no: "373", name: "みずしゅりけん", s_name: "mizusyurikenn", mtype: Type::Water, power: 6, energy: 14, turns: 3 },
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
    FastMove { no: "385", name: "このは", s_name: "konoha", mtype: Type::Grass, power: 6, energy: 7, turns: 2 },
    DUMMY_FM,
    FastMove { no: "387", name: "ジオコントロール", s_name: "jiokontoro-ru", mtype: Type::Fairy, power: 4, energy: 13, turns: 3 },
];

pub const NUM_CHARGE_MOVES: usize = 394;

const DUMMY_CM: ChargeMove = ChargeMove { no: "000", name: "dummy charge move", s_name: "", mtype: Type::Normal, power: 0, energy: 0, buff: None, buff_prob: 0.0 };

/// スペシャル技一覧
pub static CHARGE_MOVES: [ChargeMove; NUM_CHARGE_MOVES] = [
    DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM, DUMMY_CM,

    DUMMY_CM, DUMMY_CM, DUMMY_CM,
    ChargeMove { no: "013", name: "まきつく", s_name: "makituku", mtype: Type::Normal, power: 60, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "014", name: "はかいこうせん", s_name: "hakaikousenn", mtype: Type::Normal, power: 150, energy: 80, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "016", name: "あくのはどう", s_name: "akunohadou", mtype: Type::Dark, power: 80, energy: 50, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "018", name: "ヘドロこうげき", s_name: "hedorokougeki", mtype: Type::Poison, power: 50, energy: 40, buff: None, buff_prob: 0.0 },
    DUMMY_CM,

    ChargeMove { no: "020", name: "はさむ", s_name: "hasamu", mtype: Type::Normal, power: 40, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "021", name: "かえんぐるま", s_name: "kaenguruma", mtype: Type::Flare, power: 60, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "022", name: "メガホーン", s_name: "megaho-nn", mtype: Type::Bug, power: 110, energy: 55, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "024", name: "かえんほうしゃ", s_name: "kaenhousya", mtype: Type::Flare, power: 90, energy: 55, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "026", name: "あなをほる", s_name: "anawohoru", mtype: Type::Ground, power: 80, energy: 50, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "028", name: "クロスチョップ", s_name: "kurosutyoppu", mtype: Type::Fighting, power: 50, energy: 35, buff: None, buff_prob: 0.0 },
    DUMMY_CM,

    ChargeMove { no: "030", name: "サイケこうせん", s_name: "saikekousenn", mtype: Type::Psychic, power: 70, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "031", name: "じしん", s_name: "jisinn", mtype: Type::Ground, power: 110, energy: 65, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "032", name: "ストーンエッジ", s_name: "suto-nnejji", mtype: Type::Rock, power: 100, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "033", name: "れいとうパンチ", s_name: "reitoupanti", mtype: Type::Ice, power: 55, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "034", name: "ハートスタンプ", s_name: "ha-tosutanpu", mtype: Type::Psychic, power: 40, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "035", name: "ほうでん", s_name: "houdenn", mtype: Type::Electric, power: 65, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "036", name: "ラスターカノン", s_name: "rasuta-kanonn", mtype: Type::Steel, power: 110, energy: 70, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "038", name: "ドリルくちばし", s_name: "dorirukutibasi", mtype: Type::Flying, power: 65, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "039", name: "れいとうビーム", s_name: "reitoubi-mu", mtype: Type::Ice, power: 90, energy: 55, buff: None, buff_prob: 0.0 },

    ChargeMove { no: "040", name: "ふぶき", s_name: "fubuki", mtype: Type::Ice, power: 140, energy: 75, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "042", name: "ねっぷう", s_name: "neppuu", mtype: Type::Flare, power: 95, energy: 75, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "045", name: "つばめがえし", s_name: "tubamegaesi", mtype: Type::Flying, power: 55, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "046", name: "ドリルライナー", s_name: "doriruraina-", mtype: Type::Ground, power: 80, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "047", name: "はなふぶき", s_name: "hanafubuki", mtype: Type::Grass, power: 110, energy: 65, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "048", name: "メガドレイン", s_name: "megadoreinn", mtype: Type::Grass, power: 25, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "049", name: "むしのさざめき", s_name: "musinosazameki", mtype: Type::Bug, power: 100, energy: 60, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 30.0 },

    ChargeMove { no: "050", name: "どくどくのキバ", s_name: "dokudokunokiba", mtype: Type::Poison, power: 45, energy: 40, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 100.0 },
    ChargeMove { no: "051", name: "つじぎり", s_name: "tujigiri", mtype: Type::Dark, power: 50, energy: 35, buff: Some(Buff(2, 0, 0, 0)), buff_prob: 12.5 },
    DUMMY_CM,
    ChargeMove { no: "053", name: "バブルこうせん", s_name: "baburukousenn", mtype: Type::Water, power: 25, energy: 40, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 100.0 },
    ChargeMove { no: "054", name: "じごくぐるま", s_name: "jigokuguruma", mtype: Type::Fighting, power: 60, energy: 50, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "056", name: "ローキック", s_name: "ro-kikku", mtype: Type::Fighting, power: 40, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "057", name: "アクアジェット", s_name: "akuajetto", mtype: Type::Water, power: 45, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "058", name: "アクアテール", s_name: "akuate-ru", mtype: Type::Water, power: 50, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "059", name: "タネばくだん", s_name: "tanebakudann", mtype: Type::Grass, power: 60, energy: 45, buff: None, buff_prob: 0.0 },

    ChargeMove { no: "060", name: "サイコショック", s_name: "saikosyokku", mtype: Type::Psychic, power: 70, energy: 45, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "062", name: "げんしのちから", s_name: "gensinotikara", mtype: Type::Rock, power: 60, energy: 45, buff: Some(Buff(1, 1, 0, 0)), buff_prob: 10.0 },
    ChargeMove { no: "063", name: "がんせきふうじ", s_name: "gansekifuuji", mtype: Type::Rock, power: 70, energy: 60, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 100.0 },
    ChargeMove { no: "064", name: "いわなだれ", s_name: "iwanadare", mtype: Type::Rock, power: 75, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "065", name: "パワージェム", s_name: "pawa-jemu", mtype: Type::Rock, power: 80, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "066", name: "かげうち", s_name: "kageuti", mtype: Type::Ghost, power: 50, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "067", name: "シャドーパンチ", s_name: "syado-panti", mtype: Type::Ghost, power: 40, energy: 35, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "069", name: "あやしいかぜ", s_name: "ayasiikaze", mtype: Type::Ghost, power: 45, energy: 45, buff: Some(Buff(1, 1, 0, 0)), buff_prob: 10.0 },

    ChargeMove { no: "070", name: "シャドーボール", s_name: "syado-bo-ru", mtype: Type::Ghost, power: 100, energy: 55, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "072", name: "マグネットボム", s_name: "magunettobomu", mtype: Type::Steel, power: 70, energy: 45, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "074", name: "アイアンヘッド", s_name: "aianheddo", mtype: Type::Steel, power: 70, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "075", name: "パラボラチャージ", s_name: "paraboratya-ji", mtype: Type::Electric, power: 65, energy: 55, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "077", name: "かみなりパンチ", s_name: "kaminaripanti", mtype: Type::Electric, power: 55, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "078", name: "かみなり", s_name: "kaminari", mtype: Type::Electric, power: 100, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "079", name: "10まんボルト", s_name: "10manboruto", mtype: Type::Electric, power: 90, energy: 55, buff: None, buff_prob: 0.0 },

    ChargeMove { no: "080", name: "たつまき", s_name: "tatumaki", mtype: Type::Dragon, power: 45, energy: 45, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "082", name: "りゅうのはどう", s_name: "ryuunohadou", mtype: Type::Dragon, power: 90, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "083", name: "ドラゴンクロー", s_name: "doragonkuro-", mtype: Type::Dragon, power: 50, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "084", name: "チャームボイス", s_name: "tya-muboisu", mtype: Type::Fairy, power: 70, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "085", name: "ドレインキッス", s_name: "doreinnkissu", mtype: Type::Fairy, power: 60, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "086", name: "マジカルシャイン", s_name: "majikarusyainn", mtype: Type::Fairy, power: 110, energy: 70, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "087", name: "ムーンフォース", s_name: "mu-nfo-su", mtype: Type::Fairy, power: 110, energy: 60, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 10.0 },
    ChargeMove { no: "088", name: "じゃれつく", s_name: "jaretuku", mtype: Type::Fairy, power: 90, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "089", name: "クロスポイズン", s_name: "kurosupoizunn", mtype: Type::Poison, power: 50, energy: 35, buff: Some(Buff(2, 0, 0, 0)), buff_prob: 12.5 },

    ChargeMove { no: "090", name: "ヘドロばくだん", s_name: "hedorobakudann", mtype: Type::Poison, power: 80, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "091", name: "ヘドロウェーブ", s_name: "hedorowe-bu", mtype: Type::Poison, power: 110, energy: 65, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "092", name: "ダストシュート", s_name: "dasutosyu-to", mtype: Type::Poison, power: 130, energy: 75, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "094", name: "ホネこんぼう", s_name: "honekonbou", mtype: Type::Ground, power: 40, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "095", name: "じならし", s_name: "jinarasi", mtype: Type::Ground, power: 80, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "096", name: "どろばくだん", s_name: "dorobakudann", mtype: Type::Ground, power: 60, energy: 40, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "099", name: "シグナルビーム", s_name: "sigunarubi-mu", mtype: Type::Bug, power: 75, energy: 55, buff: Some(Buff(0, 0, -1, -1)), buff_prob: 20.0 },

    ChargeMove { no: "100", name: "シザークロス", s_name: "siza-kurosu", mtype: Type::Bug, power: 65, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "101", name: "ニトロチャージ", s_name: "nitorotya-ji", mtype: Type::Flare, power: 65, energy: 50, buff: Some(Buff(1, 0, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "102", name: "はじけるほのお", s_name: "hajikeruhonoo", mtype: Type::Flare, power: 70, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "103", name: "だいもんじ", s_name: "daimonji", mtype: Type::Flare, power: 140, energy: 80, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "104", name: "しおみず", s_name: "siomizu", mtype: Type::Water, power: 60, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "105", name: "みずのはどう", s_name: "mizunohadou", mtype: Type::Water, power: 70, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "106", name: "ねっとう", s_name: "nettou", mtype: Type::Water, power: 80, energy: 50, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 30.0 },
    ChargeMove { no: "107", name: "ハイドロポンプ", s_name: "haidoroponpu", mtype: Type::Water, power: 130, energy: 75, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "108", name: "サイコキネシス", s_name: "saikokinesisu", mtype: Type::Psychic, power: 85, energy: 55, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 10.0 },
    ChargeMove { no: "109", name: "サイコブレイク", s_name: "saikobureiku", mtype: Type::Psychic, power: 90, energy: 45, buff: None, buff_prob: 0.0 },

    DUMMY_CM,
    ChargeMove { no: "111", name: "こごえるかぜ", s_name: "kogoerukaze", mtype: Type::Ice, power: 60, energy: 45, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 100.0 },
    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "114", name: "ギガドレイン", s_name: "gigadoreinn", mtype: Type::Grass, power: 50, energy: 80, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "115", name: "ほのおのパンチ", s_name: "honoonopannti", mtype: Type::Flare, power: 55, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "116", name: "ソーラービーム", s_name: "so-ra-bi-mu", mtype: Type::Grass, power: 150, energy: 80, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "117", name: "リーフブレード", s_name: "ri-fubure-do", mtype: Type::Grass, power: 70, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "118", name: "パワーウィップ", s_name: "pawa-whippu", mtype: Type::Grass, power: 90, energy: 50, buff: None, buff_prob: 0.0 },
    DUMMY_CM,

    DUMMY_CM,
    ChargeMove { no: "121", name: "エアカッター", s_name: "eakatta-", mtype: Type::Flying, power: 60, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "122", name: "ぼうふう", s_name: "boufuu", mtype: Type::Flying, power: 110, energy: 65, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "123", name: "かわらわり", s_name: "kawarawari", mtype: Type::Fighting, power: 40, energy: 35, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "125", name: "スピードスター", s_name: "supi-dosuta-", mtype: Type::Normal, power: 60, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "126", name: "つのでつく", s_name: "tunodetuku", mtype: Type::Normal, power: 40, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "127", name: "ふみつけ", s_name: "fumituke", mtype: Type::Normal, power: 55, energy: 40, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "129", name: "ひっさつまえば", s_name: "hissatumaeba", mtype: Type::Normal, power: 80, energy: 50, buff: None, buff_prob: 0.0 },

    DUMMY_CM,
    ChargeMove { no: "131", name: "のしかかり", s_name: "nosikakari", mtype: Type::Normal, power: 60, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "132", name: "ねむる", s_name: "nemuru", mtype: Type::Normal, power: 50, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "133", name: "わるあがき", s_name: "waruagaki", mtype: Type::Normal, power: 35, energy: 100, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "134", name: "ねっとう(カメックス)", s_name: "nettoukamekkusu", mtype: Type::Water, power: 50, energy: 80, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "135", name: "ハイドロポンプ(カメックス)", s_name: "haidoroponpu", mtype: Type::Water, power: 90, energy: 80, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "136", name: "まきつく(緑)", s_name: "makituku", mtype: Type::Normal, power: 25, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "137", name: "まきつく(ピンク)", s_name: "makituku", mtype: Type::Normal, power: 25, energy: 45, buff: None, buff_prob: 0.0 },
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
    ChargeMove { no: "245", name: "インファイト", s_name: "infaito", mtype: Type::Fighting, power: 100, energy: 45, buff: Some(Buff(0, -2, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "246", name: "ばくれつパンチ", s_name: "bakuretupanti", mtype: Type::Fighting, power: 90, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "247", name: "きあいだま", s_name: "kiaidama", mtype: Type::Fighting, power: 150, energy: 75, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "248", name: "オーロラビーム", s_name: "o-rorabi-mu", mtype: Type::Ice, power: 80, energy: 60, buff: None, buff_prob: 0.0 },
    DUMMY_CM,

    DUMMY_CM,
    ChargeMove { no: "251", name: "ワイルドボルト", s_name: "wairudoboruto", mtype: Type::Electric, power: 100, energy: 45, buff: Some(Buff(0, -2, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "252", name: "でんじほう", s_name: "dennjihou", mtype: Type::Electric, power: 150, energy: 80, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 66.0 },
    DUMMY_CM,
    ChargeMove { no: "254", name: "ゆきなだれ", s_name: "yukinadare", mtype: Type::Ice, power: 90, energy: 45, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "256", name: "ブレイブバード", s_name: "bureibuba-do", mtype: Type::Flying, power: 130, energy: 55, buff: Some(Buff(0, -3, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "257", name: "ゴッドバード", s_name: "goddoba-do", mtype: Type::Flying, power: 75, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "258", name: "すなじごく", s_name: "sunajigoku", mtype: Type::Ground, power: 25, energy: 40, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 100.0 },
    ChargeMove { no: "259", name: "ロックブラスト", s_name: "rokkuburasuto", mtype: Type::Rock, power: 50, energy: 40, buff: None, buff_prob: 0.0 },

    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "262", name: "ぎんいろのかぜ", s_name: "ginnironokaze", mtype: Type::Bug, power: 60, energy: 45, buff: Some(Buff(1, 1, 0, 0)), buff_prob: 10.0 },
    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "265", name: "ナイトヘッド", s_name: "naitoheddo", mtype: Type::Ghost, power: 60, energy: 55, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "267", name: "ジャイロボール", s_name: "jairobo-ru", mtype: Type::Steel, power: 80, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "268", name: "ヘビーボンバー", s_name: "hebi-bonba-", mtype: Type::Steel, power: 70, energy: 50, buff: None, buff_prob: 0.0 },
    DUMMY_CM,

    ChargeMove { no: "270", name: "オーバーヒート", s_name: "o-ba-hi-to", mtype: Type::Flare, power: 130, energy: 55, buff: Some(Buff(-2, 0, 0, 0)), buff_prob: 100.0 },
    DUMMY_CM,
    ChargeMove { no: "272", name: "くさむすび", s_name: "kusamusubi", mtype: Type::Grass, power: 90, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "273", name: "エナジーボール", s_name: "enaji-bo-ru", mtype: Type::Grass, power: 90, energy: 55, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 10.0 },
    DUMMY_CM,
    ChargeMove { no: "275", name: "みらいよち", s_name: "miraiyoti", mtype: Type::Psychic, power: 120, energy: 65, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "276", name: "ミラーコート", s_name: "mira-ko-to", mtype: Type::Psychic, power: 60, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "277", name: "げきりん", s_name: "gekirinn", mtype: Type::Dragon, power: 110, energy: 60, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "279", name: "かみくだく", s_name: "kamikudaku", mtype: Type::Dark, power: 70, energy: 45, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 30.0 },

    ChargeMove { no: "280", name: "イカサマ", s_name: "ikasama", mtype: Type::Dark, power: 70, energy: 45, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "284", name: "なみのり", s_name: "naminori", mtype: Type::Water, power: 65, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "285", name: "りゅうせいぐん", s_name: "ryuuseigunn", mtype: Type::Dragon, power: 150, energy: 65, buff: Some(Buff(-2, 0, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "286", name: "はめつのねがい", s_name: "hametunonegai", mtype: Type::Steel, power: 75, energy: 40, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "288", name: "サイコブースト", s_name: "saikobu-suto", mtype: Type::Psychic, power: 70, energy: 35, buff: Some(Buff(-2, 0, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "289", name: "こんげんのはどう", s_name: "kongennnohadou", mtype: Type::Water, power: 130, energy: 60, buff: None, buff_prob: 0.0 },

    ChargeMove { no: "290", name: "だんがいのつるぎ", s_name: "dangainoturugi", mtype: Type::Ground, power: 130, energy: 60, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "292", name: "ウェザーボール(ほのお)", s_name: "weza-bo-ruhonoo", mtype: Type::Flare, power: 55, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "293", name: "ウェザーボール(こおり)", s_name: "weza-bo-rukoori", mtype: Type::Ice, power: 55, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "294", name: "ウェザーボール(いわ)", s_name: "weza-bo-ruiwa", mtype: Type::Rock, power: 55, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "295", name: "ウェザーボール(みず)", s_name: "weza-bo-rumizu", mtype: Type::Water, power: 55, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "296", name: "ハードプラント", s_name: "ha-dopuranto", mtype: Type::Grass, power: 100, energy: 45, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "298", name: "ブラストバーン", s_name: "burasutoba-nn", mtype: Type::Flare, power: 110, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "299", name: "ハイドロカノン", s_name: "haidorokanonn", mtype: Type::Water, power: 80, energy: 40, buff: None, buff_prob: 0.0 },

    ChargeMove { no: "300", name: "とっておき", s_name: "totteoki", mtype: Type::Normal, power: 90, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "301", name: "コメットパンチ", s_name: "komettopanti", mtype: Type::Steel, power: 100, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "302", name: "ロケットずつき", s_name: "rokettozutuki", mtype: Type::Normal, power: 130, energy: 75, buff: Some(Buff(0, 1, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "303", name: "アシッドボム", s_name: "asiddobomu", mtype: Type::Poison, power: 20, energy: 45, buff: Some(Buff(0, 0, 0, -2)), buff_prob: 100.0 },
    ChargeMove { no: "304", name: "だいちのちから", s_name: "daitinotikara", mtype: Type::Ground, power: 90, energy: 55, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 10.0 },
    ChargeMove { no: "305", name: "クラブハンマー", s_name: "kurabuhanma-", mtype: Type::Water, power: 85, energy: 50, buff: Some(Buff(2, 0, 0, 0)), buff_prob: 12.5 },
    ChargeMove { no: "306", name: "とびかかる", s_name: "tobikakaru", mtype: Type::Bug, power: 60, energy: 45, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 100.0 },
    DUMMY_CM, //ChargeMove { no: "307", name: "ブレイククロー", s_name: "bureikukuro-", mtype: Type::Normal, gym_power: , gym_energy: , gym_time: 1.9, power: , energy: , buff: None, buff_prob: 0.0 },
    ChargeMove { no: "308", name: "オクタンほう", s_name: "okutanhou", mtype: Type::Water, power: 50, energy: 50, buff: Some(Buff(0, 0, -2, 0)), buff_prob: 50.0 },
    ChargeMove { no: "309", name: "ミラーショット", s_name: "mira-syotto", mtype: Type::Steel, power: 35, energy: 35, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 30.0 },

    ChargeMove { no: "310", name: "ばかぢから", s_name: "bakadikara", mtype: Type::Fighting, power: 85, energy: 40, buff: Some(Buff(-1, -1, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "311", name: "とどめばり", s_name: "todomebari", mtype: Type::Bug, power: 20, energy: 35, buff: Some(Buff(1, 0, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "312", name: "グラスミキサー", s_name: "gurasumikisa-", mtype: Type::Grass, power: 45, energy: 40, buff: Some(Buff(0, 0, -2, 0)), buff_prob: 50.0 },
    DUMMY_CM, //ChargeMove { no: "313", name: "きゅうけつ", s_name: "kyuuketu", mtype: Type::Bug, gym_power: , gym_energy: , gym_time: 2.5, power: , energy: , buff: None, buff_prob: 0.0 },
    ChargeMove { no: "314", name: "ドレインパンチ", s_name: "doreinpanti", mtype: Type::Fighting, power: 20, energy: 40, buff: Some(Buff(0, 1, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "315", name: "シャドーボーン", s_name: "syado-bo-nn", mtype: Type::Ghost, power: 75, energy: 45, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 20.0 },
    ChargeMove { no: "316", name: "だくりゅう", s_name: "dakuryuu", mtype: Type::Water, power: 35, energy: 35, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 30.0 },
    ChargeMove { no: "317", name: "ブレイズキック", s_name: "bureizukikku", mtype: Type::Flare, power: 55, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "318", name: "シェルブレード", s_name: "syerubure-do", mtype: Type::Water, power: 35, energy: 35, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 50.0 },
    ChargeMove { no: "319", name: "グロウパンチ", s_name: "guroupanti", mtype: Type::Fighting, power: 20, energy: 35, buff: Some(Buff(1, 0, 0, 0)), buff_prob: 100.0 },

    DUMMY_CM,
    ChargeMove { no: "321", name: "ギガインパクト", s_name: "gigainpakuto", mtype: Type::Normal, power: 150, energy: 80, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "322", name: "やつあたり", s_name: "yatuatari", mtype: Type::Normal, power: 10, energy: 70, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "323", name: "おんがえし", s_name: "ongaesi", mtype: Type::Normal, power: 130, energy: 70, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "324", name: "シンクロノイズ", s_name: "sinkuronoizu", mtype: Type::Psychic, power: 80, energy: 50, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    DUMMY_CM,
    DUMMY_CM,
    DUMMY_CM, //ChargeMove { no: "328", name: "つのドリル", s_name: "tunodoriru", mtype: Type::Normal, gym_power: , gym_energy: , gym_time: 1.9, power: , energy: , buff: None, buff_prob: 0.0 },
    DUMMY_CM, //ChargeMove { no: "329", name: "じわれ", s_name: "jiware", mtype: Type::Ground, gym_power: , gym_energy: , gym_time: 2.8, power: , energy: , buff: None, buff_prob: 0.0 },

    ChargeMove { no: "330", name: "せいなるつるぎ", s_name: "seinaruturugi", mtype: Type::Fighting, power: 60, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "331", name: "フライングプレス", s_name: "furaingupuresu", mtype: Type::Fighting, power: 90, energy: 40, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "332", name: "はどうだん", s_name: "hadoudann", mtype: Type::Fighting, power: 100, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "333", name: "しっぺがえし", s_name: "sippegaesi", mtype: Type::Dark, power: 110, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "334", name: "がんせきほう", s_name: "gansekihou", mtype: Type::Rock, power: 110, energy: 50, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "335", name: "エアロブラスト", s_name: "earoburasuto", mtype: Type::Flying, power: 170, energy: 75, buff: Some(Buff(2, 0, 0, 0)), buff_prob: 12.5 },
    ChargeMove { no: "336", name: "テクノバスター(ノーマル)", s_name: "tekunobasuta-no-maru", mtype: Type::Normal, power: 120, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "337", name: "テクノバスター(ほのお)", s_name: "tekunobasuta-honoo", mtype: Type::Flare, power: 120, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "338", name: "テクノバスター(こおり)", s_name: "tekunobasuta-koori", mtype: Type::Ice, power: 120, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "339", name: "テクノバスター(みず)", s_name: "tekunobasuta-mizu", mtype: Type::Water, power: 120, energy: 55, buff: None, buff_prob: 0.0 },

    ChargeMove { no: "340", name: "テクノバスター(でんき)", s_name: "tekunobasuta-denki", mtype: Type::Electric, power: 120, energy: 55, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "341", name: "そらをとぶ", s_name: "sorawotobu", mtype: Type::Flying, power: 80, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "342", name: "Vジェネレート", s_name: "vjenere-to", mtype: Type::Flare, power: 95, energy: 40, buff: Some(Buff(0, -3, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "343", name: "リーフストーム", s_name: "ri-fusuto-mu", mtype: Type::Grass, power: 130, energy: 55, buff: Some(Buff(-2, 0, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "344", name: "トライアタック", s_name: "toraiatakku", mtype: Type::Normal, power: 65, energy: 50, buff: Some(Buff(0, 0, -1, -1)), buff_prob: 50.0 },
    DUMMY_CM,
    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "348", name: "フェザーダンス", s_name: "feza-dansu", mtype: Type::Flying, power: 35, energy: 50, buff: Some(Buff(0, 0, -2, 0)), buff_prob: 100.0 },
    DUMMY_CM,

    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "352", name: "ウェザーボール(ノーマル)", s_name: "weza-bo-runo-maru", mtype: Type::Normal, power: 55, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "353", name: "サイコファング", s_name: "saikofangu", mtype: Type::Psychic, power: 40, energy: 35, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 100.0 },
    DUMMY_CM,
    DUMMY_CM,
    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "358", name: "せいなるほのお", s_name: "seinaruhonoo", mtype: Type::Flare, power: 130, energy: 65, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 50.0 },
    ChargeMove { no: "359", name: "つららばり", s_name: "turarabari", mtype: Type::Ice, power: 65, energy: 40, buff: None, buff_prob: 0.0 },

    ChargeMove { no: "360", name: "エアロブラスト+", s_name: "earoburasuto+", mtype: Type::Flying, power: 170, energy: 75, buff: Some(Buff(2, 0, 0, 0)), buff_prob: 12.5 },
    ChargeMove { no: "361", name: "エアロブラスト++", s_name: "earoburasuto++", mtype: Type::Flying, power: 170, energy: 75, buff: Some(Buff(2, 0, 0, 0)), buff_prob: 12.5 },
    ChargeMove { no: "362", name: "せいなるほのお+", s_name: "seinaruhonoo+", mtype: Type::Flare, power: 130, energy: 65, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 50.0 },
    ChargeMove { no: "363", name: "せいなるほのお++", s_name: "seinaruhonoo++", mtype: Type::Flare, power: 130, energy: 65, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 50.0 },
    ChargeMove { no: "364", name: "アクロバット", s_name: "akurobatto", mtype: Type::Flying, power: 110, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "365", name: "ラスターパージ", s_name: "rasuta-pa-ji", mtype: Type::Psychic, power: 120, energy: 60, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 50.0 },
    ChargeMove { no: "366", name: "ミストボール", s_name: "misutobo-ru", mtype: Type::Psychic, power: 120, energy: 60, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 50.0 },
    ChargeMove { no: "367", name: "ぶんまわす", s_name: "bunmawasu", mtype: Type::Dark, power: 65, energy: 40, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    ChargeMove { no: "369", name: "シードフレア", s_name: "si-dofurea", mtype: Type::Grass, power: 130, energy: 75, buff: Some(Buff(0, 0, 0, -2)), buff_prob: 40.0 },

    ChargeMove { no: "370", name: "ブロッキング", s_name: "burokkingu", mtype: Type::Dark, power: 15, energy: 40, buff: Some(Buff(0, 1, 0, -1)), buff_prob: 100.0 },
    ChargeMove { no: "371", name: "シャドーダイブ", s_name: "syado-daibu", mtype: Type::Ghost, power: 120, energy: 90, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "372", name: "メテオビーム", s_name: "meteobi-mu", mtype: Type::Rock, power: 120, energy: 60, buff: Some(Buff(1, 0, 0, 0)), buff_prob: 100.0 },
    DUMMY_CM,
    ChargeMove { no: "374", name: "クロスサンダー", s_name: "kurosusanda-", mtype: Type::Electric, power: 90, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "375", name: "クロスフレイム", s_name: "kurosufureimu", mtype: Type::Flare, power: 90, energy: 45, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "376", name: "ポルターガイスト", s_name: "poruta-gaisuto", mtype: Type::Ghost, power: 150, energy: 75, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "377", name: "10まんばりき", s_name: "10manbariki", mtype: Type::Ground, power: 100, energy: 60, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "378", name: "こごえるせかい", s_name: "kogoerusekai", mtype: Type::Ice, power: 60, energy: 40, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 100.0 },
    ChargeMove { no: "379", name: "ワイドブレイカー", s_name: "waidobureika-", mtype: Type::Dragon, power: 50, energy: 35, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 100.0 },

    ChargeMove { no: "380", name: "ばくおんぱ", s_name: "bakuonpa", mtype: Type::Normal, power: 150, energy: 70, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "381", name: "ダブルパンツァー", s_name: "daburupantula-", mtype: Type::Steel, power: 50, energy: 35, buff: None, buff_prob: 0.0 },
    ChargeMove { no: "382", name: "マジカルフレイム", s_name: "majikarufureimu", mtype: Type::Flare, power: 60, energy: 45, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 100.0 },
    ChargeMove { no: "383", name: "アクアブレイク", s_name: "akuabureiku", mtype: Type::Water, power: 70, energy: 45, buff: Some(Buff(0, 0, 0, -1)), buff_prob: 30.0 },
    ChargeMove { no: "384", name: "ガリョウテンセイ", s_name: "garyoutensei", mtype: Type::Flying, power: 150, energy: 70, buff: Some(Buff(0, -1, 0, 0)), buff_prob: 100.0 },
    DUMMY_CM,
    ChargeMove { no: "386", name: "マグマストーム", s_name: "magumasuto-mu", mtype: Type::Flare, power: 65, energy: 40, buff: None, buff_prob: 0.0 },
    DUMMY_CM,
    DUMMY_CM,
    ChargeMove { no: "389", name: "デスウイング", s_name: "desuuingu", mtype: Type::Flying, power: 85, energy: 50, buff: None, buff_prob: 0.0 },

    DUMMY_CM,
    ChargeMove { no: "391", name: "トリプルアクセル", s_name: "toripuruakuseru", mtype: Type::Ice, power: 60, energy: 45, buff: Some(Buff(1, 0, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "392", name: "くさわけ", s_name: "kusawake", mtype: Type::Grass, power: 65, energy: 50, buff: Some(Buff(1, 0, 0, 0)), buff_prob: 100.0 },
    ChargeMove { no: "393", name: "ねっさのだいち", s_name: "nessanodaiti", mtype: Type::Ground, power: 80, energy: 50, buff: Some(Buff(0, 0, -1, 0)), buff_prob: 30.0 },
];
