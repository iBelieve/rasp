extern crate rasp;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use rasp::read_and_eval;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() >= 1 {
        for arg in args {
            let mut file = File::open(arg)
                .expect("Unable to find file");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Unable to read file");
            read_and_eval(&contents);
        }
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)
            .expect("Unable to read input from stdin");
        read_and_eval(&buffer);
    }
}
