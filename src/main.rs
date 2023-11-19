mod pokemon_lv;
mod pokemon_info;
mod moves;
mod index;

fn main() {
    let m = pokemon_info::load_pokemon_info();
    let hapi = m.get("ハピナス").unwrap();
    println!("{:?}", hapi);

    let pl = pokemon_lv::get_pokemon_lv(1.5);
    println!("{:?}", pl);

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
