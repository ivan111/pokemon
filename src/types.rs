/// 技の属性
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
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

impl From<&str> for Type {
    fn from(s: &str) -> Self {
        match s {
            "ノーマル" => Type::Normal,
            "ほのお" => Type::Flare,
            "みず" => Type::Water,
            "でんき" => Type::Electric,
            "くさ" => Type::Grass,
            "こおり" => Type::Ice,
            "かくとう" => Type::Fighting,
            "どく" => Type::Poison,
            "じめん" => Type::Ground,
            "ひこう" => Type::Flying,
            "エスパー" => Type::Psychic,
            "むし" => Type::Bug,
            "いわ" => Type::Rock,
            "ゴースト" => Type::Ghost,
            "ドラゴン" => Type::Dragon,
            "あく" => Type::Dark,
            "はがね" => Type::Steel,
            "フェアリー" => Type::Fairy,
            _ => panic!("Type::from(): 知らないタイプ {}", s),
        }
    }
}

impl Into<String> for Type {
    fn into(self) -> String {
        let s = match self {
            Type::Normal => "ノーマル",
            Type::Flare => "ほのお",
            Type::Water => "みず",
            Type::Electric => "でんき",
            Type::Grass => "くさ",
            Type::Ice => "こおり",
            Type::Fighting => "かくとう",
            Type::Poison => "どく",
            Type::Ground => "じめん",
            Type::Flying => "ひこう",
            Type::Psychic => "エスパー",
            Type::Bug => "むし",
            Type::Rock => "いわ",
            Type::Ghost => "ゴースト",
            Type::Dragon => "ドラゴン",
            Type::Dark => "あく",
            Type::Steel => "はがね",
            Type::Fairy => "フェアリー",
        };

        String::from(s)
    }
}

pub const TYPE_EFFECT_MATRIX: [[i8; 18]; 18] = [
    [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,-1,-2, 0, 0,-1, 0],  // ノーマル (攻撃側)
    [ 0,-1,-1, 0, 1, 1, 0, 0, 0, 0, 0, 1,-1, 0,-1, 0, 1, 0],  // ほのお
    [ 0, 1,-1, 0,-1, 0, 0, 0, 1, 0, 0, 0, 1, 0,-1, 0, 0, 0],  // みず
    [ 0, 0, 1,-1,-1, 0, 0, 0,-2, 1, 0, 0, 0, 0,-1, 0, 0, 0],  // でんき
    [ 0,-1, 1, 0,-1, 0, 0,-1, 1,-1, 0,-1, 1, 0,-1, 0,-1, 0],  // くさ
    [ 0,-1,-1, 0, 1,-1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0,-1, 0],  // こおり
    [ 1, 0, 0, 0, 0, 1, 0,-1, 0,-1,-1,-1, 1,-2, 0, 1, 1,-1],  // かくとう
    [ 0, 0, 0, 0, 1, 0, 0,-1,-1, 0, 0, 0,-1,-1, 0, 0,-2, 1],  // どく
    [ 0, 1, 0, 1,-1, 0, 0, 1, 0,-2, 0,-1, 1, 0, 0, 0, 1, 0],  // じめん
    [ 0, 0, 0,-1, 1, 0, 1, 0, 0, 0, 0, 1,-1, 0, 0, 0,-1, 0],  // ひこう
    [ 0, 0, 0, 0, 0, 0, 1, 1, 0, 0,-1, 0, 0, 0, 0,-2,-1, 0],  // エスパー
    [ 0,-1, 0, 0, 1, 0,-1,-1, 0,-1, 1, 0, 0,-1, 0, 1,-1,-1],  // むし
    [ 0, 1, 0, 0, 0, 1,-1, 0,-1, 1, 0, 1, 0, 0, 0, 0,-1, 0],  // いわ
    [-2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0,-1, 0, 0],  // ゴースト
    [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,-1,-2],  // ドラゴン
    [ 0, 0, 0, 0, 0, 0,-1, 0, 0, 0, 1, 0, 0, 1, 0,-1, 0,-1],  // あく
    [ 0,-1,-1,-1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,-1, 1],  // はがね
    [ 0,-1, 0, 0, 0, 0, 1,-1, 0, 0, 0, 0, 0, 0, 1, 1,-1, 0]   // フェアリー
];

pub const TYPE_EFFECT_ARR: [f64; 6] = [1.0 / (1.6*1.6*1.6), 1.0 / (1.6*1.6), 1.0 / 1.6, 1.0, 1.6, 1.6*1.6];

pub const NUM_TYPES: usize = 18;

pub const TYPES: [Type; NUM_TYPES] = [
    Type::Normal, Type::Flare, Type::Water, Type::Electric, Type::Grass, Type::Ice, Type::Fighting, Type::Poison, Type::Ground,
    Type::Flying, Type::Psychic, Type::Bug, Type::Rock, Type::Ghost, Type::Dragon, Type::Dark, Type::Steel, Type::Fairy
];

pub const TYPE_NAMES: [&str; NUM_TYPES] = [
    "ノーマル", "ほのお", "みず", "でんき", "くさ", "こおり", "かくとう", "どく", "じめん",
    "ひこう", "エスパー", "むし", "いわ", "ゴースト", "ドラゴン", "あく", "はがね", "フェアリー"
];

impl Type {
    pub fn print_effect_table(type2: Option<Type>) {
        println!();

        if let Some(t2) = type2 {
            let name: String = t2.into();
            println!("複合タイプ: {}", name);
            println!();
        };

        println!("          | ノ  ほ  み  で  く  こ  か  ど  じ  ひ  エ  む  い  ゴ  ド  あ  は  フ");
        println!("          | ｜  の  ず  ん  さ  お  く  く  め  こ  ス  し  わ  ｜  ラ  く  が  ェ");
        println!("          | マ  お      き      り  と      ん  う  パ          ス  ゴ      ね  ア");
        println!("          | ル                      う              ｜          ト  ン          リ");
        println!("          |                                                                     ｜");
        println!("----------------------------------------------------------------------------------");

        for i in 0..NUM_TYPES {
            let name = TYPE_NAMES[i];
            let num_spaces = 10 - (name.len() / 3 * 2);
            print!("{}{:<width$}| ", name, "", width=num_spaces);

            for k in 0..NUM_TYPES {
                let effect;

                if let Some(t2) = type2 {
                    let ti = t2 as usize;

                    if k != ti {
                        effect = (TYPE_EFFECT_MATRIX[i][k] + TYPE_EFFECT_MATRIX[i][ti]).to_string();
                    } else {
                        effect = ("  ").to_string();
                    }
                } else {
                    effect = (TYPE_EFFECT_MATRIX[i][k]).to_string();
                }

                if k == 0 {
                    print!("{:>2}", effect);
                } else  {
                    print!("  {:>2}", effect);
                }
            }

            println!();
        }

        println!();
    }

    pub fn get_type_effect_bonus(&self, types: &Vec<Self>) -> f64 {
        let i = types.iter().map(|t| TYPE_EFFECT_MATRIX[*self as usize][*t as usize]).sum::<i8>();

        assert!((-3..=2).contains(&i));

        TYPE_EFFECT_ARR[(i + 3) as usize]
    }
}

#[test]
fn test_get_type_effect_bonus() {
    let mut effects = vec![];
    for v in TYPE_EFFECT_ARR {
        effects.push(v);
    }

    for t in TYPES {
        for t1 in TYPES {
            for t2 in TYPES {
                if t1 == t2 {
                    continue;
                }

                let v = t.get_type_effect_bonus(&vec![t1, t2]);
                assert!(effects.contains(&v));
            }
        }
    }
}
