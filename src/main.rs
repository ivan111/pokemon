mod pokepedia;
mod cpm;
mod types;
mod moves;
mod index;
mod pokemon;
mod battle;
mod ranking;
mod evolution;
mod utils;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write, BufReader};

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::pokepedia::Pokepedia;
use crate::pokemon::{Pokemon, IVs, calc_lv, search_near_iv};
use crate::moves::{FastMove, ChargeMove};
use crate::types::{NUM_TYPES, TYPE_NAMES, TYPES};
use crate::evolution::rev_evolutions;
use crate::utils::{jp_width, jp_fixed_width_string};

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

                    "ecp" => {
                        let pokemons = pdir.get(&cd).unwrap();
                        if let Some(poke) = select_pokemon(pokemons) {
                            print_ecp_table(poke);
                        }
                    },

                    "top_scp" => {
                        if let Some(dict) = pokepedia::skim_pokepedia() {
                            if let Some((_, lv, ivs)) = crate::index::calc_top_scp_iv_limited_by_cp(1500, 40.0, dict) {
                                let p = Pokemon::raw_new(dict, lv, ivs, dict.fast_moves()[0], dict.charge_moves()[0], None);
                                println!("{}", p.format(jp_width(dict.name())));

                                for child_dict in rev_evolutions(dict.no()) {
                                    let p = Pokemon::raw_new(child_dict, lv, ivs, child_dict.fast_moves()[0], child_dict.charge_moves()[0], None);
                                    println!("  {}", p.format(jp_width(child_dict.name())));

                                    for gc_dict in rev_evolutions(child_dict.no()) {
                                        let p = Pokemon::raw_new(gc_dict, lv, ivs, gc_dict.fast_moves()[0], gc_dict.charge_moves()[0], None);
                                        println!("    {}", p.format(jp_width(gc_dict.name())));

                                        for ggc_dict in rev_evolutions(gc_dict.no()) {
                                            let p = Pokemon::raw_new(ggc_dict, lv, ivs, ggc_dict.fast_moves()[0], ggc_dict.charge_moves()[0], None);
                                            println!("      {}", p.format(jp_width(ggc_dict.name())));
                                        }
                                    }
                                }
                            }
                        };
                    },

                    "ls_moves" => {
                        let pokemons = pdir.get(&cd).unwrap();
                        if let Some(poke) = select_pokemon(pokemons) {
                            ls_moves(&poke.move_perm());
                        }
                    },

                    "top" => {
                        let pokemons = pdir.get(&cd).unwrap();
                        top_ecp(pokemons);
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

                    "mkdir" => {
                        if words.len() == 2 {
                            let name = String::from(words[1]);

                            if pdir.contains_key(&name) {
                                eprintln!("既に存在するディレクトリ: {}", name);
                            } else {
                                pdir.insert(name.clone(), Vec::new());
                                changed_pdir.insert(name, true);
                            }
                        } else {
                            eprintln!("Usage: mkdir name");
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

                    "sim" => {
                        let pokemons = pdir.get(&cd).unwrap();
                        let opponents = pdir.get("sl_tr").unwrap();
                        let width = pokemons.iter().map(|p| jp_width(p.name())).max();
                        let len = opponents.len();

                        if let Some(width) = width {
                            for poke in pokemons {
                                let mut num_wins = [0; 3];

                                for p in opponents {
                                    let mut turns0: [i32; 3] = [0; 3];
                                    let mut turns1: [i32; 3] = [0; 3];
                                    let mut ratio: [f64; 3] = [0.0; 3];

                                    for i in 0..=2 {
                                        (_, turns0[i]) = poke.calc_power_per_turn(Some(p), None, i as i32);
                                        (_, turns1[i]) = p.calc_power_per_turn(Some(poke), None, i as i32);
                                        ratio[i] = turns1[i] as f64 / turns0[i] as f64;

                                        if turns0[i] < turns1[i] {
                                            num_wins[i] += 1;
                                        }
                                    }
                                }

                                let name = jp_fixed_width_string(poke.name(), width);
                                println!("{} {:2},  {:2},  {:2} / {}", name, num_wins[0], num_wins[1], num_wins[2], len);
                            }
                        }
                    },

                    "sim1" => {
                        let pokemons = pdir.get(&cd).unwrap();
                        if let Some(poke) = select_pokemon(pokemons) {
                            println!("{}", poke.format(jp_width(poke.name())));

                            let opponents = pdir.get("sl_tr").unwrap();
                            let width = opponents.iter().map(|p| jp_width(p.name())).max();

                            if let Some(width) = width {
                                let mut num_wins = [0; 3];
                                let len = opponents.len();

                                for p in opponents {
                                    let name = jp_fixed_width_string(p.name(), width);

                                    let mut turns0: [i32; 3] = [0; 3];
                                    let mut turns1: [i32; 3] = [0; 3];
                                    let mut ratio: [f64; 3] = [0.0; 3];

                                    for i in 0..=2 {
                                        (_, turns0[i]) = poke.calc_power_per_turn(Some(p), None, i as i32);
                                        (_, turns1[i]) = p.calc_power_per_turn(Some(poke), None, i as i32);
                                        ratio[i] = turns1[i] as f64 / turns0[i] as f64;

                                        if turns0[i] < turns1[i] {
                                            num_wins[i] += 1;
                                        }
                                    }

                                    let result;

                                    let r = &ratio[1];

                                    if *r >= 1.3 {
                                        result = "oooo";
                                    } else if *r >= 1.2 {
                                        result = "ooo ";
                                    } else if *r >= 1.1 {
                                        result = "oo  ";
                                    } else if *r >= 1.03 {
                                        result = "o   ";
                                    } else if *r >= 0.97 {
                                        result = "-   ";
                                    } else if *r >= 0.9 {
                                        result = "x   ";
                                    } else if *r >= 0.8 {
                                        result = "xx  ";
                                    } else if *r >= 0.7 {
                                        result = "xxx ";
                                    } else {
                                        result = "xxxx";
                                    }

                                    println!("{} {} [2] {}/{} = {:.2} [1] {}/{} = {:.2} [0] {}/{} = {:.2}", result, name,
                                             turns1[2], turns0[2], ratio[2],
                                             turns1[1], turns0[1], ratio[1],
                                             turns1[0], turns0[0], ratio[0]);
                                }

                                println!("wins {},  {},  {} / {}", num_wins[0], num_wins[1], num_wins[2], len);
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

fn ratio_to_rank(ratio: f64) -> &'static str {
    if ratio >= 1.3 {
        "oooo"
    } else if ratio >= 1.2 {
        "ooo "
    } else if ratio >= 1.1 {
        "oo  "
    } else if ratio >= 1.03 {
        "o   "
    } else if ratio >= 0.97 {
        "-   "
    } else if ratio >= 0.9 {
        "x   "
    } else if ratio >= 0.8 {
        "xx  "
    } else if ratio >= 0.7 {
        "xxx "
    } else {
        "xxxx"
    }
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
    if pokes.is_empty() {
        return;
    }

    let width = pokes.iter().map(|p| jp_width(p.name())).max().unwrap();

    for p in pokes {
        println!("{}", p.format(width));
    }
}

fn ls_moves(pokes: &[Pokemon]) {
    if pokes.is_empty() {
        return;
    }

    println!("{} CP {} SCP {}", pokes[0].name(), pokes[0].cp(), pokes[0].scp());
    println!("(PPT, EPT, turns) fast_move,  (PPE, turns) charge_move");

    let mut w_fm = 0;
    let mut w_cm1 = 0;
    let mut w_cm2 = 0;

    for p in pokes {
        w_fm = std::cmp::max(w_fm, jp_width(&p.fast_move_desc()));
        w_cm1 = std::cmp::max(w_cm1, jp_width(&p.charge_move1_desc()));
        w_cm2 = std::cmp::max(w_cm2, jp_width(&p.charge_move2_desc()));
    }

    let mut sorted = pokes.to_vec();

    sorted.sort_by_key(|p| -p.ecp(None, None, 1));

    for p in sorted {
        let ecp2 = p.ecp(None, None, 2);
        let ecp1 = p.ecp(None, None, 1);
        let ecp0 = p.ecp(None, None, 0);

        let fm = jp_fixed_width_string(&p.fast_move_desc(), w_fm);
        let cm1 = jp_fixed_width_string(&p.charge_move1_desc(), w_cm1);
        let cm2 = jp_fixed_width_string(&p.charge_move2_desc(), w_cm2);

        println!("ECP[2] {:>4} [1] {:>4} [0] {:>4},  {} {} {}", ecp2, ecp1, ecp0, fm, cm1, cm2);
    }
}

fn top_ecp(pokes: &[Pokemon]) {
    if pokes.is_empty() {
        return;
    }

    let width = pokes.iter().map(|p| jp_width(p.name())).max().unwrap();

    let mut sorted = pokes.to_vec();
    sorted.sort_by_key(|p| -p.ecp(None, None, 1));

    for p in sorted {
        let name = jp_fixed_width_string(p.name(), width);

        let ecp1 = p.ecp(None, None, 1);
        let fm = p.fast_move_desc();
        let cm1 = p.charge_move1_desc();
        let cm2 = p.charge_move2_desc();

        let mut perm = p.move_perm();
        perm.sort_by_key(|p| -p.ecp(None, None, 1));
        let top = &perm[0];

        let top_ecp1 = top.ecp(None, None, 1);
        let top_fm = top.fast_move_desc();
        let top_cm1 = top.charge_move1_desc();
        let top_cm2 = top.charge_move2_desc();

        println!("{} CP {} SCP {}", name, p.cp(), p.scp());
        println!("    cur ECP1 {:>4} {} {} {}", ecp1, fm, cm1, cm2);
        println!("    top ECP1 {:>4} {} {} {}", top_ecp1, top_fm, top_cm1, top_cm2);
        println!();
    }
}

fn print_ecp_table(poke: &Pokemon) {
    println!();

    println!("          |  ECP2   ECP1   ECP0");
    println!("-------------------------------");

    for i in 0..NUM_TYPES {
        let name = jp_fixed_width_string(TYPE_NAMES[i], 10);
        print!("{}| ", name);

        let types = vec![TYPES[i]];
        let ecp2 = poke.ecp(None, Some(types.clone()), 2);
        let ecp1 = poke.ecp(None, Some(types.clone()), 1);
        let ecp0 = poke.ecp(None, Some(types), 0);

        println!(" {:>4}   {:>4}   {:>4}", ecp2, ecp1, ecp0);
    }

    println!();
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

fn select_pokemon(pokemons: &[Pokemon]) -> Option<&Pokemon> {
    let width = pokemons.iter().map(|p| jp_width(p.name())).max();

    if let Some(width) = width {
        match pokemon::skim_pokemons(pokemons, width) {
            None => None,
            Some(i) => {
                Some(&pokemons[i])
            }
        }
    } else {
        None
    }
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

        print!("IVs(atk def sta): ");
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
