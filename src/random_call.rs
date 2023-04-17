use rand::prelude::*;

pub fn get_dice() -> i32 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..7); //[1,7)
    println!("난수 1~6: {}", random_number);
    
    random_number
}

fn _test() {}
