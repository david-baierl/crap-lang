#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    Default,

    // Comma,
    // Assignment,

    // Ternary,

    // Logical_OR,
    // Logical_AND,

    // Bitwise_OR,
    // Bitwise_XOR,
    // Bitwise_AND,

    // Logical_equal,
    // Logical_size,

    // Bitwise_shift,

    Additive,
    Multiplicative,

    Unary,
    Primary,
}
