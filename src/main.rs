use std::{collections::HashMap, env, process::exit};
mod args_beta;

use args_beta::{argument_parser, ArgAction};
use util::banner;

pub mod args;
mod core;
mod password;
mod util;

const VERSION: &'static str = "1.0.3";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut parsed_args: (HashMap<&'static str, ArgAction>, String) = argument_parser(args)
        .unwrap_or_else(|err: String| {
            banner(VERSION);
            println!("\n{}", err);
            exit(1);
        });
    let result_document = args_beta::createDoc(&parsed_args.0);
    // println!("{result_document}");
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
            println!("\nERROR: {}", err)
        }
    }
}
