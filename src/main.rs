mod pokepedia;
mod cpm;
mod types;
mod moves;
mod index;
mod pokemon;
mod battle;
mod ranking;
mod utils;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write, BufReader};

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::pokepedia::Pokepedia;
use crate::pokemon::{Pokemon, IVs, calc_lv, search_near_iv};
use crate::moves::{FastMove, ChargeMove};

fn load_pokemons() -> HashMap<String, Vec<Pokemon>> {
    let mut poke_path = dirs::home_dir().unwrap();
    poke_path.push("pokemons");

    let mut pdir = HashMap::new();

    for entry in poke_path.read_dir().expect("pokemonsディレクトリの読み込みに失敗") {
        let entry = entry.expect("pokemonsディレクトリの読み込みに失敗");

        let file_name = entry.file_name().to_string_lossy().into_owned();

        let pokemons = {
            let f = File::open(&poke_path.join(file_name.clone())).unwrap();
            let mut reader = BufReader::new(f);
            pokemon::load_pokemon(&mut reader).unwrap()
        };

        pdir.insert(file_name, pokemons);
    }

    pdir
}

fn main() -> Result<()> {
    let mut cd = "main".to_string();  // カレントディレクトリ

    let mut pdir = load_pokemons();

    let mut rl = DefaultEditor::new()?;

    'repl: loop {
        let prompt = format!("{} > ", cd);
        let readline = rl.readline(&prompt);

        match readline {
            Ok(src_line) => {
                let line = src_line.trim();
                let _ = rl.add_history_entry(line);
                let words = line.split_whitespace().collect::<Vec<_>>();
                let cmd = words[0];

                match cmd {
                    "q" | "quit" | "exit" => {
                        break;
                    },

                    "ls" => {
                        if words.len() == 2 {
                            let name = String::from(words[1]);

                            if name == "/" {
                                for s in pdir.keys() {
                                    println!("{}", s);
                                }
                            } else if pdir.contains_key(&name) {
                                for p in pdir.get(&name).unwrap() {
                                    p.print();
                                }
                            } else {
                                eprintln!("存在しないディレクトリ: {}", name);
                            }
                        } else {
                            for p in pdir.get(&cd).unwrap() {
                                p.print();
                            }
                        }
                    },

                    "cd" => {
                        match words.len() {
                            1 => { cd = "main".to_string(); },
                            2 => {
                                let name = String::from(words[1]);

                                if pdir.contains_key(&name) {
                                    cd = name;
                                } else {
                                    eprintln!("存在しないディレクトリ: {}", name);
                                }
                            },
                            _ => {
                                eprintln!("引数が多すぎる");
                            }
                        }
                    },

                    "a" | "add" => {
                        if let Some(dict) = pokepedia::skim_pokepedia() {
                            println!("ポケモン: {}", dict.name());

                            let mut cp = read_cp();
                            let mut ivs = read_ivs();

                            let fast_move = read_fast_move(dict);
                            println!("ノーマルアタック: {}", fast_move.name());

                            let charge_move1 = read_charge_move1(dict);
                            println!("スペシャルアタック1: {}", charge_move1.name());

                            let charge_move2 = read_charge_move2(dict);
                            if let Some(mv) = charge_move2 {
                                println!("スペシャルアタック2: {}", mv.name());
                            }

                            let mut lv;

                            loop {
                                lv = match calc_lv(dict, cp, ivs) {
                                    None => {
                                        let mut msgs = vec![];
                                        msgs.push(format!("{}: ポケモンレベルの取得に失敗(CPか個体値が間違っている)", dict.name()));

                                        let near_ivs = search_near_iv(dict, cp, ivs);
                                        if near_ivs.is_empty() {
                                            eprintln!("CPか個体値の入力が間違っている。");

                                            if !yes_or_no("CPと個体値を入力しなおす？(Yesならyを入力): ") {
                                                continue 'repl;
                                            }

                                            cp = read_cp();
                                            ivs = read_ivs();
                                            continue;
                                        } else {
                                            msgs.push("もしかして、この値?".to_string());

                                            for ivs in near_ivs {
                                                msgs.push(format!("{:?}", ivs.to_tuple()));
                                            }
                                        }

                                        eprintln!("{}", msgs.join("\n"));
                                        ivs = read_ivs();
                                        continue;
                                    },

                                    Some(lv) => lv,
                                };

                                break;
                            }

                        }
                    },

                    "effect" => {
                        types::Type::print_effect_table(None);
                    },

                    "" => (),

                    _ => {
                        eprintln!("存在しないコマンド {}", cmd);
                    },
                }
            },

            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break
            },

            Err(err) => {
                eprintln!("Error: {:?}", err);
                break
            }
        }
    }

    Ok(())
}

fn read_cp() -> i32 {
    loop {
        let mut cp_str = String::new();

        print!("CP: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut cp_str).expect("CPの読み込みに失敗");

        if let Ok(cp) = cp_str.trim().parse() {
            if (10..10000).contains(&cp) {
                return cp;
            }

            eprintln!("CPの値が正しくありません。");
        } else {
            eprintln!("CPは数字で入力してください。");
        }
    }
}

fn read_ivs() -> IVs {
    'outer: loop {
        let mut ivs_str = String::new();

        print!("個体値(攻撃 防御 スタミナ (ex) 7 14 3): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut ivs_str).expect("個体値の読み込みに失敗");

        let ivs_str_vec = ivs_str.split_whitespace().collect::<Vec<_>>();

        if ivs_str_vec.len() != 3 {
            eprintln!("空白区切りの3つの数字を入力してください。");
            continue;
        }

        let mut ivs_vec: Vec<i32> = Vec::new();

        for s in ivs_str_vec {
            if let Ok(v) = s.trim().parse() {
                if (0..16).contains(&v) {
                    ivs_vec.push(v);
                } else {
                    eprintln!("値の範囲が正しくありません。0から15の値で入力してください。");
                    continue 'outer;
                }
            } else {
                eprintln!("値は数字で入力してください。");
                continue 'outer;
            }
        }

        return IVs::new(ivs_vec[0], ivs_vec[1], ivs_vec[2]).unwrap();
    }
}

fn read_fast_move(dict: &'static Pokepedia) -> &'static FastMove {
    loop {
        if let Some(mv) = moves::skim_fast_move_in_dict(dict) {
            return mv;
        }
    }
}

fn read_charge_move1(dict: &'static Pokepedia) -> &'static ChargeMove {
    loop {
        if let Some(mv) = moves::skim_charge_move_in_dict(dict) {
            return mv;
        }
    }
}

fn yes_or_no(prompt: &str) -> bool {
    let mut q = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut q).expect("読み込みに失敗");

    q.trim() == "y"
}

fn read_charge_move2(dict: &'static Pokepedia) -> Option<&'static ChargeMove> {
    if !yes_or_no("スペシャルアタック2もある？(あるならyを入力): ") {
        return None;
    }

    loop {
        if let Some(mv) = moves::skim_charge_move_in_dict(dict) {
            return Some(mv);
        }
    }
}
