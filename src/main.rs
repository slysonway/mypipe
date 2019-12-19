extern crate clap;

use clap::{App, Arg};
use std::process::Command;

fn main(){
    let matches = App::new("mypipe")
        .version("0.1.0")
        .about("Pipe PK")
        .author("Pierre Kettmus <pkettmus@gmail.com>")
        .arg(Arg::with_name("input")
            .short("i")
            .long("--input")
            .value_name("First argument")
            .help("Set an input argument")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("output")
            .short("o")
            .long("--output")
            .value_name("Second argument")
            .help("Set an output argument")
            .required(true)
            .takes_value(true)
        )
        .get_matches();

    let input = matches.value_of("input").unwrap().to_string();
    let out = matches.value_of("output").unwrap().to_string();

    let cmd_in = Command::new(input)
        .output()
        .expect("Error while processing the command");

    let get_std = String::from_utf8_lossy(&cmd_in.stdout).to_string();
    let cmd_out = Command::new(out)
        .arg(get_std)
        .output()
        .expect("Error while processing the command");

    println!("{}", String::from_utf8_lossy(&cmd_out.stdout).to_string());

}
