extern crate colored;
extern crate exitfailure;
extern crate failure;
extern crate structopt;

use colored::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;
use std::io::{self, Read};

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Moo!")]
    /// What does the cow say?
    message: String,
    #[structopt(short = "d", long = "dead")]
    dead: bool,
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// load the cat picture fro the specified file
    horsefile: Option<std::path::PathBuf>,
    #[structopt(short = "i", long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool,
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    let mut message = String::new();

    if options.stdin {
        io::stdin().read_to_string(& mut message)?;
    } else {
        message = options.message;
    }

    let eye = if options.dead { "x" } else { "o" };

    println!("{}", message.bright_yellow().underline().on_purple());

    match &options.horsefile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|_| format!("could not read file {:?}", path))?;
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", &cat_picture);
            Ok(())
        }
        None => {
            println!("{}", message.bright_yellow().underline().on_purple());
            println!(" \\");
            println!(" \\");
            println!(" /\\_/\\");
            println!(" ( {eye} {eye} )", eye = eye.to_string().red().bold()); 
            println!("  | o |=");
            println!("  |===|");
            Ok(())
        }
    }
}
