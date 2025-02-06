fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is ALSO a mutable reference to x

    *y = 10; // Modify x through y
    *z = 20; // Modify x through z (this is where it gets problematic)

    println!("x = {}", x); // This will output an unexpected result, likely 20 because the last write wins.
}