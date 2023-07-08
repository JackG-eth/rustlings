// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.


fn main() {
    call_me(3);
}

// must define the scope of the varaible when passing it in.
fn call_me(num: i8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
