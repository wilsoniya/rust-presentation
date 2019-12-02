// combinator error handling

use rand;

fn do_something_dangerous() -> Result<u32, String> {
    let success: bool = rand::random();
    match success {
        true => Ok(rand::random()),
        false => Err(String::from("SYSTEM FAILURE")),
    }
}

fn main() -> Result<(), String>{
    do_something_dangerous()
        .map(|valid_result| println!("dangerous result: {}", valid_result))
}
