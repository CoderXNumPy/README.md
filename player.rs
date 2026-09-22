struct Player {
    name: String,
    hp: i32,
}

impl Player {
    fn heal_player(&mut self, amount: i32) {
        self.hp = self.hp + amount;
    }

    fn damage_player(&mut self, damage: i32) {
        self.hp = self.hp - damage;
    }

    fn show_status(&self) {
        println!("{} ki HP: {}", self.name, self.hp);
    }
}

fn main() {
    let mut p1 = Player {
        name: String::from("Xylon"),
        hp: 100,
    };

    p1.heal_player(100);
    p1.show_status();

    p1.damage_player(100);
    p1.show_status();
}
