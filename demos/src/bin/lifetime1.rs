fn main() {
    let reference;

    {
        let value = 42;
        reference = &value;
    }

    println!("The answer to life, the universe and everything: {}", reference);
}
