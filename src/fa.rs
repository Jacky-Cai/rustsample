use std::env;

const X: i32 = 10;

pub fn print_aaa() {
    println!("{}", 42);
}

pub mod bbb {
    pub fn print_bbb() {
        println!("{}", 37);
    }
}

pub fn testIo() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args)
}
