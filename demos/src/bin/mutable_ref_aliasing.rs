fn main() {
    let mut string = String::from("the rain in Spain");
    let _r1 = &string;
    let mut _mr1 = &mut string;
    println!("Hello, world! {}", _r1);
}
