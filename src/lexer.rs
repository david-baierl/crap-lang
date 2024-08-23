use regex::{Match, Regex};
use tokens::{Token, TokenKind};

mod tokens;

type PatternHandler = Box<dyn Fn(&mut Lexer, Match) -> () + 'static>;
pub struct RegexPattern {
    regex: Regex,
    handler: PatternHandler,
}

pub struct Lexer {
    tokens: Vec<tokens::Token>,
    index: usize,
}

impl Lexer {
    pub fn advance(&mut self, length: usize) {
        self.index += length
    }

    pub fn from() -> Lexer {
        Lexer {
            tokens: vec![],
            index: 0,
        }
    }
}

fn default_handler(kind: tokens::TokenKind) -> PatternHandler {
    Box::new(move |lexer: &mut Lexer, find| {
        let token = Token::from(
            kind,
            lexer.index.try_into().unwrap(),
        );

        lexer.tokens.push(token);
        lexer.advance(find.len());
    })
}

fn skip_handler(lexer: &mut Lexer, find: Match) {
    lexer.advance(find.len());
}

pub fn tokenize<'a>(buffer: &'a str) -> Vec<tokens::Token> {
    let mut lexer = Lexer::from();

    let patterns: Vec<RegexPattern> = vec![

        // --- whitespace --- //

        {RegexPattern {
            regex: Regex::new(r"^[\s\t\n\r]+").unwrap(),
            handler: Box::from(skip_handler),
        }},

        // --- identifier & keywords --- //

        {RegexPattern {
            regex: Regex::new(r#"^"[^"]*""#).unwrap(),
            handler: default_handler(TokenKind::String),
        }},

        // --- literals --- //

        {RegexPattern {
            regex: Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(),
            // @TODO: special handler for keywords
            handler: default_handler(TokenKind::Identifier),
        }},
        {RegexPattern {
            regex: Regex::new(r"^[0-9][0-9_]*(\.[0-9]*)?").unwrap(),
            handler: default_handler(TokenKind::Number),
        }},

        // --- punctuators --- //

        {RegexPattern {
            regex: Regex::new(r"^\(").unwrap(),
            handler: default_handler(TokenKind::OpenParen),
        }},
        {RegexPattern {
            regex: Regex::new(r"^\)").unwrap(),
            handler: default_handler(TokenKind::CloseParen),
        }},

        // --- operators --- //

        {RegexPattern {
            regex: Regex::new(r"^\+").unwrap(),
            handler: default_handler(TokenKind::Plus),
        }},
        {RegexPattern {
            regex: Regex::new(r"^-").unwrap(),
            handler: default_handler(TokenKind::Minus),
        }},
        {RegexPattern {
            regex: Regex::new(r"^\*").unwrap(),
            handler: default_handler(TokenKind::Star),
        }},
        {RegexPattern {
            regex: Regex::new(r"^/").unwrap(),
            handler: default_handler(TokenKind::Slash),
        }},
        {RegexPattern {
            regex: Regex::new(r"^%").unwrap(),
            handler: default_handler(TokenKind::Percent),
        }},
    ];

    'outer: while lexer.index < buffer.len() {
        let slice = &buffer[lexer.index..];

        for pattern in patterns.iter() {
            let find = match pattern.regex.find(&slice) {
                Some(_find) => { _find }
                None => { continue; }
            };

            if find.start() != 0 { continue; }
            pattern.handler.as_ref()(&mut lexer, find);

            continue 'outer;
        }

        // @TODO: error nothing matched
        println!("end");
        break;
    }

    lexer.tokens.push(Token::from(
        TokenKind::Eof,
        lexer.index.try_into().unwrap()
    ));

    for token in lexer.tokens.iter() {
        tokens::debug(token)
    }

    lexer.tokens
}
