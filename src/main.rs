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
use crate::utils::jp_width;

fn main() -> Result<()> {
    let mut cd = "main".to_string();  // カレントディレクトリ

    let (mut pdir, mut changed_pdir) = load_pokemons();

    let mut rl = DefaultEditor::new()?;

    loop {
        let prompt = format!("{} > ", cd);
        let readline = rl.readline(&prompt);

        match readline {
            Ok(src_line) => {
                let line = src_line.trim();
                let _ = rl.add_history_entry(line);
                let words = line.split_whitespace().collect::<Vec<_>>();
                if words.is_empty() {
                    continue;
                }
                let cmd = words[0];

                match cmd {
                    "q" | "quit" | "exit" => {
                        let mut is_changed = false;
                        for v in changed_pdir.values() {
                            if *v {
                                is_changed = true;
                                break;
                            }
                        }

                        if is_changed {
                            if yes_or_no("変更を保存せず終了しますか？(Yesならyを入力): ") {
                                break;
                            }
                        } else {
                            break;
                        }
                    },

                    "ls" => {
                        if words.len() == 2 {
                            let name = String::from(words[1]);

                            if name == "/" {
                                for s in pdir.keys() {
                                    println!("{}", s);
                                }
                            } else if pdir.contains_key(&name) {
                                ls_print(pdir.get(&name).unwrap());
                            } else {
                                eprintln!("存在しないディレクトリ: {}", name);
                            }
                        } else {
                            ls_print(pdir.get(&cd).unwrap());
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
                        if let Some(poke) = create_pokemon() {
                            let v = pdir.get_mut(&cd).unwrap();
                            v.push(poke);
                            println!("ポケモンを作成しました。");

                            changed_pdir.insert(cd.clone(), true);
                        }
                    },

                    "e" | "edit" => {
                        let pokemons = pdir.get_mut(&cd).unwrap();
                        if let Some(poke) = select_pokemon_mut(&mut *pokemons) {
                            edit_pokemon(poke);
                            changed_pdir.insert(cd.clone(), true);
                        }
                    },

                    "rm" => {
                        let pokemons = pdir.get_mut(&cd).unwrap();
                        if remove_pokemons(&mut *pokemons) {
                            changed_pdir.insert(cd.clone(), true);
                            println!("削除しました。");
                        }
                    },

                    "save" => {
                        save_pokemons(&pdir, &mut changed_pdir);
                        println!("保存しました。");
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

fn load_pokemons() -> (HashMap<String, Vec<Pokemon>>, HashMap<String, bool>) {
    let mut poke_path = dirs::home_dir().unwrap();
    poke_path.push("pokemons");

    let mut pdir = HashMap::new();
    let mut changed_pdir = HashMap::new();

    for entry in poke_path.read_dir().expect("pokemonsディレクトリの読み込みに失敗") {
        let entry = entry.expect("pokemonsディレクトリの読み込みに失敗");

        let file_name = entry.file_name().to_string_lossy().into_owned();

        let pokemons = {
            let f = File::open(&poke_path.join(file_name.clone())).unwrap();
            let mut reader = BufReader::new(f);
            pokemon::load_pokemons(&mut reader).unwrap()
        };

        pdir.insert(file_name.clone(), pokemons);
        changed_pdir.insert(file_name, false);
    }

    (pdir, changed_pdir)
}

fn save_pokemons(pdir: &HashMap<String, Vec<Pokemon>>, changed_pdir: &mut HashMap<String, bool>) {
    let mut poke_path = dirs::home_dir().unwrap();
    poke_path.push("pokemons");

    for (k, v) in &mut *changed_pdir {
        if *v && pdir.get(k).is_some() {
            let file_name = poke_path.join(k);

            let mut writer = File::create(&poke_path.join(file_name.clone())).unwrap();

            let _ = pokemon::save_pokemons(&mut writer, pdir.get(k).unwrap());

            *v = false;
        }
    }
}

fn ls_print(pokes: &[Pokemon]) {
    let width = pokes.iter().map(|p| jp_width(p.name())).max();

    if let Some(width) = width {
        for p in pokes {
            println!("{}", p.format(width));
        }
    }
}

fn create_pokemon() -> Option<Pokemon> {
    let dict = match pokepedia::skim_pokepedia() {
        None => { return None; },
        Some(dict) => dict
    };
    println!("ポケモン: {}", dict.name());

    let mut cp = read_cp();
    let mut ivs = read_ivs();

    let lv = match calc_lv_or_read_again(dict, &mut cp, &mut ivs) {
        None => { return None; },
        Some(lv) => lv,
    };

    println!("ポケモンLv: {}", lv);

    let fast_move = read_fast_move(dict);
    println!("ノーマルアタック: {}", fast_move.name());

    let charge_move1 = read_charge_move1(dict);
    println!("スペシャルアタック1: {}", charge_move1.name());

    let charge_move2 = read_charge_move2(dict);
    if let Some(mv) = charge_move2 {
        println!("スペシャルアタック2: {}", mv.name());
    }

    Some(Pokemon::raw_new(dict, lv, ivs, fast_move, charge_move1, charge_move2))
}

fn select_pokemon_mut(pokemons: &mut [Pokemon]) -> Option<&mut Pokemon> {
    let width = pokemons.iter().map(|p| jp_width(p.name())).max();

    if let Some(width) = width {
        match pokemon::skim_pokemons(pokemons, width) {
            None => None,
            Some(i) => {
                Some(&mut pokemons[i])
            }
        }
    } else {
        None
    }
}

fn edit_pokemon(poke: &mut Pokemon) {
    loop {
        let mut input = String::new();

        print!("(q)uit, (c)p, (f)ast move, charge move(1) | (2), (r)emove charge move 2: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("読み込みに失敗");

        match input.trim() {
            "c" => {
                let cp = read_cp();
                if poke.set_cp(cp) {
                    println!("CPを'{}'に変更", cp);
                } else {
                    eprintln!("CPが間違っている");
                }
            },
            "f" => {
                let fast_move = read_fast_move(poke.dict());
                poke.set_fast_move(fast_move);
                println!("ノーマルアタックを'{}'に変更", fast_move.name());
            },
            "1" => {
                let charge_move = read_charge_move1(poke.dict());
                poke.set_charge_move1(charge_move);
                println!("スペシャルアタック1を'{}'に変更", charge_move.name());
            },
            "2" => {
                let charge_move = read_charge_move1(poke.dict());
                poke.set_charge_move2(Some(charge_move));
                println!("スペシャルアタック2を'{}'に変更", charge_move.name());
            },
            "r" => {
                poke.set_charge_move2(None);
                println!("スペシャルアタック2を削除");
            },
            "q" => { return; },
            _ => (),
        }
    }
}

fn remove_pokemons(pokemons: &mut Vec<Pokemon>) -> bool {
    let width = pokemons.iter().map(|p| jp_width(p.name())).max();

    if let Some(width) = width {
        match pokemon::skim_pokemons(pokemons, width) {
            None => false,
            Some(i) => {
                pokemons.remove(i);
                true
            }
        }
    } else {
        false
    }
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

fn calc_lv_or_read_again(dict: &'static Pokepedia, cp: &mut i32, ivs: &mut IVs) -> Option<f32> {
    loop {
        let lv = match calc_lv(dict, *cp, *ivs) {
            None => {
                let near_ivs = search_near_iv(dict, *cp, *ivs);

                if near_ivs.is_empty() {
                    eprintln!("CPか個体値の入力が間違っている。");

                    if !yes_or_no("CPと個体値を入力しなおす？(Yesならyを入力): ") {
                        return None;
                    }

                    *cp = read_cp();
                    *ivs = read_ivs();
                } else {
                    let mut msgs = vec![];
                    msgs.push(format!("{}: ポケモンレベルの取得に失敗(CP({})か個体値({:?})が間違っている)",
                              dict.name(), *cp, ivs.to_tuple()));

                    msgs.push("もしかして、この値?".to_string());

                    for ivs in near_ivs {
                        msgs.push(format!("{:?}", ivs.to_tuple()));
                    }

                    eprintln!("{}", msgs.join("\n"));

                    if !yes_or_no("個体値を入力しなおす？(Yesならyを入力): ") {
                        return None;
                    }

                    *ivs = read_ivs();
                }

                continue;
            },

            Some(lv) => lv,
        };

        return Some(lv);
    }
}
