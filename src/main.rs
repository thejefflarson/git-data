use std::env;
use std::io::prelude::*;
use std::process;
use std::process::Command;
use std::fs::OpenOptions;
use std::path::PathBuf;

fn help() {
    process::exit(1)
}

fn root_dir() -> String {
    let output = Command::new("git")
                  .arg("rev-parse")
                  .arg("--show-toplevel")
                  .output()
                  .ok()
                  .expect("Error running git");
    if !output.status.success() {
        help();
    }
    String::from_utf8_lossy(&output.stdout).trim_right().to_string()
}

fn add(paths: &[String]) {
    let mut path = PathBuf::from(root_dir());
    path.push(".gitattributes");
    println!("{:?}", path);
    let mut attrs = OpenOptions::new()
                     .read(true)
                     .write(true)
                     .create(true)
                     .open(path)
                     .ok()
                     .expect("Couldn't access .gitattributes file.");

    let mut contents = String::new();
    attrs.read_to_string(&mut contents).ok();
    for path in paths {
        if contents.contains(path) { continue; }
        write!(attrs, "{}\tfilter=data\tbinary\n", path).ok().expect("Couldn't write to .gitattributes.");
        attrs.sync_data().ok();
        Command::new("git").arg("add").arg(path).output().ok();
    }
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
        "add" => add(&args[2..]),
        "filter-clean" => clean(),
        "filter-smudge" => smudge(),
        "init" => init(),
        "sync" => sync(),
        _ => help()
    }
}