use std::collections::HashMap;
mod eval;
mod dep_man;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use clap::{Parser, Subcommand};
use dirs::home_dir;

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

        #[arg(short, long)]
        verbose: bool,
    },
    Add {
        #[arg(short, long)]
        path: String,
    },
}

fn main() {
    let args = Args::parse();
    match args.sub {
        Subs::Run { file , verbose} => {
            let mut dictionary: HashMap<String, Vec<String>> = HashMap::new();
            let mut stacks: HashMap<String, Vec<String>> = HashMap::new();
            let mut wstack: String = "main".to_string();
            let mut run: bool = true;
            let mut ifd: Vec<String> = Vec::new();
            let mut rep: Vec<String> = Vec::new();
            let mut mode: eval::Mode = eval::Mode::Normal;
            let mut def: eval::Word = eval::Word::blank();
            eval::init(&mut stacks, &mut dictionary);
            
            let f = match File::open(file) {
                Ok(f) => f,
                Err(e) => {println!("Error: {}", e); return;},
            };
            let read = BufReader::new(f);
            
            for (index, result) in read.lines().enumerate() {
                match result {
                    Ok(line) => {
                        for item in line.split_whitespace() {
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
                                println!("line {}, word: {}", index + 1, item);
                                break;
                            }
                        }
                    }
                    Err(e) => {println!("Error on line {}: {}", index + 1, e); return;}
                }
                if !run {
                    break;
                }
            }
            if verbose {
                println!("{:?}", stacks);
            }
        },
        Subs::Add {path} => {
            let path = Path::new(&path);
            match home_dir() {
                Some(home) => if !(Path::new(&home.join(format!(".backdeps/{}", path.file_name().unwrap().display()))).exists()) {
                    match dep_man::process_dep(path) {
                        Ok(_) => (),
                        Err(e) => println!("Error: {}", e),
                    }
                    } else { 
                        println!("Error: dependency of this name already exists")
                    }
                None => println!("Error: the home directory could not be determined")
            }
        }
    }
}
