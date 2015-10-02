use std::env;
use std::process;

fn help() {
    process::exit(1)
}

fn add() {
    println!("adding!")
}

fn clean() {
    println!("clean!")
}

fn smudge() {
    println!("smudge!")
}

fn init() {
    println!("init!")
}

fn sync() {
    println!("adding!")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { help(); }
    match args[1].as_ref() {
        "add" => add(),
        "filter-clean" => clean(),
        "filter-smudge" => smudge(),
        "init" => init(),
        "sync" => sync(),
        _ => help()
    }
}