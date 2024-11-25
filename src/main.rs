#[derive(Clone)]
struct Player {
    name: String,
    lvl: u32,
    exp: u32,
}

impl Player {
    fn new(_name: &str, _level: u32, _experience: u32) -> Self {
        Self {
            name: _name.to_string(),
            lvl: _level,
            exp: _experience,
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
        self.exp += points;
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