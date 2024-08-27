#[derive(Debug, Clone, Copy)]
pub enum Token {
    RightAngle,
    LeftAngle,
    Minus,
    Plus,
    Dot,
    // Input,
    BracketStart,
    BracketEnd,
}

pub fn tokenizer(src: &str) -> Vec<Token> {
    let mut tokens = vec![];
    for char in src.chars() {
        let token = match char {
            '<' => Token::LeftAngle,
            '>' => Token::RightAngle,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '.' => Token::Dot,
            // ',' => Token::Input,
            '[' => Token::BracketStart,
            ']' => Token::BracketEnd,
            _ => panic!("Unknown token."),
        };

        tokens.push(token);
    }

    tokens
}
