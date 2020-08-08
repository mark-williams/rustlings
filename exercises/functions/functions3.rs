// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

fn main() {
    call_me(3);
}

fn call_me(num: i32) {
    let max: i32 = 10 * num;
    for i in 0..max {
        println!("Ring! Call number {}", i + 1);
    }
}
