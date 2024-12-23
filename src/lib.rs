mod lexer;
mod parser;
mod rule;

#[cfg(test)]
mod test {
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    fn hello() {
        let chars = r#"
        (
            symbol "AAPL💀💀💀;", # Apple Inc
            close_price 100.27,
            past_prices [99.80, 100.17, -110.17],
            delisted False,
            something Badly,
            idk idk idk,
            address None,
            financials (
                eps 0.27,
                revenue 100_000,
            )
        )
        "#;

        let parser = Parser::new(chars);

        println!("{:?}", parser.parse().unwrap());
    }
}
