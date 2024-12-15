fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // Cannot assign twice to immutable variable
    println!("The value of x is: {}", x);

    // Shadowing
    /*
    We can shadow a variable to a different type, but when using mut, we can't change the type.
    */

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // let mut spaces = "   ";
    // spaces = spaces.len(); // expected `&str`, found `usize`
}
