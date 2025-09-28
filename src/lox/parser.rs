pub enum Expression {
    Binary {
        left: Expression,
        operator: Token,
        right: Expression
    },
    Grouping {
        expression: Expression
    },
    Literal
}

