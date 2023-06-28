//#########################
// D E P E N D E N C I E S
//#########################

    use std::{fmt, ops::Range};
    use std::fmt::Display;
    use rusty_toolkit::{
        Token, ReadToken,
        LexingErr,
        FixedStr, L16, L32
    }; // use ..


//#######################
// D E F I N I T I O N S
//#######################

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum WsToken {
        Let, Set, If, While, Choose, Out, Tell, Def,

        Ident(FixedStr<L16>), DigitLit(i16), StrLit(FixedStr<L32>),

        Int, Bool, Str, Deck, Void,

        Plus, Minus, Star, Slash, Dice,
        Eq, Ne, Gt, Ge, Lt, Le,
        InArrow, OutArrow,
        LeftParen,   RightParen,
        LeftBracket, RightBracket,
        Colon, SemiColon, Qmark,
        Pipe,
        Tide,
        Ellipsis,

        Eos, Eof,
    } // enum WsToken


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl Token for WsToken {
        const EOF:              Self         = WsToken::Eof;
        const COMMENT_KEY:      Option<char> = Some('#');
        const LINE_COMMENT_KEY: Option<char> = Some('~');
        const STR_LIT_KEY:      Option<char> = Some('\'');
    } // impl ..

    impl ReadToken for WsToken {
        fn read_str_lit(input: &str, peek: Range<usize>)   -> Self { WsToken::StrLit(FixedStr::from(&input[peek])) }
        fn read_ident(input: &str, peek: Range<usize>)     -> Self { WsToken::Ident(FixedStr::from(&input[peek])) }
        fn read_digit_lit(input: &str, peek: Range<usize>) -> Result<Self, LexingErr> {
            match &input[peek.clone()].parse::<i16>() {
                Ok(x)  => Ok(WsToken::DigitLit(*x)),
                Err(_) => Err(LexingErr::NotADigit(peek.start)),
            } // match ..
        } // fn ..
    
        fn read_keyword(input: &str, peek: Range<usize>) -> Result<Self, LexingErr> {
            match &input[peek.clone()] {
                "let"    => Ok(WsToken::Let),
                "set"    => Ok(WsToken::Set),
                "if"     => Ok(WsToken::If),
                "while"  => Ok(WsToken::While),
                "choose" => Ok(WsToken::Choose),
                "out"    => Ok(WsToken::Out),
                "tell"   => Ok(WsToken::Tell),
                "define" => Ok(WsToken::Def),

                "integer" => Ok(WsToken::Int),
                "boolean" => Ok(WsToken::Bool),
                "void"    => Ok(WsToken::Void),
                "text"    => Ok(WsToken::Str),
                "deck"    => Ok(WsToken::Deck),

                "+" => Ok(WsToken::Plus),
                "-" => Ok(WsToken::Minus),
                "*" => Ok(WsToken::Star),
                "/" => Ok(WsToken::Slash),
                "d" => Ok(WsToken::Dice),

                "="  => Ok(WsToken::Eq),
                "/=" => Ok(WsToken::Ne),
                ">"  => Ok(WsToken::Gt),
                ">=" => Ok(WsToken::Ge),
                "<"  => Ok(WsToken::Lt),
                "<=" => Ok(WsToken::Le),

                "<-" => Ok(WsToken::InArrow),
                "->" => Ok(WsToken::OutArrow),

                "(" => Ok(WsToken::LeftParen),
                ")" => Ok(WsToken::RightParen),
                "[" => Ok(WsToken::LeftBracket),
                "]" => Ok(WsToken::RightBracket),

                ":"   => Ok(WsToken::Colon),
                ";"   => Ok(WsToken::SemiColon),
                "?"   => Ok(WsToken::Qmark),
                "~"   => Ok(WsToken::Tide),
                "|"   => Ok(WsToken::Pipe),
                "..." => Ok(WsToken::Ellipsis),

                ">>>>" => Ok(WsToken::Eos),
                ""     => Ok(WsToken::Eof),
                _      => Err(LexingErr::NotAKeyword(peek.start)),
        }} // fn ..
    } // impl ReadToken ..


    impl Display for WsToken {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", match self {
                WsToken::Ident(ident) => format!("Ident({})", ident.as_ref()),
                WsToken::StrLit(lit)  => String::from(lit.as_ref()),
                _                     => format!("{:?}", self),
            }) // write()
        } // fn ..
    } // impl ..
