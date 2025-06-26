use std::{env, process::exit};
mod args_beta;

use args_beta::argument_parser;
use util::banner;

pub mod args;
mod core;
mod password;
mod util;

const VERSION: f64 = 0.1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut parsed_args = argument_parser(args).unwrap_or_else(|err: String| {
        banner(VERSION);
        println!("\n{}", err);
        exit(1);
    });
    // // println!("{:?}", config);
    let res: Result<String, String> = core::process_args(&mut parsed_args.0, &parsed_args.1);
    match res {
        Ok(valv) => {
            if valv != String::default() {
                println!("[+] {}", valv);
            }
        }
        Err(err) => {
            banner(VERSION);
            println!("ERROR: {}", err)
        }
    }
    //println!("{:?}", config);
    //banner(); // show the banner of this program
}
