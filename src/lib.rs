#![feature(globs)]
#![feature(phase)]

extern crate regex;
#[phase(plugin)] extern crate regex_macros;

pub mod lexer;
pub mod parser;
