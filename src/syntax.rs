//#########################
// D E P E N D E N C I E S
//#########################

    use std::error::Error;
    use std::fmt::Display;
    use std::fmt;
    use std::collections::HashMap;
    use rusty_toolkit::{
        FixedStr, L16,
        Lexer, LexingErr,
        RandBitField,
    }; // use ..

    use crate::{
        WsToken,
        Stm, Decl, Expr,
        Primitive,
    }; // use ..


//#######################
// D E F I N I T I O N S
//#######################

    /// A struct that contains the current context.
    pub struct SyntaxContext {
                   registers:      HashMap<FixedStr<L16>, Primitive>,
        pub(crate) rand_bit_field: RandBitField<u16>,
                   token_index:    usize,
                   next_tokens:    Vec<WsToken>,
    } // struct ..


    pub trait SyntaxElement {
        type Input;
        type Output;

        /// Scans the current syntax context to get a chain a elements.
        fn scan(
            context:   &mut SyntaxContext,
            input:     Self::Input,
            end_token: WsToken,
        ) -> Result<Self::Output, InterpreterErr>;
    } // trait ..


    impl Error for InterpreterErr {}
    impl Display for InterpreterErr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", match self {
                InterpreterErr::WrongToken(i, t)      => format!("Wrong token `{}` found at index `{}`!", t, i),
                InterpreterErr::WrongEof(i)           => format!("Wrong end of file at index `{}`!", i),
                InterpreterErr::ExpectedARef(i)       => format!("Expected a reference at index `{}`!", i),
                InterpreterErr::ExpectedAnExpr(i)     => format!("Expected an expression at index `{}`!", i),
                InterpreterErr::ExpectedADecl(i)      => format!("Expected a declaration at index `{}`!", i),
                InterpreterErr::ExpectedAStm(i)       => format!("Expected a statement at index `{}`!", i),
                InterpreterErr::FailedToReadPrimitive => format!("Failed to read a primitive keyword!"),
                InterpreterErr::UninitReg(n)          => format!("Uninitialised register with name `{}`!", n),
            }) // write()
        } // fn ..
    } // impl ..


    #[derive(Debug)]
    pub enum InterpreterErr {
        WrongToken            (usize, WsToken),
        WrongEof              (usize),
        ExpectedARef          (usize),
        ExpectedAnExpr        (usize),
        ExpectedADecl         (usize),
        ExpectedAStm          (usize),
        UninitReg             (FixedStr<L16>),
        FailedToReadPrimitive,
    } // enum ..


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl Default for SyntaxContext {
        fn default() -> Self {
            SyntaxContext {
                registers:      HashMap::default(),
                rand_bit_field: RandBitField::default(),
                token_index:    0usize,
                next_tokens:    Vec::default(),
            } // SyntaxContext ..
        } // fn ..
    } // impl ..


    impl SyntaxContext {
        /// The index of the last peeked token since the beginning of the context.
        pub(crate) fn token_index(&self) -> usize { self.token_index }

        /// Extends the context with newly lexed tokens.
        pub fn push(&mut self, src: &str) -> Result<(), LexingErr> { Ok(self.next_tokens.extend_from_slice(&Lexer::run(src)?)) }

        /// Moves the context to the next token.
        pub(crate) fn next(&mut self) { self.next_tokens.remove(0usize); self.token_index += 1usize; }
        
        /// Peeks the current token.
        pub(crate) fn peek(&self) -> Option<&WsToken> { self.next_tokens.first() }

        /// Returns the value stored in a given register.
        pub(crate) fn reg(
            &self,
            name: &FixedStr<L16>,
        ) -> Result<&Primitive, InterpreterErr> {
            match self.registers.get(name) {
                Some(value) => Ok(value),
                None        => Err(InterpreterErr::UninitReg(name.clone())),
            } // match ..
        } // fn ..
        
        
        /// Returns the mutable value stored in a given register.
        pub(crate) fn reg_mut(
            &mut self,
            name: &FixedStr<L16>,
        ) -> Result<&mut Primitive, InterpreterErr> {
            match self.registers.get_mut(name) {
                Some(value) => Ok(value),
                None        => Err(InterpreterErr::UninitReg(name.clone())),
            } // match ..
        } // fn ..

        
        /// Allocates a new register.
        pub(crate) fn new_reg(
            &mut self,
            id:    &FixedStr<L16>,
            value: Primitive,
        ) { self.registers.insert(id.clone(), value); }


        /// Desallocates an old register.
        pub(crate) fn del_reg(
            &mut self,
            id: &FixedStr<L16>,
        ) { self.registers.remove(id); }

        
        /// Returns an eventual reference.
        pub(crate) fn next_ref(&mut self) -> Result<FixedStr<L16>, InterpreterErr> {
            if WsToken::LeftBracket == *self.peek().unwrap() {

                self.next();
                if let Some(WsToken::Ident(name)) = self.peek() {

                    let name = name.clone();
                    self.next();
                    self.next();
                    
                    Ok(name)

                } else { Err(InterpreterErr::ExpectedARef(self.token_index)) }
            } else { Err(InterpreterErr::ExpectedARef(self.token_index)) }
        } // fn ..


        /// Returns an eventual expression.
        pub(crate) fn next_expr(
            &mut self,
            start_token: WsToken,
            end_token:   WsToken,
        ) -> Result<Expr, InterpreterErr> {

            if start_token == *self.peek().unwrap() {

                self.next();
                Ok(Expr::scan(self, None, end_token)?)

            } else { Err(InterpreterErr::ExpectedAnExpr(self.token_index)) }
        } // fn ..


        /// Returns an eventual declaration.
        pub(crate) fn next_decl(&mut self, start_token: WsToken) -> Result<Decl, InterpreterErr> {
            if self.peek() == Some(&start_token) {

                self.next();
                Ok(Decl::Expr(Expr::scan(self, None, WsToken::SemiColon)?))
            
            } else if self.peek() == Some(&WsToken::Colon) {

                self.next();
                Ok(Decl::Closure(Stm::scan(self, Vec::default(), WsToken::Eos)?))

            }  else { Err(InterpreterErr::ExpectedADecl(self.token_index)) }
        } // fn ..
    } // impl ..
