
#[derive(Debug)]
enum Weapon {

    Sword(i32),
    Mace(i32),
    Bow(i32),
    
}
struct Player {
    name: String,
    hp: i32,
    weapon: Weapon,
    
}
impl Player {
    fn attack(&self, target: &mut Player) {
        target.hp = target.hp - 30;
        println!("{} Attacks {}",self.name, target.name);
        
        
    }
    fn is_alive(&self ) -> bool {
        self.hp > 0
        
    }
    fn show_status(&self) {
        println!("{} HP {}",self.name, self.hp);
        
        
    }
}
fn main() {
    let mut p1 = Player {
        name: String::from("XyLon"),
        hp: 100,
        weapon: Weapon::Mace(30),
        
    };
    let mut p2 = Player {
        name: String::from("Steve"),
        hp: 90,
        weapon: Weapon::Bow(20),
        
    };
    loop {
        p1.attack(&mut p2);
        p2.show_status();
        
        if !p2.is_alive() {
            println!("Wins {}",p1.name);
            break;
            
        }
        p2.attack(&mut p1);
        p1.show_status();
        
        if !p1.is_alive() {
            println!("Wins {}",p2.name);
            break;
        }
    }
}


 
