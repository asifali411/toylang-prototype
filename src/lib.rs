use std::{cell::RefCell, collections::HashMap, process::ExitCode, rc::Rc};

mod errors;
mod interpreter;
mod lexer;
mod parser;
mod native;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use errors::lang_error::LangError;

use crate::{interpreter::{environment::Environment, resolver::Resolver, value::{NativeFn, Value}}, native::{io::{input, output}, types::{to_num, to_string, type_of}}};
type Env = Rc<RefCell<Environment>>;

pub fn run(source: String) -> ExitCode {
    match try_run(source) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            e.display();
            ExitCode::FAILURE
        }
    }
}

fn try_run(source: String) -> Result<(), LangError> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let statements = Parser::new(&tokens).parse()?;

    let mut resolver = Resolver::new();
    for statement in &statements {
        resolver.resolve_stmt(statement);
    }

    let mut interpreter = Interpreter::new();
    interpreter.locals = resolver.locals;

    define_natives(&interpreter.globals);

    for statement in &statements {
        interpreter.execute(statement)?;
    }

    Ok(())
}

fn define_natives(env: &Env) {

    let mut native_functions: HashMap<String, NativeFn> = HashMap::new();

    native_functions.insert(String::from("output"), output);
    native_functions.insert(String::from("input"), input);
    native_functions.insert(String::from("number"), to_num);
    native_functions.insert(String::from("string"), to_string);
    native_functions.insert(String::from("type"), type_of);

    for (name, func) in native_functions {
        env.borrow_mut().define_var(name.clone(), Value::NativeFunction {
            name,
            func,
        });
    }
}