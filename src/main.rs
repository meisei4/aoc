#[derive(Clone)]
struct Player {
    name: String,
    lvl: u32,
    exp: u32,
    power_up: Option<String>,
}

impl Player {
    fn new(_name: &str, _level: u32, _experience: u32) -> Self {
        Self {
            name: _name.to_string(),
            lvl: _level,
            exp: _experience,
            power_up: None,
        }
    }

    fn _change_name(mut self, _name: &str){
        self.name = _name.to_string();
    }

    fn show_stats(&self) {
        println!("[Show Stats] {} - Level: {}, XP: {}", self.name, self.lvl, self.exp);
    }

    fn level_up(&mut self) {
        self.lvl += 1;
        self.exp = 0;
    }

    fn gain_exp(&mut self, points: u32) {
        let multiplier = match &self.power_up {
            Some(ability) => {
                match ability.as_str() {
                    "XPMultiplier" => 2, // x2 multiplier
                    _ => 1, // Default multiplier for unrecognized abilities
                }
            },
            None => 1,
        };
        self.exp += points * multiplier;
        println!(
            "{} gains {} experience points (multiplier: {}). Total XP: {}",
            self.name, points * multiplier, multiplier, self.exp
        );
    }

    fn save_player_data(self) {
        println!("[Save Data] {}'s data has been saved. Current stats:", self.name);
        self.show_stats();
    }

    fn level_up_and_save(&mut self) {
        println!("[Method: level_up_and_save] Attempting to level up and save {}.", self.name);
        self.level_up();
        // MOVE ERROR!
        //self.save_player_data();
    }

    fn set_power_up(&mut self, ability: String) {
        self.power_up = Some(ability);
    }
}

fn main() {
    let mut player1 = Player::new("player_one", 1, 0);
    player1.show_stats();

    player1.gain_exp(100);
    player1.show_stats();

    player1.level_up();
    player1.show_stats();

    let player_name = &player1.name; // Immutable borrow (active)
    //player1.gain_exp(200);           // Mutable borrow (conflict)
    player1.show_stats();            // only a & reference (no-conflict)
    // CANNOT HAVE BOTH IMMUTABLE AND MUTABLE ACTIVE AT THE SAME TIME
    println!("check out this immutable borrow of players's name: {}", player_name);

    save_game(player1);
    // FIX: uncomment the following lines and comment the above line
    //let player1_clone = player1.clone();
    //save_game(player1_clone);

    // MOVE ERROR!
    //player1.show_stats();

    let mut player2 = Player::new("player_two", 1, 0);
    special_level_up(&mut player2);
    player2.show_stats();

    let mut player3 = Player::new("player_three", 1, 0);
    player3.level_up_and_save();

    let mut player4 = Player::new("player_four", 3, 300);
    // Power up is not set so panic occurs
    // let ability_panic = player5.power_up.clone().expect("Player_five should have a power-up ability");
    // println!("{} has acquired the ability using expect: {}", player5.name, ability_panic);

    player4.gain_exp(50); // Should use default multiplier

    let mut player5 = Player::new("player_five", 2, 150);
    player5.show_stats();
    player5.set_power_up("XPMultiplier".to_string());

    // Using unwrap (Avoids panic)
    let ability_unwrapped = player5.power_up.clone().unwrap();
    println!("{} has acquired the ability using unwrap: {}", player5.name, ability_unwrapped);

    // Using expect (will panic with a custom message if power_up is None)
    let ability_expected = player5.power_up.clone().expect("Player should have a power-up ability");
    println!("{} has acquired the ability using expect: {}", player5.name, ability_expected);

    player5.gain_exp(50); // Should apply multiplier based on "XPMultiplier"

}

fn save_game(player: Player) {
    println!("[Function: save_game] Saving {} to the game database...", player.name);
    player.save_player_data();
    // MOVE ERROR!
    //player._change_name("new_name")
    // MOVE ERROR!
    //println!("[Function: save_game] {} has been successfully saved to the game.\n", player.name);
}

fn special_level_up(player: &mut Player) {
    println!("[Function: special_level_up] Applying special level up to {}.", player.name);
    player.gain_exp(200);
    player.level_up();
    //MOVE ERROR!
    //save_game(*player);
}
