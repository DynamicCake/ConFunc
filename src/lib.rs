mod error;


mod expression;
mod lexer;
mod parser;

#[cfg(test)]
mod tests {
    use crate::{lexer::{Lexer, TokenType}};

    #[test] 
    fn printout() {
        println!("Running printout");
        let lexer = Lexer::new("join(\"joined: \", add(1, 3), sub(4, 2))");
        

        let mut list = Vec::new();
        for it in lexer {
            let it = it.unwrap();
            println!("{:?}", it);
            list.push(it.token_type);
        }

        println!("{:?}", list);

    }

    #[test]
    fn test_identifier() {
        let str = "hello";
        let mut lexer = Lexer::new(str);

        assert_eq!(lexer.next().expect("Next expected").unwrap().token_type, TokenType::Identifier("hello".to_string()))
    }
}
