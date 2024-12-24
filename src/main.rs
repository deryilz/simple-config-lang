// remove
#![allow(dead_code)]

mod lexer;
mod parser;
mod rule_parser;

use std::time::Instant;

use crate::{lexer::Lexer, parser::Parser};

fn main() {
    let chars = r#"
            (
                symbol "AAPL💀💀💀💀💀", # Apple Inc
                close_price 100.27,
                past_prices [99.80, 100.17, 110.17],
                delisted False,
                address None,
                financials (
                    eps 0.27,
                    revenue 100_000,
                )
            )
        "#;

    let start = Instant::now();
    let mut parser = Parser::new(chars);
    let value = parser.parse().unwrap();
    let end = Instant::now();

    println!("{:?}", end.duration_since(start));
    println!("{:?}", value);
}
