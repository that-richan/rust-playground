use crate::player::Player;
use rand::Rng;
use std::collections::HashMap;
use std::{fs, io};

mod player;

fn main() {
    let players_file = "players.txt";
    let players: Vec<Player> = read_players_from_file(players_file)
        .expect(&format!("Failed to read '{players_file}' file"));

    let mut player_map: HashMap<String, Player> = HashMap::new();

    let string1 = String::from("Hello Nobr!");
    println!("{}", string1.replace("N", "B"));

    let first = players.get(0);
    if let Some(player) = first {
        println!("The first player is {}", player.name());
    }

    for player in players {
        player_map.insert(player.name().clone(), player);
    }

    if let Some(favorite) = player_map.get("That Richan") {
        println!("My favorite player: {:?}", favorite);
    }

    for (name, player) in &mut player_map {
        let damage_to_deal = rand::thread_rng().gen_range(1..=36);
        let (damage, is_dead) = player.health().damage(damage_to_deal);
        println!("Dealt {} damage to {}.", damage, name);
        if is_dead {
            println!("The player {} has died.", name);
        }
    }

    println!("{:?}", player_map);
}

fn read_players_from_file(file_name: &str) -> Result<Vec<Player>, io::Error> {
    let file_contents = fs::read_to_string(file_name)?;

    Ok(file_contents.lines().map(|s| Player::new(s)).collect())
}
