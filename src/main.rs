extern crate rusoto;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{stdin, stdout, Write, Read};
//{stdin, stdout, Write, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, exit};

fn help() {
    exit(1)
}

fn git_cmd(args: Vec<&str>, mess: &str) -> String {
    let output = Command::new("git")
                     .args(&args)
                     .output()
                     .ok()
                     .expect(mess);
    String::from_utf8_lossy(&output.stdout).trim_right().to_string()
}

fn root_dir() -> String {
    git_cmd(vec!["rev-parse", "--show-toplevel"],
            "Error running git, are you sure this is a git repo?")
}

fn git_dir() -> String {
    git_cmd(vec!["rev-parse", "--git-dir"],
            "Error running git, are you sure this is a git repo?")
}

fn object_dir() -> PathBuf {
    Path::new(&root_dir()).join(&git_dir())
}

fn add(paths: &[String]) {
    let mut path = PathBuf::from(root_dir());
    path.push(".gitattributes");
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
        if contents.contains(path) {
            continue;
        }
        write!(attrs, "{}\tfilter=data\tbinary\n", path)
            .ok()
            .expect("Couldn't write to .gitattributes.");
        attrs.sync_data().ok();
        Command::new("git").arg("add").arg(path).output().ok();
    }
}

fn clean() {
    Command::new("git")
        .args(&vec!["hash-object", "--stdin"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .ok()
        .expect("error running git hash-object");
}

fn smudge() {
    let slice = [0; 8];
    let header = stdin.read_exact(slice);
    if slice == &"git-data" {
       stdout().write(&slice).unwrap_or_else(|e| { exit(1) });
    }

}

fn init() {
    println!("init!")
}

fn sync() {
    println!("adding!")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
    }
    match args[1].as_ref() {
        "add" => add(&args[2..]),
        "filter-clean" => clean(),
        "filter-smudge" => smudge(),
        "init" => init(),
        "sync" => sync(),
        _ => help(),
    }
}
