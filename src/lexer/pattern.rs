use regex::Regex;

pub enum PatternType {
    Regex(Regex),
    String(&'static str),
}

impl PatternType {
    // @TODO: add value
    pub fn find<'a>(&self, slice: &'a str) -> Option<&'a str> {
        match self {
            PatternType::Regex(pattern) => {
                pattern.find(slice).map(|find| find.as_str())
            },
            PatternType::String(value) => {
                if slice.starts_with(value) {
                    return Some(value);
                }
                None
            }
        }
    }
}

pub struct Pattern {
    /* --- literals & constants --- */
    pub string: PatternType,
    pub number: PatternType,

    /* --- punctuators --- */
    pub open_paren: PatternType,
    pub close_paren: PatternType,
    pub semi: PatternType,
    pub question: PatternType,
    pub colon: PatternType,
    pub equal: PatternType,

    /* --- operators --- */
    pub plus: PatternType,
    pub minus: PatternType,
    pub star: PatternType,
    pub slash: PatternType,
    pub percent: PatternType,

    /* --- identifier --- */
    pub identifier: PatternType,

    /* --- keywords --- */

    /* --- special characters --- */
    pub single_line_comment: PatternType,
    pub multi_line_comment: PatternType,
    pub end_of_line: PatternType,
    pub whitespace: PatternType,
}

impl Pattern {
    pub fn new() -> Pattern {
        Pattern {
            // --- literals & constants --- //
            string: PatternType::Regex(Regex::new(r#"^"[^"]*""#).unwrap()),
            number: PatternType::Regex(Regex::new(r"^[0-9][0-9_]*(\.[0-9]*)?").unwrap()),

            /* --- punctuators --- */
            open_paren: PatternType::String("("),
            close_paren: PatternType::String(")"),
            semi: PatternType::String(";"),
            question: PatternType::String("?"),
            colon: PatternType::String(":"),
            equal: PatternType::String("="),

            /* --- operators --- */
            plus: PatternType::String("+"),
            minus: PatternType::String("-"),
            star: PatternType::String("*"),
            slash: PatternType::String("/"),
            percent: PatternType::String("%"),

            /* --- identifier --- */
            identifier: PatternType::Regex(Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap()),

            /* --- keywords --- */

            /* --- special characters --- */
            single_line_comment: PatternType::Regex(Regex::new(r"^//.*([\r\n]|$)").unwrap()),
            multi_line_comment: PatternType::Regex(Regex::new(r"^/\*(.|[\r\n])*?\*/").unwrap()),
            end_of_line: PatternType::Regex(Regex::new(r"^[\r\n]+").unwrap()),
            whitespace: PatternType::Regex(Regex::new(r"^[\s\t]+").unwrap()),
        }
    }
}
