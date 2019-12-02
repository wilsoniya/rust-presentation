// macro error handling

use rand;

fn do_something_dangerous() -> Result<u32, String> {
    let success: bool = rand::random();
    match success {
        true => Ok(rand::random()),
        false => Err(String::from("SYSTEM FAILURE")),
    }
}

fn main() -> Result<(), String>{
    let valid_result = do_something_dangerous()?;
    println!("dangerous result: {}", valid_result);
    Ok(())
}
