fn main() {
    println!("Hello, world");

    // Print a variable
    println!("Number: {}", 1);
    println!("{} is a great {}", "Fernando", "programmer");

    // Positional arguments
    println!(
        "Enhance your coding skills from {0} courses.  {0} courses are very {1}",
        "Educative", "interactive"
    );

    let name: &str = "Fernando";
    let age: u32 = 23;

    println!("My name is {} and I am {} years old", name, age);

    // Named arguments
    println!(
        "{company} provides {kind} courses",
        company = "Educative",
        kind = "interactive"
    );

    // Convert value to binary, hexadecimal and octal
    println!(
        "Number : 10 \nBinary:{:b} Hexadecimal:{:x} Octal:{:o}",
        10, 10, 10
    );
}
