use std::collections::HashMap;
mod eval;
mod dep_man;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "back", about = "the back programming language interpreter")]
struct Args {
    #[command(subcommand)]
    sub: Subs,
}

#[derive(Subcommand)]
enum Subs {
    Run {
        #[arg(short, long)]
        file: String,
    },
}

fn main() {
    let args = Args::parse();
    match args.sub {
        Subs::Run { file } => {
            let mut dictionary: HashMap<String, Vec<String>> = HashMap::new();
            let mut stacks: HashMap<String, Vec<String>> = HashMap::new();
            let mut wstack: String = "main".to_string();
            let mut run: bool;
            let mut ifd: Vec<String> = Vec::new();
            let mut rep: Vec<String> = Vec::new();
            let mut mode: eval::Mode = eval::Mode::Normal;
            let mut def: eval::Word = eval::Word::blank();
            let verbose = false;
            eval::init(&mut stacks, &mut dictionary);
            
            let f = match File::open(file) {
                Ok(f) => f,
                Err(e) => {println!("Error: {}", e); return;},
            };
            let read = BufReader::new(f);
            
            for (index, result) in read.lines().enumerate() {
                match result {
                    Ok(line) => {
                        for item in line.split_whitespace().collect::<Vec<_>>() {
                            (run, wstack) = eval::eval(
                                item,
                                &mut stacks,
                                wstack,
                                &mut ifd,
                                &mut rep,
                                &mut dictionary,
                                &mut mode,
                                &mut def,
                                verbose,
                            );
                            if !run {
                                break;
                            }
                        }
                    }
                    Err(e) => {println!("Error on line {}: {}", index + 1, e); return;}
                }
            }
            if verbose {
                println!("{:?}", stacks);
            }
        }
    }
}
