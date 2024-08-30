use std::{mem::MaybeUninit, sync::Once};
use regex::Regex;
use crate::utils::Singleton;

pub struct Pattern {
    // --- literals & constants --- //

    pub string: Regex,
    pub number: Regex,

    // --- punctuators --- //

    pub open_paren: Regex,
    pub close_paren: Regex,

    // --- operators --- //

    pub plus: Regex,
    pub minus: Regex,
    pub star: Regex,
    pub slash: Regex,
    pub percent: Regex,
    
    // --- identifier --- //

    pub identifier: Regex,

    // --- keywords --- //

    // --- special characters --- //

    pub whitespace: Regex,
    pub end_of_line: Regex,
}

impl Pattern {
    fn new() -> Pattern {
        Pattern {
            // --- literals & constants --- //

            string: Regex::new(r#"^"[^"]*""#).unwrap(),
            number: Regex::new(r"^[0-9][0-9_]*(\.[0-9]*)?").unwrap(),

            // --- punctuators --- //

            open_paren: Regex::new(r"^\(").unwrap(),
            close_paren: Regex::new(r"^\)").unwrap(),

            // --- operators --- //

            plus: Regex::new(r"^\+").unwrap(),
            minus: Regex::new(r"^-").unwrap(),
            star: Regex::new(r"^\*").unwrap(),
            slash: Regex::new(r"^/").unwrap(),
            percent: Regex::new(r"^%").unwrap(),
            
            // --- identifier --- //

            identifier: Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(),


            // --- keywords --- //

            // --- special characters --- //

            whitespace: Regex::new(r"^[\s\t]+").unwrap(),
            end_of_line: Regex::new(r"^[\n\r]+").unwrap(),
        }
    }
}

// -----------------------------------------
// singleton
// -----------------------------------------

static mut PATTERN: MaybeUninit<Pattern> = MaybeUninit::uninit();
static ONCE: Once = Once::new();

impl Singleton<Pattern> for Pattern {
    fn instance() -> &'static Pattern {
        unsafe { 
            ONCE.call_once(|| { PATTERN.write(Pattern::new()); });
            PATTERN.assume_init_ref()
        }
    }
}