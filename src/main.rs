use std::collections::HashMap;
use std::{env, fs, io, io::Write};

pub mod bools;
pub mod chars;
pub mod errors;
pub mod file;
pub mod floats;
pub mod helper;
pub mod list;
pub mod logic;
pub mod variable;
use errors::*;
use file::Code;
use variable::{
	CallStack, Functions, Labels, Variable, Variable::*, VariableT, VariableT::*, Variables,
};

const KEYWORDS: [&str; 15] = [
	"let", "if", "endif", "print", "clear", "label", "jump", "jump_rel", "type", "end", "fn",
	"last", "len", "exit", "return",
];

fn main() {
	let mut code = Code::new();
	for file in env::args().skip(1) {
		code.import(&file)
			.unwrap_or_else(|_| panic!("Couldn't read file: {}", &file));
	}
	match logic::run(code) {
		Ok(_) => {}
		Err(e) => eprintln!("{}", e),
	}
}
