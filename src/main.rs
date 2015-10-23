extern crate crypto;
extern crate rustc_serialize;

use std::env;
use std::fmt;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path;
use std::process;
use std::process::Command;

use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;

use rustc_serialize::base64::{ToBase64, STANDARD};

fn help() {
    process::exit(1)
}

fn git_cmd(args: &[str], mess: &str) -> String {
    let cmd = Command::new("git");
    for arg in args {
        cmd.add(arg);
    }
    let output = cmd.output().ok().expect(mess);
    String::from_utf8_lossy(&output.stdout).trim_right().to_string()
}

fn root_dir() -> String {
    git_cmd(vec![
        "rev-parse",
        "--show-toplevel"
    ], "Error running git, are you sure this is a git repo?")
}

fn git_dir() -> String {
    git_cmd(vec![
        "rev-parse",
        "--git-dir"
    ], "Error running git, are you sure this is a git repo?")
}

fn object_dir() -> PathBuf {
    Path::new(root_dir()).join(git_dir())
}

fn get_secret_key() -> String {
    env::var("AWS_SECRET_ACCESS_KEY").ok().expect("You must set AWS_SECRET_ACCESS_KEY and AWS_ACCESS_KEY_ID to use s3 as an endpoint.")
}

fn sign_request(request: &str) -> String {
    let mut hmac = Hmac::new(Sha1::new(), &get_secret_key().into_bytes());
    hmac.input(request.as_bytes());
    hmac.result().code().to_base64(STANDARD)
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