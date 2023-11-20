mod pokepedia;
mod cpm;
mod moves;
mod index;
mod pokemon;

use std::fs::File;
use std::io::BufReader;

fn main() {
    let f = File::open("mypokemon.json").unwrap();
    let mut reader = BufReader::new(f);
    let pokemons = pokemon::load_pokemon(&mut reader).unwrap();
    println!("{:?}", pokemons[1]);

    let m  = pokepedia::get_pokepedia_by_name();
    let koko = m.get("ココロモリ").unwrap();
    println!("{:?}", koko);

    let cpm = cpm::get_cpm(1.5);
    println!("{:?}", cpm);

    let fmoves_no_map = moves::get_fast_move_map_by_no();
    let m1 = fmoves_no_map.get("216").unwrap();
    println!("{:?}", m1);

    let fmoves_name_map = moves::get_fast_move_map_by_name();
    let m2 = fmoves_name_map.get("でんこうせっか").unwrap();
    println!("{:?}", m2);

    let cmoves_no_map = moves::get_charge_move_map_by_no();
    let m3 = cmoves_no_map.get("316").unwrap();
    println!("{:?}", m3);

    let cmoves_name_map = moves::get_charge_move_map_by_name();
    let m4 = cmoves_name_map.get("エアロブラスト").unwrap();
    println!("{:?}", m4);
}
