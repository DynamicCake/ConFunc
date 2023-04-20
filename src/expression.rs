use crate::lexer::Literal;


enum Expression {

    Call {
        identifier: String,
        params: Vec<Self>
    },
    Literal(Literal),

}

