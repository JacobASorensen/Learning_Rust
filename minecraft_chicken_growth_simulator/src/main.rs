
/*
Chicken Info: (pulled from Minecraft wiki)

Each Chicken has EggLayTime which is randomly set to a random value between 6000 and 12000 
When they lay an egg this value gets decremented on every tick

each egg has a 1/8 chance of spawning a baby
each successful attempt has a further 1/32 to spawn 3 extra baby chickens

Baby chickens take 24000 ticks to grow up

Ticks per second: 20

*/

use rand::prelude::*;
use std::io;

const BABY_GROW_TIME: u16 = 24000;

fn main() {

    let mut egg_thread = thread_rng();
    let mut delay_thread = thread_rng();

    println!("How many starting chickens do you want?");
    let starting_chickens: usize = get_number::<usize>("That was not a valid size".to_string());

    println!("How many ending chickens do you want?");
    let ending_chickens: usize = get_number::<usize>("That was not a valid size".to_string());    

    let mut adults: Vec<u16> = Vec::new();
    let mut babies: Vec<u16> = Vec::new();
    let mut a_size = adults.len();
    let mut b_size;

    for _i in 0..starting_chickens {
        adults.push(do_new_egg_delay(&mut delay_thread));
    }

    let mut new_chicks = 0;
    let mut new_adults = 0;

    let mut tick: u128 = 0;
    let mut minute: u128;
    while a_size < ending_chickens {

        for i in 0..adults.len() {
            adults[i] -= 1;
            if adults[i] == 0 {
                new_chicks += do_egg_lay(&mut egg_thread);
                adults[i] = do_new_egg_delay(&mut delay_thread);
            }
        }

        let mut baby_itr = 0;
        while baby_itr < babies.len() {
            babies[baby_itr] -= 1;
            if babies[baby_itr] == 0 {
                babies.swap_remove(baby_itr);
                new_adults += 1;
            } else {
                baby_itr += 1;
            }
        }

        for _i in 0..new_chicks {
            babies.push(BABY_GROW_TIME);
        }

        for _i in 0..new_adults {
            adults.push(do_new_egg_delay(&mut delay_thread));
        }
        
        tick += 1;
        minute = tick/1200;
        new_chicks = 0;
        new_adults = 0;
        a_size = adults.len();
        b_size = babies.len();

        print!("\rTick: {tick} Minute: {minute} Adults: {a_size} Babies: {b_size}");
        
    }
    print!("\n");
}

fn do_new_egg_delay( delay_thread: &mut ThreadRng) -> u16 {
    return delay_thread.gen_range(6000..12000);
}

fn do_egg_lay(egg_thread: &mut ThreadRng) -> u8 {
    if 0 == egg_thread.gen_range(0..8) {
        if 0 == egg_thread.gen_range(0..32) {
            return 4;
        } else {
            return 1;
        }
    } else {
        return 0;
    }
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