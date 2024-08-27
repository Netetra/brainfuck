use crate::tokenizer::Token;

#[derive(Debug)]
pub enum Node {
    DecrementPointer,
    IncrementPointer,
    Decrement,
    Increment,
    Output,
    // Input,
    Block(Vec<Node>),
}

peg::parser! {
    pub grammar brainfuck() for [Token] {
        rule decrement_pointer() -> Node
            = [Token::LeftAngle] { Node::DecrementPointer }
        rule increment_pointer() -> Node
            = [Token::RightAngle] { Node::IncrementPointer }
        rule decrement() -> Node
            = [Token::Minus] { Node::Decrement }
        rule increment() -> Node
            = [Token::Plus] { Node::Increment }
        rule output() -> Node
            = [Token::Dot] { Node::Output }
        rule command() -> Node
            = decrement_pointer() / increment_pointer() / decrement() / increment() / output() / block()
        rule block() -> Node
            = [Token::BracketStart] commands:(command()*) [Token::BracketEnd] { Node::Block(commands) }
        pub rule program() -> Vec<Node>
            = commands:(command()*) { commands }
    }
}
