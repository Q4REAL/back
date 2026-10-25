use std::collections::HashMap;
use std::{fs, io, path::Path};
use std::io::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fs::OpenOptions;
use std::io::ErrorKind;
use dirs::home_dir;
use crate::eval::Word;

pub fn open_dirs(path: &Path, dictionary: &mut HashMap<String, Vec<String>>) -> io::Result<()> {
    for node in fs::read_dir(path)? {
        match node {
            Ok(n) => {
                let p = n.path();
                if p.is_dir() {
                    match open_dirs(&p, dictionary) {
                        Ok(_) => {},
                        Err(e) => println!("Error: {}", e),
                    }
                } else {
                    let pf = p.clone();
                    println!("{}", pf.display());
                    if pf.exists() && pf.is_file() {
                        open_file(&p, None, dictionary);
                    }
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
    Ok(())
}

pub fn open_file(path: &Path, request: Option<&str>, dictionary: &mut HashMap<String, Vec<String>>) {
    match home_dir() {
        Some(home) => {
            match File::open(path) {
                Ok(f) => {
                    let read = BufReader::new(f);
                    let mut run = true;
                    let def = &mut Word::blank();
                    for (index, result) in read.lines().enumerate() {
                        match result {
                            Ok(line) => {
                                let mut i = 0;
                                for word in line.split_whitespace() {
                                    if i != 0 && i != line.split_whitespace().collect::<Vec<&str>>().len() - 1 {
                                        if i == 1 {
                                            if !request.is_none() {
                                                if word == request.unwrap() {
                                                    def.name = word.to_string();
                                                    run = false
                                                }
                                            } else {
                                                def.name = word.to_string();
                                            }
                                        } else {
                                            def.data.push(word.to_string())
                                        }
                                    }
                                    i += 1;
                                }
                                if def.name.is_empty() {
                                    *def = Word::blank()
                                } else {
                                    dictionary.insert(format!("{}/{}", path.strip_prefix(home.join(".backdeps/")).unwrap().with_extension("").display(), def.name.clone()), def.data.clone());
                                }
                                if !run {
                                    break
                                }
                            }
                            Err(e) => {println!("Error on line {}: {}", index, e); run = false}
                        } 
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        None => println!("Error: home dir could not be determined")
    }
}


pub fn process_dep(dep_path: &Path) -> io::Result<()> {
    let op_home = home_dir();
    if dep_path.is_dir() && op_home != None {
        let home = op_home.unwrap();
        for node in fs::read_dir(dep_path)? {
            match node {
                Ok(n) => {
                    let p = n.path();
                    if p.is_dir() {
                        fs::create_dir_all( home.join(format!(".backdeps/{}", p.parent().unwrap().strip_prefix(&home).unwrap().display())))?;
                        process_dep(&p)?
                    } else {
                        if p.extension().unwrap() == "back" { 
                            match File::open(&p) {
                                Ok(f) => {
                                    fs::create_dir_all( home.join(format!(".backdeps/{}", p.parent().unwrap().strip_prefix(&home).unwrap().display())))?;
                                    parse_file(f, Path::new(&home.join(format!(".backdeps/{}", p.strip_prefix(&home).unwrap().display()))));
                                },
                                Err(e) => println!("Error: {}", e),
                            };
                        }
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        Ok(())
    } else if dep_path.is_file() && op_home != None {
        if dep_path.extension().unwrap() == "back" {
            let home = op_home.unwrap();
            match File::open(&dep_path) {
                Ok(f) => {
                    fs::create_dir_all( home.join(format!(".backdeps/{}", dep_path.parent().unwrap().strip_prefix(&home).unwrap().display())))?;
                    parse_file(f, Path::new(&home.join(format!(".backdeps/{}", dep_path.strip_prefix(&home).unwrap().display()))));
                },
                Err(e) => println!("Error: {}", e),
            };
            Ok(())
        } else {
            println!("Error: file is not a back file");
            Ok(())
        }
    } else {
        println!("Error: Path does not lead to an acessable Dir or File, or the home directory could not be determined");
        Ok(())
    }
}


fn parse_file(file: File, path: &Path) {
    let read = BufReader::new(file);
    let mut def = String::new();
    let mut blk = Vec::new();
    let mut collecting = false;
    for (index, result) in read.lines().enumerate() {
        match result {
            Ok(line) => {
                for word in line.split_whitespace() {
                    if collecting {
                        match word {
                            ";" => match blk.pop() {
                                Some(_) => {
                                    def.push_str(format!("{} ", word).as_str());
                                }
                                None => {
                                    if def.split_whitespace().collect::<Vec<&str>>().len() >= 1 && def.len() > 0 {
                                        let file: Option<File> = match OpenOptions::new()
                                                .write(true)
                                                .create_new(true)
                                                .append(true)
                                                .open(format!("{}", path.display()))
                                            {
                                                Ok(f) => Some(f),
                                                Err(e) if e.kind() == ErrorKind::AlreadyExists => Some(OpenOptions::new()
                                                        .write(true)
                                                        .append(true)
                                                        .open(format!("{}", path.display())).unwrap()),
                                                Err(e) => {println!("Error: {}", e); None}
                                            };
                                        if !file.is_none() {
                                            match write!(file.unwrap(), "{}", format!(": {};\n", def)) {
                                                Ok(_) => {println!("{}", format!(": {} ;\n", def))},
                                                Err(e) => println!("Error writing to file {}: {}", path.display(), e)
                                            }
                                        }
                                        def = String::new();
                                        collecting = false;
                                    } else {
                                        def = String::new();
                                        collecting = false;
                                    }
                                }
                            },
                            "&" | "?" | ":" => {
                                blk.push("blk".to_string());
                                def.push_str(format!("{} ", word).as_str());
                            }
                            _ => {
                                def.push_str(format!("{} ", word).as_str()) 
                            }
                        }
                    } else {
                        if word == ":" {
                            collecting = true
                        }
                    }
                }
            }
            Err(e) => println!("Error on line {} in file {}: {}", index, path.display(), e)
        }
    }
}