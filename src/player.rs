use std::cmp::min;

#[derive(Debug)]
pub enum PlayerHealth {
    Alive { health: u32 },
    Dead { time_of_death: String },
}

impl PlayerHealth {
    pub fn amount(&self) -> u32 {
        if let PlayerHealth::Alive { health } = self {
            return *health;
        }

        0
    }

    pub fn damage(&mut self, damage: u32) -> (u32, bool) {
        match self {
            PlayerHealth::Alive { ref mut health } => {
                let damage_to_deal = min(damage, *health);

                *health -= damage_to_deal;

                if *health <= 0 {
                    *self = PlayerHealth::Dead {
                        time_of_death: String::from("Now"),
                    };
                }

                return (
                    damage_to_deal,
                    match self {
                        PlayerHealth::Alive { health: _ } => false,
                        PlayerHealth::Dead { time_of_death: _ } => true,
                    },
                );
            }
            _ => (0, true),
        }
    }
}

#[derive(Debug)]
pub struct Player {
    name: String,
    health: PlayerHealth,
}

impl Player {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn health(&mut self) -> &mut PlayerHealth {
        &mut self.health
    }

    pub fn new(name: &str) -> Player {
        Player {
            name: String::from(name),
            health: PlayerHealth::Alive { health: 32 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (Player) {
        (Player::new("Test"))
    }

    #[test]
    fn no_damage() {
        let (mut player) = setup();

        assert_eq!(player.health.damage(0), (0, false));
    }

    #[test]
    fn non_death_damage() {
        let (mut player) = setup();

        let damage = player.health.amount() - 1;
        assert_eq!(player.health.damage(damage), (damage, false));
    }

    #[test]
    fn exactly_death_damage() {
        let (mut player) = setup();

        let health_amount = player.health.amount();
        assert_eq!(player.health.damage(health_amount), (health_amount, true));
    }

    #[test]
    fn over_death_damage() {
        let (mut player) = setup();

        let health_amount = player.health.amount();
        assert_eq!(
            player.health.damage(health_amount * 2),
            (health_amount, true)
        );
    }

    #[test]
    fn correct_name() {
        let name = "Test name";
        let player = Player::new(name);
        assert_eq!(player.name(), name);
    }
}
