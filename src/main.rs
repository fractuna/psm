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
    argument_parser(args);
    // let config: args::Config = args::parse_arguments(&args).unwrap_or_else(|err: String| {
    //     banner(VERSION);
    //     println!("\n{}", err);
    //     exit(1);
    //     //args::Config::default()
    // });
    // // println!("{:?}", config);
    // let res: Result<String, String> = core::process_args(&config);
    // match res {
    //     Ok(valv) => {
    //         if valv != String::default() {
    //             println!("[+] {}", valv);
    //         }
    //     }
    //     Err(err) => {
    //         banner(VERSION);
    //         println!("ERROR: {}", err)
    //     }
    // }
    //println!("{:?}", config);
    //banner(); // show the banner of this program
}
