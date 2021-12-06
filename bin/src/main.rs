use std::{fs::File, io::Read};

use clap::{App, Arg, SubCommand};
use parser::ast::TopLevel;

fn main() {
    let matches = App::new("pootis name here")
        .author("krista-chan <qbotdev84@gmail.com>, spu7nix <main@spu7nix.net>, flow, camila314") // put emails here
        .version("v0.0.0")
        .about("Langjam0002 entry")
        .subcommand(
            SubCommand::with_name("build")
            .arg(
                Arg::with_name("path")
                .short("p")
                .required(true)
                .index(1) 
            )
            .arg(
                Arg::with_name("input")
                .short("i")
                .long("input")
                .takes_value(true)
                .required(true)
            )
        )
        .get_matches();
    
    if let Some(m) = matches.subcommand_matches("build") {
        let path = m.value_of("path").unwrap();
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // wait i can do FromStr for toplevel now // sure
        let parsed = contents.parse::<TopLevel>().unwrap(); // fancy
        dbg!(&parsed);
        let evaled = interpreter::interpret::interpret(parsed, interpreter::traits::Value::String(m.value_of("input").unwrap().to_string()));
        println!("Done!");
        dbg!(evaled.unwrap());
    }
}// lol
