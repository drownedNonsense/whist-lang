//#########################
// D E P E N D E N C I E S
//#########################

    use rusty_toolkit::{FixedStr, L16};

    use crate::{
        WsToken,
        Expr,
        SyntaxContext, SyntaxElement,
        InterpreterErr,
        Primitive,
    }; // use ..


//#######################
// D E F I N I T I O N S
//#######################

    #[derive(Debug, Hash, PartialEq, Eq, Clone)]
    /// The statement enumerator.
    pub enum Stm {
        RegLet (FixedStr<L16>, Decl),
        RegSet (FixedStr<L16>, Decl),
        While  (Expr, Decl),
        If     (Expr, Decl),
        Tell   (Expr),
        Out    (Expr),
    } // enum Stm


    #[derive(Debug, Hash, PartialEq, Eq, Clone)]
    /// The declaration enumerator.
    pub enum Decl {
        Expr    (Expr),
        Closure (Vec<Stm>),
    } // enum ..
    


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl SyntaxElement for Stm {
        type Input  = Vec<Self>;
        type Output = Vec<Self>;

        fn scan(
            context:   &mut SyntaxContext,
            input:     Self::Input,
            end_token: WsToken,
        ) -> Result<Self::Output, InterpreterErr> {

            if let Some(token) = context.peek() {
                if token == &end_token { context.next(); Ok(input) } else {
                    match match *context.peek().unwrap() {
                        WsToken::Let   => { context.next(); let name = context.next_ref()?; let decl = context.next_decl(WsToken::InArrow)?;                                                       Ok(Some(Stm::RegLet(name, decl))) }
                        WsToken::Set   => { context.next(); let name = context.next_ref()?; let decl = context.next_decl(WsToken::InArrow)?;                                                       Ok(Some(Stm::RegSet(name, decl))) }
                        WsToken::Def   => { context.next(); let name = context.next_ref()?; let decl = context.next_decl(WsToken::InArrow)?; context.new_reg(&name, crate::Primitive::Proc(decl)); Ok(None) }
                        WsToken::Tell  => { context.next(); let expr = context.next_expr(WsToken::OutArrow, WsToken::SemiColon)?;                                                                  Ok(Some(Stm::Tell(expr))) }
                        WsToken::If    => { context.next(); let expr = context.next_expr(WsToken::Pipe, WsToken::Pipe)?; let decl = context.next_decl(WsToken::OutArrow)?;                         Ok(Some(Stm::If(expr, decl))) }
                        WsToken::While => { context.next(); let expr = context.next_expr(WsToken::Pipe, WsToken::Pipe)?; let decl = context.next_decl(WsToken::OutArrow)?;                         Ok(Some(Stm::While(expr, decl))) }
                        WsToken::Out   => { context.next(); let expr = context.next_expr(WsToken::OutArrow, WsToken::SemiColon)?;                                                                  Ok(Some(Stm::Out(expr))) }
                        _              => Err(InterpreterErr::WrongToken(context.token_index(), context.peek().unwrap().clone())),
                    }? {
                        Some(stm) => { let mut output = input; output.push(stm); Self::scan(context, output, end_token) },
                        None      => Ok(input),
                    }
                } // if ..
            } else { Err(InterpreterErr::WrongEof(context.token_index())) } // if ..
        } // fn ..
    } // impl ..


    impl Stm {
        /// Runs through a declaration to output an eventual expression.
        pub fn run (
            context: &mut SyntaxContext,
            decl:    &Decl,
        ) -> Result<Option<Expr>, InterpreterErr> {
            match decl {
                Decl::Expr(expr)    => Ok(Some(expr.clone())),
                Decl::Closure(stms) => {
                    match stms.iter().find_map(|stm| match stm.as_expr(context) {
                        Ok(Some(out)) => Some(Ok(Some(out))),
                        Err(err)      => Some(Err(err)),
                        _             => None,
                    }) {
                        Some(result) => result,
                        None         => Ok(None),
                    } // match ..
                } // => ..
            } // => ..
        } // fn ..


        /// Returns an eventual expression out of a statement.
        fn as_expr(&self, context: &mut SyntaxContext) -> Result<Option<Expr>, InterpreterErr> {
            match self {
                Stm::RegLet(name, decl) => { let expr = Self::run(context, decl).unwrap().unwrap(); let value = expr.as_val(context)?.as_primitive(context)?; match value { Primitive::Void => context.del_reg(name), _ =>  context.new_reg(name, value),   } Ok(None) },
                Stm::RegSet(name, decl) => { let expr = Self::run(context, decl).unwrap().unwrap(); let value = expr.as_val(context)?.as_primitive(context)?; match value { Primitive::Void => context.del_reg(name), _ => *context.reg_mut(name)? = value, } Ok(None) },
                Stm::If(expr, decl)     => { if Expr::Val(expr.as_val(context)?).as_bool(context)? { Self::run(context, decl) } else { Ok(None) }}
                Stm::While(expr, decl)  => { while Expr::Val(expr.as_val(context)?).as_bool(context)? { Self::run(context, decl)?; }  Ok(None) }
                Stm::Out(expr)          => { Ok(Some(expr.clone())) },
                Stm::Tell(expr)         => { println!("{:?}", expr.as_val(context)?.unwraped(context)?.as_primitive(context)?); Ok(None) },
            } // match ..
        } // fn ..
    } // impl ..
