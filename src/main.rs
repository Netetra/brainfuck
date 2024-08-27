use std::io::Write;

use env_logger::{Builder, Env};
use log::{debug, info, LevelFilter};
use parser::brainfuck;
use runner::Runner;
use tokenizer::tokenizer;

mod parser;
mod runner;
mod tokenizer;

fn main() {
    let env = Env::default().filter_or("RUST_LOG", "INFO");
    let mut builder = Builder::from_env(env);
    builder
        .format(|buf, record| writeln!(buf, "[{}] {}", record.level(), record.args()))
        .init();

    let mut input = String::new();
    print!(">> ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).ok();
    let input = input.trim_end();
    info!("input: {}", input);

    let tokens = tokenizer(input);
    let ast = brainfuck::program(&tokens).unwrap();
    debug!("AST: {:?}", &ast);

    let mut runner = Runner::new(10);

    info!("output: {}", runner.run(&ast));
}
