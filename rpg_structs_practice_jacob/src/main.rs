use std::io;
use std::io::Write;
use serde_derive::{Deserialize, Serialize};
use std::fs::OpenOptions;

/*
The purpose of this program will to be to practice using slices and structs
I will attempt to create a program that requests some information from the user,
and then puts that information into a struct, and pulls information from those structs

I'm going to try to build a very basic RPG fighter, where the player enters entity info, and the entered entities
will be saved to a json file.

*/
#[derive(Deserialize, Serialize, Debug)]
struct FightStats {
    max_health: u32,
    attack_damage: u32,
    attack_speed: f32,
}

#[derive(Deserialize, Serialize, Debug)]
struct Entity {
    name: String,
    description: String,
    stats: FightStats,
}

impl Entity {
    fn dps(&self) -> f32 {
        self.stats.attack_damage as f32 * self.stats.attack_speed
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Characters {
    entities: Vec<Entity>,
}

fn create_data_file_if_absent(data_path: &std::path::Path) {
    let prefix = data_path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    let file = OpenOptions::new().write(true).create(true).open(data_path);
}

fn get_character_data_from_file(data_path: &std::path::Path) -> Characters{
    let saved_characters_string = std::fs::read_to_string(&data_path).unwrap();
        
    if saved_characters_string.len() > 0 {
        serde_json::from_str::<Characters>(&saved_characters_string).unwrap()
    } else {
        Characters {
            entities: Vec::<Entity>::new()
        }
    }
}

fn main() {
    
    let data_path = std::path::Path::new("data/characters.json");
    create_data_file_if_absent(&data_path);

    let mut saved_characters = get_character_data_from_file(&data_path);
    
    let mut do_main_loop = true;

    while do_main_loop {
        println!("Welcome to super mega epic RPG fighter game");
        println!("There are 4 choices:");
        println!("0: add an entity");
        println!("1: edit an entity");
        println!("2: print the stored entities");
        println!("3: fight two entities");
        println!("4: close program");
        let choice = get_number::<u32>("that was not a valid int".to_string());
        match choice {
            0 => {saved_characters.entities.push(add_entity()); save_state(&data_path,&saved_characters)},
            1 => {edit_entity(&saved_characters); save_state(&data_path,&saved_characters)},
            2 => println!("Stored Characters are: {:#?}", saved_characters),
            3 => fight_two_entities(&saved_characters),
            4 => do_main_loop = false,
            _ => println!("That was not an integer from 0 to 4."),
        }
    }

    save_state(&data_path,&saved_characters);
}

fn save_state(data_path: &std::path::Path,characters: &Characters) {
    std::fs::write(
        data_path,
        serde_json::to_string_pretty(&characters).unwrap(),
    );
}

fn edit_entity(characters: &Characters) {

}

fn add_entity() -> Entity {

    println!("enter a name for the entity");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    name = name.trim().to_string();
    println!("enter a description for the entity");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");
    description = description.trim().to_string();
    println!("enter a maximum health");
    let max_health = get_number::<u32>("that was not a valid int".to_string());
    println!("enter an attack damage");
    let attack_damage = get_number::<u32>("that was not a valid int".to_string());
    println!("enter an attack speed");
    let attack_speed = get_number::<f32>("that was not a valid number".to_string());
    let stats = FightStats {
        max_health,
        attack_damage,
        attack_speed
    };

    return Entity {
        name,
        description,
        stats,
    }
}

fn fight_two_entities(characters: &Characters) {

}


// nabbed these from control_flow_jacob
fn please_press_enter() {
    let mut choice = String::new();
    print!("...");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
}

fn get_number<A: std::str::FromStr>(error_message: String) -> A{
    loop{
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            let choice: A = match choice.trim().parse()  {
                Ok(num) => num,
                Err(_) => {
                    println!("{}",error_message);
                    continue;
                },
            };
            return choice;
    }
}