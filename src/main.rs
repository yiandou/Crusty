mod ast;

use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CRUSTYParser;

fn main() {

}