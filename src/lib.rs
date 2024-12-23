mod lexer;
mod parser;

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;

    #[test]
    fn hello() {
        let mut lexer = Lexer::from_string("(())");
        while let Ok(token) = lexer.next() {
            println!("Token: {:?}", token);
        }
    }
}
