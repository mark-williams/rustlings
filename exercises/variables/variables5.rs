// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
    let mut number = "23-1900";
    println!("Number {}", number);
    {
        // This block takes the outer var and 'redeclares' it
        // in the scope of this block
        let number = 1234;
        println!("Number {}", number);
    }
}
