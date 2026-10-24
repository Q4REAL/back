use std::collections::HashMap;
use std::io;
use std::io::Write;

mod archive {
    pub const CONTINUE: &'static str = r##"

    [ back_rep ] $

    [ 1 ] &
        [ continue~the~program?~(y/n): ] ->>
        <- [ y ] = ?
            [ continuing... ] ->>
            ! [ -1 ]
        ;
    ;
    [ ending. ] ->>

    "##;
    
    pub const DEFS: &'static str = r##"

    : def_1 [ 2 ] : def_2 ** : def_3 * ; def_3 ; def_2 ;
    
    def_1 ->>
    [ ~ ] ->>
    
    : recursive 
        ** [ 1 ] = ?
            [ done! ] ->>
        ;
        ** [ 1 ] != ?
            [ still~going... ] ->>
            [ 1 ] -
            recursive
        ;
    ;
        
    [ 5 ] recursive
    "##;
}

enum Mode {
    Normal,
    Comment,
    Define,
    Rep,
    If(bool),
    Push,
}

#[derive(Debug)]
struct Word {
    name: String,
    data: Vec<String>,
}

impl Word {
    fn blank() -> Word {
        Word {
            name: "".to_string(),
            data: Vec::new(),
        }
    }
}

fn main() {
    let mut dictionary: HashMap<String, Vec<String>> = HashMap::new();
    let mut stacks: HashMap<String, Vec<String>> = HashMap::new();
    let mut wstack: String = "main".to_string();
    let mut run: bool;
    let mut ifd: Vec<String> = Vec::new();
    let mut rep: Vec<String> = Vec::new();
    let mut mode: Mode = Mode::Normal;
    let mut def: Word = Word::blank();
    let program: String = archive::DEFS.to_string();
    let verbose = false;
    
    {
    stacks.insert("main".to_string(), Vec::new());
    stacks.insert("back_rep".to_string(), Vec::new());
    stacks.insert("back_block".to_string(), Vec::new());

    dictionary.insert("->".to_string(), Vec::new());
    dictionary.insert("->>".to_string(), Vec::new());
    dictionary.insert("<-".to_string(), Vec::new());
    dictionary.insert("=>".to_string(), Vec::new());
    dictionary.insert("=>>".to_string(), Vec::new());
    dictionary.insert("<<-".to_string(), Vec::new());
    dictionary.insert("**".to_string(), Vec::new());
    dictionary.insert("..".to_string(), Vec::new());
    dictionary.insert("+".to_string(), Vec::new());
    dictionary.insert("-".to_string(), Vec::new());
    dictionary.insert("*".to_string(), Vec::new());
    dictionary.insert("/".to_string(), Vec::new());
    dictionary.insert(">".to_string(), Vec::new());
    dictionary.insert("<".to_string(), Vec::new());
    dictionary.insert("=".to_string(), Vec::new());
    dictionary.insert("!=".to_string(), Vec::new());
    dictionary.insert("?".to_string(), Vec::new());
    dictionary.insert("$".to_string(), Vec::new());
    dictionary.insert("$$".to_string(), Vec::new());
    dictionary.insert("$>".to_string(), Vec::new());
    dictionary.insert("!".to_string(), Vec::new());
    dictionary.insert("&".to_string(), Vec::new());
    dictionary.insert("[".to_string(), Vec::new());
    }

    for item in program.split_whitespace().collect::<Vec<_>>() {
        (run, wstack) = eval(
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
    if verbose {
        println!("{:?}", stacks);
    }
}

fn eval<'a>(
    word: &'a str,
    stacks: &mut HashMap<String, Vec<String>>,
    wstack: String,
    ifd: &mut Vec<String>,
    rep: &mut Vec<String>,
    dictionary: &mut HashMap<String, Vec<String>>,
    mode: &mut Mode,
    def: &mut Word,
    verbose: bool,
) -> (bool, String) {
    if verbose {
        println!(
            "{:?} ({}) [ifd: {:?}, rep: {:?}, {:?}] -> {}",
            stacks, wstack, ifd, rep, def, word
        );
    }
    match mode {
        Mode::Normal => match word {
            "->" => match stacks.get_mut(&wstack).unwrap().pop() {
                Some(v) => {
                    let nv = v.replace("~", " ");
                    print!("{}", nv);
                    (true, wstack)
                }
                None => {
                    println!("\n---");
                    println!("Error: stack underflow");
                    (false, wstack)
                }
            }, // print
            "=>" => match stacks.get_mut(&wstack).unwrap().pop() {
                Some(v) => {
                    print!("{}", v);
                    (true, wstack)
                }
                None => {
                    println!("\n---");
                    println!("Error: stack underflow");
                    (false, wstack)
                }
            }, // litteral print
            "->>" => match stacks.get_mut(&wstack).unwrap().pop() {
                Some(v) => {
                    let nv = v.replace("~", " ");
                    println!("{}", nv);
                    (true, wstack)
                }
                None => {
                    println!("\n---");
                    println!("Error: stack underflow");
                    (false, wstack)
                }
            }, // println
            "=>>" => match stacks.get_mut(&wstack).unwrap().pop() {
                Some(v) => {
                    println!("{}", v);
                    (true, wstack)
                }
                None => {
                    println!("\n---");
                    println!("Error: stack underflow");
                    (false, wstack)
                }
            }, // literal println
            "<-" => {
                let mut inp = String::new();

                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut inp).unwrap();

                stacks
                    .get_mut(&wstack)
                    .unwrap()
                    .push(inp.replace(" ", "~").trim().to_string());
                (true, wstack)
            } // input
            "<<-" => {
                let mut inp = String::new();

                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut inp).unwrap();

                for item in inp.split_whitespace().collect::<Vec<_>>() {
                    stacks.get_mut(&wstack).unwrap().push(item.to_string());
                }
                (true, wstack)
            } // literal input
            "**" => {
                let mut run = true;
                let val = match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => v,
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                        "".to_string()
                    }
                };
                if run == false {
                    (false, wstack)
                } else {
                    stacks.get_mut(&wstack).unwrap().push(val.clone());
                    stacks.get_mut(&wstack).unwrap().push(val.clone());
                    (true, wstack)
                }
            } // dub
            ".." => {
                let mut run = true;
                let str1 = match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => v,
                    None => { 
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                        "".to_string()
                    }
                };
                let str2 = match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => v,
                    None => { 
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                        "".to_string()
                    }
                };
                if run {
                    stacks.get_mut(&wstack).unwrap().push(format!("{}{}", str2, str1));
                    (true, wstack)
                } else {
                    (false, wstack)
                }
            } // concatination
            "+" => {
                let mut num1 = 0.0;
                let mut num2 = 0.0;
                let mut run = true;
                match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => match v.parse::<f64>() {
                        Ok(v) => num1 = v,
                        Err(_) => {
                            println!("\n---");
                            println!("Error: attempted to add a non-number");
                            run = false;
                        }
                    },
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                    }
                }
                match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => match v.parse::<f64>() {
                        Ok(v) => num2 = v,
                        Err(_) => {
                            println!("\n---");
                            println!("Error: attempted to add a non-number");
                            run = false;
                        }
                    },
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                    }
                }
                if run == false {
                    (false, wstack)
                } else {
                    stacks
                        .get_mut(&wstack)
                        .unwrap()
                        .push(format!("{}", num2 + num1));
                    (true, wstack)
                }
            }
            "-" => {
                let mut num1 = 0.0;
                let mut num2 = 0.0;
                let mut run = true;
                match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => match v.parse::<f64>() {
                        Ok(v) => num1 = v,
                        Err(_) => {
                            println!("\n---");
                            println!("Error: attempted to min a non-number");
                            run = false;
                        }
                    },
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                    }
                }
                match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => match v.parse::<f64>() {
                        Ok(v) => num2 = v,
                        Err(_) => {
                            println!("\n---");
                            println!("Error: attempted to add a non-number");
                            run = false;
                        }
                    },
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                    }
                }
                if run == false {
                    (false, wstack)
                } else {
                    stacks
                        .get_mut(&wstack)
                        .unwrap()
                        .push(format!("{}", num2 - num1));
                    (true, wstack)
                }
            }
            "*" => {
                let mut num1 = 0.0;
                let mut num2 = 0.0;
                let mut run = true;
                match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => match v.parse::<f64>() {
                        Ok(v) => num1 = v,
                        Err(_) => {
                            println!("\n---");
                            println!("Error: attempted to mult a non-number");
                            run = false;
                        }
                    },
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                    }
                }
                match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => match v.parse::<f64>() {
                        Ok(v) => num2 = v,
                        Err(_) => {
                            println!("\n---");
                            println!("Error: attempted to add a non-number");
                            run = false;
                        }
                    },
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                    }
                }
                if run == false {
                    (false, wstack)
                } else {
                    stacks
                        .get_mut(&wstack)
                        .unwrap()
                        .push(format!("{}", num2 * num1));
                    (true, wstack)
                }
            }
            "/" => {
                let mut num1 = 0.0;
                let mut num2 = 0.0;
                let mut run = true;
                match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => match v.parse::<f64>() {
                        Ok(v) => num1 = v,
                        Err(_) => {
                            println!("\n---");
                            println!("Error: attempted to div a non-number");
                            run = false;
                        }
                    },
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                    }
                }
                match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => match v.parse::<f64>() {
                        Ok(v) => num2 = v,
                        Err(_) => {
                            println!("\n---");
                            println!("Error: attempted to add a non-number");
                            run = false;
                        }
                    },
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                    }
                }
                if run == false {
                    (false, wstack)
                } else {
                    stacks
                        .get_mut(&wstack)
                        .unwrap()
                        .push(format!("{}", num2 / num1));
                    (true, wstack)
                }
            }
            "#" => {
                *mode = Mode::Comment;
                (true, wstack)
            } // comment
            ":" => {
                *mode = Mode::Define;
                (true, wstack)
            } // define
            "?" => match stacks.get_mut(&wstack).unwrap().pop() {
                Some(v) => match v.as_str() {
                    "true" => {
                        *mode = Mode::If(true);
                        (true, wstack)
                    }
                    "false" => {
                        *mode = Mode::If(false);
                        (true, wstack)
                    }
                    _ => {
                        println!("\n---");
                        println!("Error: attempted to use ? on a non-boolean");
                        (false, wstack)
                    }
                },
                None => {
                    println!("\n---");
                    println!("Error: stack underflow");
                    (false, wstack)
                }
            }, // if
            ">" => {
                let mut num1 = 0.0;
                let mut num2 = 0.0;
                let mut run = true;
                match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => match v.parse::<f64>() {
                        Ok(v) => num1 = v,
                        Err(_) => {
                            println!("\n---");
                            println!("Error: tried to use > on a non-number");
                            run = false;
                        }
                    },
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                    }
                }
                match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => match v.parse::<f64>() {
                        Ok(v) => num2 = v,
                        Err(_) => {
                            println!("\n---");
                            println!("Error: tried to use > on a non-number");
                            run = false;
                        }
                    },
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                    }
                }
                if run == false {
                    (false, wstack)
                } else {
                    if num2 > num1 {
                        stacks.get_mut(&wstack).unwrap().push("true".to_string());
                    } else {
                        stacks.get_mut(&wstack).unwrap().push("false".to_string());
                    }
                    (true, wstack)
                }
            }
            "<" => {
                let mut num1 = 0.0;
                let mut num2 = 0.0;
                let mut run = true;
                match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => match v.parse::<f64>() {
                        Ok(v) => num1 = v,
                        Err(_) => {
                            println!("\n---");
                            println!("Error: tried to use < on a non-number");
                            run = false;
                        }
                    },
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                    }
                }
                match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => match v.parse::<f64>() {
                        Ok(v) => num2 = v,
                        Err(_) => {
                            println!("\n---");
                            println!("Error: tried to use > on a non-number");
                            run = false;
                        }
                    },
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                    }
                }
                if run == false {
                    (false, wstack)
                } else {
                    if num2 < num1 {
                        stacks.get_mut(&wstack).unwrap().push("true".to_string());
                    } else {
                        stacks.get_mut(&wstack).unwrap().push("false".to_string());
                    }
                    (true, wstack)
                }
            }
            "=" => {
                let mut num1 = 0.0;
                let mut str1 = String::new();
                let mut num2 = 0.0;
                let mut str2 = String::new();
                let mut run = true;
                let mut nums = true;
                match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => match v.parse::<f64>() {
                        Ok(v) => num1 = v,
                        Err(_) => {
                            str1 = v;
                            nums = false;
                        }
                    },
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                    }
                }
                match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => match v.parse::<f64>() {
                        Ok(v) => num2 = v,
                        Err(_) => {
                            str2 = v;
                            nums = false;
                        }
                    },
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                    }
                }
                if run == false {
                    (false, wstack)
                } else {
                    if nums {
                        if num2 == num1 {
                            stacks.get_mut(&wstack).unwrap().push("true".to_string());
                        } else {
                            stacks.get_mut(&wstack).unwrap().push("false".to_string());
                        }
                    } else {
                        if str2 == str1 {
                            stacks.get_mut(&wstack).unwrap().push("true".to_string());
                        } else {
                            stacks.get_mut(&wstack).unwrap().push("false".to_string());
                        }
                    }
                    (true, wstack)
                }
            }
            "!=" => {
                let mut num1 = 0.0;
                let mut str1 = String::new();
                let mut num2 = 0.0;
                let mut str2 = String::new();
                let mut run = true;
                let mut nums = true;
                match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => match v.parse::<f64>() {
                        Ok(v) => num1 = v,
                        Err(_) => {
                            str1 = v;
                            nums = false;
                        }
                    },
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                    }
                }
                match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => match v.parse::<f64>() {
                        Ok(v) => num2 = v,
                        Err(_) => {
                            str2 = v;
                            nums = false;
                        }
                    },
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        run = false;
                    }
                }
                if run == false {
                    (false, wstack)
                } else {
                    if nums {
                        if num2 != num1 {
                            stacks.get_mut(&wstack).unwrap().push("true".to_string());
                        } else {
                            stacks.get_mut(&wstack).unwrap().push("false".to_string());
                        }
                    } else {
                        if str2 != str1 {
                            stacks.get_mut(&wstack).unwrap().push("true".to_string());
                        } else {
                            stacks.get_mut(&wstack).unwrap().push("false".to_string());
                        }
                    }
                    (true, wstack)
                }
            }
            "&" => match stacks.get_mut(&wstack).unwrap().pop() {
                Some(v) => {
                    if stacks.contains_key(&"back_rep".to_string()) {
                        match v.parse::<f64>() {
                            Ok(v) => {
                                stacks
                                    .get_mut(&"back_rep".to_string())
                                    .unwrap()
                                    .push(format!("{}", v));
                                *mode = Mode::Rep;
                                (true, wstack)
                            }
                            Err(_) => {
                                println!("\n---");
                                println!("Error: rep must take a number");
                                (false, wstack)
                            }
                        }
                    } else {
                        println!("\n---");
                        println!("Error: rep stack is nonexistant");
                        (false, wstack)
                    }
                }
                None => {
                    println!("\n---");
                    println!("Error: stack underflow");
                    (false, wstack)
                }
            }, // for
            "$$" => match stacks.get_mut(&wstack).unwrap().pop() {
                Some(v) => {
                    if !stacks.contains_key(&v) {
                        stacks.insert(v, Vec::new());
                    }
                    (true, wstack)
                }
                None => {
                    println!("\n---");
                    println!("Error: stack underflow");
                    (false, wstack)
                }
            }, // new stack
            "$" => match stacks.get_mut(&wstack).unwrap().pop() {
                Some(v) => {
                    if stacks.contains_key(&v) {
                        (true, v)
                    } else {
                        println!("\n---");
                        println!("Error: stack does not exist");
                        (false, wstack)
                    }
                }
                None => {
                    println!("\n---");
                    println!("Error: stack underflow");
                    (false, wstack)
                }
            }, // swap stack
            "!" => match stacks.get_mut(&wstack).unwrap().pop() {
                Some(_) => (true, wstack),
                None => {
                    println!("\n---");
                    println!("Error: stack underflow");
                    (false, wstack)
                }
            },
            "$>" => match stacks.get_mut(&wstack).unwrap().pop() {
                Some(s) => match stacks.get_mut(&wstack).unwrap().pop() {
                    Some(v) => {
                        if stacks.contains_key(&s) {
                            stacks.get_mut(&s).unwrap().push(v);
                            (true, wstack)
                        } else {
                            println!("\n---");
                            println!("Error: stack does not exist");
                            (false, wstack)
                        }
                    }
                    None => {
                        println!("\n---");
                        println!("Error: stack underflow");
                        (false, wstack)
                    }
                },
                None => {
                    println!("\n---");
                    println!("Error: stack underflow");
                    (false, wstack)
                }
            },
            "[" => {
                *mode = Mode::Push;
                (true, wstack)
            }
            // "@" => { } // load external words
            _ => {
                if dictionary.contains_key(&word.to_string()) {
                    let mut run: bool = true;
                    let repn: &mut Vec<String> = &mut Vec::new();
                    let defn: &mut Word = &mut Word::blank();
                    let ifdn: &mut Vec<String> = &mut Vec::new();
                    let mut wstackn = wstack.clone();
                    let moden = &mut Mode::Normal;
                    for word in dictionary.get(&word.to_string()).cloned().unwrap() {
                        (run, wstackn) = eval(
                            word.as_str(),
                            stacks,
                            wstackn,
                            ifdn,
                            repn,
                            dictionary,
                            moden,
                            defn,
                            verbose,
                        );
                        if run == false {
                            break;
                        }
                    }
                    if run == false {
                        (false, wstack)
                    } else {
                        (true, wstack)
                    }
                } else {
                    println!("\n---");
                    println!("Error: undefined word: {}", word);
                    (false, wstack)
                }
            }
        },
        Mode::Push => match word {
            "]" => {
                *mode = Mode::Normal;
                (true, wstack)
            }
            _ => {
                stacks.get_mut(&wstack).unwrap().push(word.to_string());
                (true, wstack)
            }
        },
        Mode::Comment => match word {
            "#" => {
                *mode = Mode::Normal;
                (true, wstack)
            }
            _ => (true, wstack),
        },
        Mode::Define => match word {
            ";" => match stacks.get_mut(&"back_block".to_string()).unwrap().pop() {
                Some(_) => {
                    def.data.push(word.to_string());
                    (true, wstack)
                }
                None => {
                    if def.name != "" && def.data.len() > 0 {
                        dictionary.insert(def.name.clone(), def.data.clone());
                        *mode = Mode::Normal;
                        *def = Word::blank();
                        (true, wstack)
                    } else {
                        println!("\n---");
                        println!("Error: invalid define");
                        (false, wstack)
                    }
                }
            },
            "&" | "?" | ":" => {
                stacks
                    .get_mut(&"back_block".to_string())
                    .unwrap()
                    .push("blk".to_string());
                def.data.push(word.to_string());
                (true, wstack)
            }
            _ => {
                if def.name == "" {
                    def.name = word.to_string();
                    (true, wstack)
                } else {
                    def.data.push(word.to_string());
                    (true, wstack)
                }
            }
        },
        Mode::Rep => match word {
            ";" => match stacks.get_mut(&"back_block".to_string()).unwrap().pop() {
                Some(_) => {
                    rep.push(word.to_string());
                    (true, wstack)
                }
                None => {
                    let mut run: bool;
                    let moden = &mut Mode::Normal;
                    let repn: &mut Vec<String> = &mut Vec::new();
                    let defn: &mut Word = &mut Word::blank();
                    let ifdn: &mut Vec<String> = &mut Vec::new();
                    let mut wstackn = wstack.clone();
                    let cnt = match stacks.get_mut(&"back_rep".to_string()).unwrap().pop() {
                        Some(v) => match v.parse::<f64>() {
                            Ok(v) => {
                                run = true;
                                v
                            }
                            Err(_) => {
                                println!("\n---");
                                println!("Error: rep must take a number");
                                run = false;
                                0.0
                            }
                        },
                        None => {
                            println!("\n---");
                            println!("Error: stack underflow 1");
                            run = false;
                            0.0
                        }
                    };
                    if run {
                        let mut sb = 0.0;
                        stacks
                            .get_mut(&"back_rep".to_string())
                            .unwrap()
                            .push("0".to_string());
                        loop {
                            if cnt - sb <= 0.0 {
                                break;
                            }
                            for word in &mut *rep {
                                (run, wstackn) = eval(
                                    word.as_str(),
                                    stacks,
                                    wstackn,
                                    ifdn,
                                    repn,
                                    dictionary,
                                    moden,
                                    defn,
                                    verbose,
                                );
                                if !run {
                                    break;
                                }
                            }
                            sb = match stacks.get_mut(&"back_rep".to_string()).unwrap().pop() {
                                Some(v) => match v.parse::<f64>() {
                                    Ok(v) => {
                                        run = true;
                                        v
                                    }
                                    Err(_) => {
                                        println!("\n---");
                                        println!("Error: rep must take a number");
                                        run = false;
                                        cnt
                                    }
                                },
                                None => {
                                    println!("\n---");
                                    println!("Error: stack underflow 2");
                                    run = false;
                                    cnt
                                }
                            } + 1.0;
                            stacks
                                .get_mut(&"back_rep".to_string())
                                .unwrap()
                                .push(format!("{}", sb));

                        }
                        if !run {
                            (false, wstack)
                        } else {
                            *mode = Mode::Normal;
                            *rep = Vec::new();
                            stacks
                                .get_mut(&"back_rep".to_string())
                                .unwrap()
                                .pop();
                            (true, wstack)
                        }
                    } else {
                        (false, wstack)
                    }
                }
            },
            "&" | "?" | ":" => {
                stacks
                    .get_mut(&"back_block".to_string())
                    .unwrap()
                    .push("blk".to_string());
                rep.push(word.to_string());
                (true, wstack)
            }
            _ => {
                rep.push(word.to_string());
                (true, wstack)
            }
        },
        Mode::If(b) => match b {
            true => match word {
                ";" => match stacks.get_mut(&"back_block".to_string()).unwrap().pop() {
                    Some(_) => {
                        ifd.push(word.to_string());
                        (true, wstack)
                    }
                    None => {
                        let mut run: bool = true;
                        let repn: &mut Vec<String> = &mut Vec::new();
                        let defn: &mut Word = &mut Word::blank();
                        let ifdn: &mut Vec<String> = &mut Vec::new();
                        let mut wstackn = wstack.clone();
                        let moden = &mut Mode::Normal;
                        for word in &mut *ifd {
                            (run, wstackn) = eval(
                                word.as_str(),
                                stacks,
                                wstackn,
                                ifdn,
                                repn,
                                dictionary,
                                moden,
                                defn,
                                verbose,
                            );
                            if !run {
                                break;
                            }
                        }
                        if !run {
                            (false, wstack)
                        } else {
                            *ifd = Vec::new();
                            *mode = Mode::Normal;
                            (true, wstack)
                        }
                    }
                },
                "&" | "?" | ":" => {
                    stacks
                        .get_mut(&"back_block".to_string())
                        .unwrap()
                        .push("blk".to_string());
                    ifd.push(word.to_string());
                    (true, wstack)
                }
                _ => {
                    ifd.push(word.to_string());
                    (true, wstack)
                }
            },
            false => match word {
                ";" => match stacks.get_mut(&"back_block".to_string()).unwrap().pop() {
                    Some(_) => (true, wstack),
                    None => {
                        *mode = Mode::Normal;
                        (true, wstack)
                    }
                },
                "&" | "?" | ":" => {
                    stacks
                        .get_mut(&"back_block".to_string())
                        .unwrap()
                        .push("blk".to_string());
                    (true, wstack)
                }
                _ => (true, wstack),
            },
        },
    }
}
