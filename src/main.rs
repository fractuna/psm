mod args;
mod core;
mod password;
mod util;

use std::{collections::HashMap, env, process::exit};

use args::{argument_parser, ArgAction};
use util::Error;

use crate::util::banner;

const VERSION: &'static str = "3.0.5";

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check the arguments
    if args.len() <= 1 {
        banner(VERSION);
        exit(1);
    }

    let mut parsed_args: (HashMap<&'static str, ArgAction>, String) = argument_parser(args)
        .unwrap_or_else(|err: String| {
            Error(err.as_str());
            exit(1);
        });

    // println!("{:?}", &parsed_args);

    let result_document = args::createDoc(&parsed_args.0);
    let res: Result<String, String> = core::process_args(&mut parsed_args.0, &parsed_args.1);
    match res {
        Ok(valv) => {
            if valv != String::default() {
                println!("[+] {}", valv);
            }
        }
        Err(err) => {
            Error(err.as_str());
        }
    }
}
