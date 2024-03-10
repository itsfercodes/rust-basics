// Simple function
fn say_hello() {
    println!("Hello world!");
}

// Parameters
fn say_apples(apples: i32) {
    println!("I have {} apples", apples);
}

// Return values
fn double(x: i32) -> i32 {
    println!("I’m going to double {}", x);
    // If a block ends with an expression,
    // then evaluating the block results in the value of that expression
    x * 2
}

fn main() {
    say_hello();
    say_apples(10);
    println!("3 * 2 == {}", double(3));
}
