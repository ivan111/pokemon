mod pokepedia;
mod cpm;
mod types;
mod moves;
mod index;
mod pokemon;
mod battle;
mod ranking;

use std::fs::File;
use std::io::BufReader;

fn main() {
    let mut file_name = dirs::home_dir().unwrap();
    file_name.push("mypokemon.json");
    println!("load {:?}", file_name);

    let f = File::open(file_name).unwrap();
    let mut reader = BufReader::new(f);
    let pokemons = pokemon::load_pokemon(&mut reader).unwrap();

    //let team1 = vec![pokemons[0].clone(), pokemons[1].clone(), pokemons[2].clone()];
    let team1 = vec![pokemons[0].clone()];
    //let team2 = vec![pokemons[0].clone(), pokemons[1].clone(), pokemons[2].clone()];
    let team2 = vec![pokemons[2].clone()];
    let mut battle = battle::Battle::new("たけし".to_string(), team1, "さとし".to_string(), team2);

    //battle.start();

    for p in &ranking::scp_ranking(1500, 40.0)[..10] {
        let s = p.stats();
        println!("{} SCP={}, atk={:.2}, def={:.2}, hp={}", p.name(), p.scp(), s.attack, s.defense, p.hp());
    }
}
