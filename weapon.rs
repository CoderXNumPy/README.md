#[derive(Debug)]
enum Weapon {
    Sword,
    Mace,
    Bow,
}

struct Player {
    name: String,
    hp: i32,
    weapon: Weapon,
}

fn check_weapon(weapon_type: &str) {
    match weapon_type {
        "sword" => println!("Close range"),
        "mace" => println!("Mace Attack L"),
        _ => println!("Unknown weapon category"),
    }
}

fn main() {
    let mut p1 = Player {
        name: String::from("Alex"),
        hp: 100,
        weapon: Weapon::Mace,
    };
    
    println!("{} has {}hp and Weapon {:?}", p1.name, p1.hp, p1.weapon);
    
    check_weapon("mace");
}
