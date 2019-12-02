struct Person {
    name: String,
    age: u8,
}

fn main() {
    let stimpy = Person {
        name: String::from("Stimpson J. Cat"),
        age: 2,
    };

    stimpy.name = String::from("Ren");
}
