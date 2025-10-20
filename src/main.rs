mod args;
mod core;
mod password;
mod util;

use std::{collections::HashMap, env, process::exit};

use args::{argument_parser, ArgAction};
use util::{Info, Warn, Error};


const VERSION: &'static str = "1.0.3";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut parsed_args: (HashMap<&'static str, ArgAction>, String) = argument_parser(args)
        .unwrap_or_else(|err: String| {
            // banner(VERSION);
            println!("\nERROR: {}", err);
            Error(err.as_str());
            exit(1);
        });
    // println!("{:?}", &parsed_args);
    let result_document = args::createDoc(&parsed_args.0);
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
            // banner(VERSION);
            Error(err.as_str());
            // println!("\nERROR: {}", err)
        }
    }
}
