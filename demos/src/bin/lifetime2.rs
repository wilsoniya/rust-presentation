fn get_best_number<'a>() -> &'a u8 {
    let age = 2;
    &age
}

fn main() {
    println!("The best number is: {}", get_best_number());
}

