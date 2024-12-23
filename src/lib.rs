mod lexer;
mod parser;

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;

    #[test]
    fn hello() {
        let mut lexer = Lexer::from_string(
        r#"
        (
            symbol "AAPL", # Apple Inc
            close_price 100.27,
            past_prices [99.80, 100.17, 110.17],
            delisted False,
            address None,
            financials (
                eps 0.27,
                revenue 100_000,
            )
        )
        "#
        );

        while !lexer.is_done() {
            match lexer.next() {
                Ok(token) => println!("Token: {:?}", token),
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                },
            }
        }
    }
}
