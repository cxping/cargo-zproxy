use std::env;

fn main() {
    let  cdir = env::current_dir();
    println!("Hello, world!{:?}",cdir);
    let cargo_home = env::var("CARGO_HOME").unwrap();
    println!("CARGO_HOME:{:?}",cargo_home);
}



