fn max<'a, T: Ord>(first: &'a T, second: &'a T) -> &'a T {
    if first >= second {
        first
    } else {
        second
    }
}

fn main() {
    let (a, b) = (5, 10);
    println!("max({}, {}) = {}", a, b, max(&a, &b));
}
