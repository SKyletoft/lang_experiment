#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt;
use std::io;

pub mod bools;
pub mod floats;
pub mod variable;
use variable::Variable;

type CustomErr = Box<dyn std::error::Error>;

#[derive(Copy, Clone, Debug)]
struct ParseError {}
impl std::error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse error!")
    }
}
fn perr() -> CustomErr {
	Box::new(ParseError {})
}

#[derive(Copy, Clone, Debug)]
struct TypeError {}
impl std::error::Error for TypeError {}
impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type error!")
    }
}

fn perr() -> CustomErr {
    Box::new(ParseError {})
}
fn terr() -> CustomErr {
    Box::new(TypeError {})
}

fn create_variable(
    variables: &mut HashMap<String, Variable>,
    words: &[&str],
) -> Result<(), CustomErr> {
    let name = words[0].to_string();
    if words[2] != "=" {
        return Err(perr());
    }
    let new = match words[1] {
        "num" => floats::evaluate_floats(&words[3..], &variables)?,
        "bool" => bools::evaluate_bools(&words[3..], &variables)?,
        "list" => unimplemented!(),
        _ => unimplemented!(),
    };
    variables.insert(name, new);
    Ok(())
}
fn if_statement() -> Result<(), CustomErr> {
    Ok(())
}
fn print(variables: &mut HashMap<String, Variable>, words: &[&str]) -> Result<(), CustomErr> {
    print!("> ");
    for &word in words {
        let result = variables.get(word).ok_or_else(perr)?;
        print!("{} ", result);
    }
    println!();
    Ok(())
}
fn clear() -> Result<(), CustomErr> {
    Ok(())
}

fn create_labels(
    labels: &mut HashMap<String, usize>,
    words: &[&str],
    index: usize,
) -> Result<(), CustomErr> {
    if words.is_empty() {
        return Err(perr());
    }
    labels.insert(words[0].to_string(), index);
    Ok(())
}

struct Code {
    code: Vec<String>,
    index: usize,
}

impl Code {
    fn new() -> Self {
        Code {
            code: Vec::new(),
            index: 0,
        }
    }
    fn next<'a>(&'a mut self) -> Result<&'a str, CustomErr> {
        self.index += 1;
        while self.index >= self.code.len() {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line)?;
            self.code.push(input_line);
        }
        Ok(&self.code[self.index])
    }
}

fn jump(
    labels: &HashMap<String, usize>,
    call_stack: &mut Vec<usize>,
	jump_next: &mut Option<usize>,
	words: &[&str],
	index: usize
) -> Result<(), CustomErr> {
	if words.is_empty() {
		return Err(perr());
	}
	let &target = labels.get(words[0]).ok_or_else(perr)?;
	call_stack.push(index);
	*jump_next = Some(target);
    Ok(())
}

fn main() -> Result<(), CustomErr> {
    let mut variables: HashMap<String, Variable> = HashMap::new();
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut call_stack: Vec<usize> = Vec::new();
    let mut jump_next: Option<usize> = None;
    let mut code = Code::new();
    loop {
        let index = code.index + 1;
        let input_line = code.next()?;
        let words = input_line.trim().split_whitespace().collect::<Vec<&str>>();
        if words.is_empty() {
            continue;
        }
        //dbg!(&words);
        let result = match words[0] {
            "let" => create_variable(&mut variables, &words[1..]),
            "if" => if_statement(),
            "print" => print(&mut variables, &words[1..]),
            "clear" => clear(),
            "label" => create_labels(&mut labels, &words[1..], index),
            "jump" => jump(&labels, &mut call_stack, &mut jump_next, &words[1..], index),
            "end" => unimplemented!(),
            _ => Err(perr()),
        };
        if result.is_err() {
            println!("Parse error");
        }
        if let Some(target) = jump_next {
			code.index = target;
			jump_next = None;
        }
    }
    //Ok(())
}
