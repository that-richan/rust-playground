use std::cmp::min;

#[derive(Debug)]
enum PlayerHealth {
    Alive { health: u32 },
    Dead { time_of_death: String },
}

#[derive(Debug)]
struct Player {
    name: String,
    health: PlayerHealth,
}

impl Player {
    fn damage(&mut self, damage: u32) -> (u32, bool) {
        match self.health {
            PlayerHealth::Alive { ref mut health } => {
                let damage_to_deal = min(damage, *health);

                *health -= damage_to_deal;

                if *health <= 0 {
                    self.health = PlayerHealth::Dead {
                        time_of_death: String::from("Now"),
                    };
                }

                return (
                    damage_to_deal,
                    match self.health {
                        PlayerHealth::Alive { health: _ } => false,
                        PlayerHealth::Dead { time_of_death: _ } => true,
                    },
                );
            }
            _ => (0, true),
        }
    }

    fn new(name: String) -> Player {
        Player {
            name,
            health: PlayerHealth::Alive { health: 32 },
        }
    }
}

fn main() {
    let mut player = Player::new("That Richan".to_string());

    let (damage, is_dead) = player.damage(32);
    println!("Dealt {} {} damage to {}", damage, is_dead, player.name);

    println!("{:?}", player);
}
