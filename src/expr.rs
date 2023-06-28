//#########################
// D E P E N D E N C I E S
//#########################

    use rusty_toolkit::{FixedStr, L16};

    use crate::{
        SyntaxContext, SyntaxElement,
        InterpreterErr,
        WsToken,
        PrimitiveId, Primitive,
    }; // use ..


//#######################
// D E F I N I T I O N S
//#######################

    #[derive(Debug, Hash, PartialEq, Eq, Clone)] pub enum Expr  { Val(Val), Op(Op), }
    #[derive(Debug, Hash, PartialEq, Eq, Clone)] pub enum Op    { Neg(Val), Add(Val, Box<Expr>), Sub(Val, Box<Expr>), Mul(Val, Box<Expr>), Div(Val, Box<Expr>), Dice(Val, Box<Expr>, Option<CmpTo>), Cmp(CmpTo, Box<Expr>)}
    #[derive(Debug, Hash, PartialEq, Eq, Clone)] pub enum Val   { Int(i16), Bool(bool), Ref(PrimitiveId, FixedStr<L16>), Expr(Box<Expr>), Void, }
    #[derive(Debug, Hash, PartialEq, Eq, Clone)] pub enum CmpTo { Eq(Val), Ne(Val), Gt(Val), Ge(Val), Lt(Val), Le(Val) }


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl SyntaxElement for Expr {
        type Input  = Option<Self>;
        type Output = Self;

        fn scan(
            context:   &mut SyntaxContext,
            input:     Self::Input,
            end_token: WsToken,
        ) -> Result<Self::Output, InterpreterErr> {

            fn op<F: Fn(Expr, Val) -> Expr>(
                context:   &mut SyntaxContext,
                end_token: WsToken,
                a:         Expr,
                func:      F,
            ) -> Result<Expr, InterpreterErr> { context.next(); let b = next_value(context).unwrap(); Expr::scan(context, Some(func(a, b)), end_token) }


            fn next_value(context: &mut SyntaxContext) -> Result<Val, InterpreterErr> {
                let a = match *context.peek().unwrap() {

                    WsToken::DigitLit(a) => { context.next(); Ok(Val::Int(a)) },
                    WsToken::Ellipsis    => { context.next(); Ok(Val::Void) },

                    WsToken::Minus       => { context.next(); let b = next_value(context)?; Ok(Val::Expr(Box::new(Expr::Op(Op::Neg(b))))) }
                    WsToken::Throw       => { context.next(); let b = next_value(context)?; Ok(Val::Int (Expr::Val(b).as_int(context)?)) }

                    WsToken::LeftParen   => { context.next(); Ok(Val::Expr(Box::new(Expr::scan(context, None, WsToken::RightParen)?))) },
                    _                    => { let id = PrimitiveId::try_from(context.peek().unwrap())?; context.next(); let name = context.next_ref()?; Ok(Val::Ref(id, name)) },

                }?; // let ..


                match *context.peek().unwrap() {
                    WsToken::Star  => { context.next(); let b = next_value(context)?; Ok(Val::Expr(Box::new(Expr::Op(Op::Mul(b, Box::new(Expr::Val(a)))))))  }
                    WsToken::Slash => { context.next(); let b = next_value(context)?; Ok(Val::Expr(Box::new(Expr::Op(Op::Div(b, Box::new(Expr::Val(a))))))) }
                    WsToken::Dice  => { context.next(); let b = next_value(context)?; match *context.peek().unwrap() {
                        WsToken::Colon => { context.next(); let token = context.peek().unwrap(); match token {
                            WsToken::Eq => { context.next(); let c = next_value(context)?; Ok(Val::Expr(Box::new(Expr::Op(Op::Dice(b, Box::new(Expr::Val(a)), Some(CmpTo::Eq(c))))))) },
                            WsToken::Ne => { context.next(); let c = next_value(context)?; Ok(Val::Expr(Box::new(Expr::Op(Op::Dice(b, Box::new(Expr::Val(a)), Some(CmpTo::Ne(c))))))) },
                            WsToken::Gt => { context.next(); let c = next_value(context)?; Ok(Val::Expr(Box::new(Expr::Op(Op::Dice(b, Box::new(Expr::Val(a)), Some(CmpTo::Gt(c))))))) },
                            WsToken::Ge => { context.next(); let c = next_value(context)?; Ok(Val::Expr(Box::new(Expr::Op(Op::Dice(b, Box::new(Expr::Val(a)), Some(CmpTo::Ge(c))))))) },
                            WsToken::Lt => { context.next(); let c = next_value(context)?; Ok(Val::Expr(Box::new(Expr::Op(Op::Dice(b, Box::new(Expr::Val(a)), Some(CmpTo::Lt(c))))))) },
                            WsToken::Le => { context.next(); let c = next_value(context)?; Ok(Val::Expr(Box::new(Expr::Op(Op::Dice(b, Box::new(Expr::Val(a)), Some(CmpTo::Le(c))))))) },
                            _           => Err(InterpreterErr::WrongToken(context.token_index(), token.clone())),
                            }}, // => ..
                        _ => Ok(Val::Expr(Box::new(Expr::Op(Op::Dice(b, Box::new(Expr::Val(a)), None))))),
                    }}, // => ..
                    _  => Ok(a),
                } // match ..
            } // fn ..


            if *context.peek().unwrap() == end_token {
                context.next();
                match input {
                    Some(input) => Ok(input),
                    None        => Err(InterpreterErr::ExpectedAnExpr(context.token_index())),
                } // match ..
            } else {
                match input {
                    
                    Some(expr) => { let token = context.peek().unwrap(); match token {
                        WsToken::Plus  => op(context, end_token, expr, |a, b| Expr::Op(Op::Add(b, Box::new(a)))),
                        WsToken::Minus => op(context, end_token, expr, |a, b| Expr::Op(Op::Sub(b, Box::new(a)))),
                        WsToken::Star  => op(context, end_token, expr, |a, b| Expr::Op(Op::Mul(b, Box::new(a)))),
                        WsToken::Slash => op(context, end_token, expr, |a, b| Expr::Op(Op::Div(b, Box::new(a)))),
                        WsToken::Dice  => op(context, end_token, expr, |a, b| Expr::Op(Op::Dice(b, Box::new(a), None))),
                        WsToken::Eq    => op(context, end_token, expr, |a, b| Expr::Op(Op::Cmp(CmpTo::Eq(b), Box::new(a)))),
                        WsToken::Ne    => op(context, end_token, expr, |a, b| Expr::Op(Op::Cmp(CmpTo::Ne(b), Box::new(a)))),
                        WsToken::Gt    => op(context, end_token, expr, |a, b| Expr::Op(Op::Cmp(CmpTo::Gt(b), Box::new(a)))),
                        WsToken::Ge    => op(context, end_token, expr, |a, b| Expr::Op(Op::Cmp(CmpTo::Ge(b), Box::new(a)))),
                        WsToken::Lt    => op(context, end_token, expr, |a, b| Expr::Op(Op::Cmp(CmpTo::Lt(b), Box::new(a)))),
                        WsToken::Le    => op(context, end_token, expr, |a, b| Expr::Op(Op::Cmp(CmpTo::Le(b), Box::new(a)))),
                        _              => Err(InterpreterErr::WrongToken(context.token_index(), token.clone())),
                    }}, // => ..
                    None => match *context.peek().unwrap() {
                        _ => { let val = next_value(context).unwrap(); Self::scan(context, Some(Expr::Val(val)), end_token) },
                    }, // => ..
            }} // if ..
        } // fn ..
    } // impl ..


    impl Expr {
        pub(crate) fn as_val(
            &self,
            context: &mut SyntaxContext,
        ) -> Result<Val, InterpreterErr> {

            match self {
                Expr::Val(value) => match value {
                    Val::Expr(expr)    => expr.as_val(context),
                    Val::Ref(id, name) => match (id, context.reg(name)?.clone()) {
                        (PrimitiveId::Int,  primitive) => primitive.as_val(context)?.as_int(context),
                        (PrimitiveId::Bool, primitive) => primitive.as_val(context)?.as_bool(context),
                        (PrimitiveId::Void, primitive) => primitive.as_val(context)?.as_void(context),
                        (PrimitiveId::Proc, primitive) => primitive.as_val(context)?.as_proc(context),
                    }, // => ..
                    _ => Ok(value.clone()),
                }, // match ..
                Expr::Op(opr) => Ok(match opr {
                    Op::Neg(a)               => Val::Int(-Expr::Val(a.clone()).as_int(context)?),
                    Op::Add(a, b)            => Val::Int( b.as_int(context)? + Expr::Val(a.clone()).as_int(context)?),
                    Op::Sub(a, b)            => Val::Int( b.as_int(context)? - Expr::Val(a.clone()).as_int(context)?),
                    Op::Mul(a, b)            => Val::Int( b.as_int(context)? * Expr::Val(a.clone()).as_int(context)?),
                    Op::Div(a, b)            => Val::Int( b.as_int(context)? / Expr::Val(a.clone()).as_int(context)?),
                    Op::Dice(a, b, c)        => Val::Int( b.dice_throw(context, Expr::Val(a.clone()), c.clone())?),
                    Op::Cmp(CmpTo::Eq(a), b) => Val::Bool(b.as_int(context)? == Expr::Val(a.clone()).as_int(context)?),
                    Op::Cmp(CmpTo::Ne(a), b) => Val::Bool(b.as_int(context)? != Expr::Val(a.clone()).as_int(context)?),
                    Op::Cmp(CmpTo::Gt(a), b) => Val::Bool(b.as_int(context)? >= Expr::Val(a.clone()).as_int(context)?),
                    Op::Cmp(CmpTo::Ge(a), b) => Val::Bool(b.as_int(context)? >  Expr::Val(a.clone()).as_int(context)?),
                    Op::Cmp(CmpTo::Lt(a), b) => Val::Bool(b.as_int(context)? <  Expr::Val(a.clone()).as_int(context)?),
                    Op::Cmp(CmpTo::Le(a), b) => Val::Bool(b.as_int(context)? <= Expr::Val(a.clone()).as_int(context)?),
                }), // => ..
            } // match ..
        } // fn ..


        /// Gets an integer output from an expression.
        fn as_int(
            &self,
            context: &mut SyntaxContext,
        ) -> Result<i16, InterpreterErr> {

            fn value_to_i16(context: &mut SyntaxContext, value: Val) -> Result<i16, InterpreterErr> {
                match value {
                    Val::Int(int)     => Ok(int),
                    Val::Bool(bool)   => Ok(if bool { 1i16 } else { 0i16 }),
                    Val::Expr(expr)   => expr.as_int(context),
                    Val::Ref(_, name) => { let value = context.reg(&name)?.clone().as_val(context)?; value_to_i16(context, value) },
                    Val::Void         => Ok(0i16),
                } // match ..
            } // fn ..


            match self {
                Expr::Val(value) => value_to_i16(context, value.clone()),
                Expr::Op(opr) => Ok(match opr {
                    Op::Neg(a)               => -Expr::Val(a.clone()).as_int(context)?,
                    Op::Add(a, b)            =>  b.as_int(context)? + Expr::Val(a.clone()).as_int(context)?,
                    Op::Sub(a, b)            =>  b.as_int(context)? - Expr::Val(a.clone()).as_int(context)?,
                    Op::Mul(a, b)            =>  b.as_int(context)? * Expr::Val(a.clone()).as_int(context)?,
                    Op::Div(a, b)            =>  b.as_int(context)? / Expr::Val(a.clone()).as_int(context)?,
                    Op::Dice(a, b, c)        =>  b.dice_throw(context, Expr::Val(a.clone()), c.clone())?,
                    Op::Cmp(CmpTo::Eq(a), b) => if b.as_int(context)? == Expr::Val(a.clone()).as_int(context)? { 1i16 } else { 0i16 },
                    Op::Cmp(CmpTo::Ne(a), b) => if b.as_int(context)? != Expr::Val(a.clone()).as_int(context)? { 1i16 } else { 0i16 },
                    Op::Cmp(CmpTo::Gt(a), b) => if b.as_int(context)? >= Expr::Val(a.clone()).as_int(context)? { 1i16 } else { 0i16 },
                    Op::Cmp(CmpTo::Ge(a), b) => if b.as_int(context)? >  Expr::Val(a.clone()).as_int(context)? { 1i16 } else { 0i16 },
                    Op::Cmp(CmpTo::Lt(a), b) => if b.as_int(context)? <  Expr::Val(a.clone()).as_int(context)? { 1i16 } else { 0i16 },
                    Op::Cmp(CmpTo::Le(a), b) => if b.as_int(context)? <= Expr::Val(a.clone()).as_int(context)? { 1i16 } else { 0i16 },
                }), // => ..
            } // match ..
        } // fn ..


        /// Gets a boolean output from an expression.
        pub(crate) fn as_bool(
            &self,
            context: &mut SyntaxContext,
        ) -> Result<bool, InterpreterErr> {

            fn value_to_bool(context: &mut SyntaxContext, value: Val) -> Result<bool, InterpreterErr> {
                match value {
                    Val::Int(int)      => Ok(int != 0i16),
                    Val::Bool(bool)    => Ok(bool),
                    Val::Expr(expr)    => expr.as_bool(context),
                    Val::Ref(_, name)  => { let value = context.reg(&name)?.clone().as_val(context)?; value_to_bool(context, value) },
                    Val::Void          => Ok(false),
                } // match ..
            } // fn ..


            match self {
                Expr::Val(value) => value_to_bool(context, value.clone()),
                Expr::Op(op) => Ok(match op {
                    Op::Neg(a)               => !Expr::Val(a.clone()).as_bool(context)?,
                    Op::Add(a, b)            => b.as_bool(context)? |  Expr::Val(a.clone()).as_bool(context)?,
                    Op::Sub(a, b)            => b.as_bool(context)? & !Expr::Val(a.clone()).as_bool(context)?,
                    Op::Mul(a, b)            => b.as_bool(context)? &  Expr::Val(a.clone()).as_bool(context)?,
                    Op::Div(_, b)            => b.as_bool(context)?,
                    Op::Dice(_, b, _)        => b.as_bool(context)?,
                    Op::Cmp(CmpTo::Eq(a), b) => b.as_bool(context)? == Expr::Val(a.clone()).as_bool(context)?,
                    Op::Cmp(CmpTo::Ne(a), b) => b.as_bool(context)? != Expr::Val(a.clone()).as_bool(context)?,
                    Op::Cmp(CmpTo::Gt(a), b) => b.as_int(context)?  >= Expr::Val(a.clone()).as_int(context)?,
                    Op::Cmp(CmpTo::Ge(a), b) => b.as_int(context)?  >  Expr::Val(a.clone()).as_int(context)?,
                    Op::Cmp(CmpTo::Lt(a), b) => b.as_int(context)?  <  Expr::Val(a.clone()).as_int(context)?,
                    Op::Cmp(CmpTo::Le(a), b) => b.as_int(context)?  <= Expr::Val(a.clone()).as_int(context)?,
                }), // => ..
            } // match ..
        } // fn ..
        

        /// Gets a dice throw output from an expression.
        fn dice_throw(
            &self,
            context: &mut SyntaxContext,
            face:    Expr,
            cmp:     Option<CmpTo>,
        ) -> Result<i16, InterpreterErr> {
    
            let face    = face.as_int(context)?;
            let n       = self.as_int(context)?;
    
    
            (0i16..n).map(|_| {
                let dice = context.rand_bit_field.generate_irange(1i16..=face);
                match cmp.clone() {
                    Some(CmpTo::Eq(cmp)) => if dice == Expr::Val(cmp).as_int(context)? { Ok(1i16) } else { Ok(0i16) },
                    Some(CmpTo::Ne(cmp)) => if dice != Expr::Val(cmp).as_int(context)? { Ok(1i16) } else { Ok(0i16) },
                    Some(CmpTo::Gt(cmp)) => if dice >  Expr::Val(cmp).as_int(context)? { Ok(1i16) } else { Ok(0i16) },
                    Some(CmpTo::Ge(cmp)) => if dice >= Expr::Val(cmp).as_int(context)? { Ok(1i16) } else { Ok(0i16) },
                    Some(CmpTo::Lt(cmp)) => if dice <  Expr::Val(cmp).as_int(context)? { Ok(1i16) } else { Ok(0i16) },
                    Some(CmpTo::Le(cmp)) => if dice <= Expr::Val(cmp).as_int(context)? { Ok(1i16) } else { Ok(0i16) },
                    None                 => Ok(dice),
                } // match ..
            }).sum()
        } // fn ..
    } // impl ..


    impl Val {
        pub(crate) fn unwraped(&self, context: &mut SyntaxContext) -> Result<Self, InterpreterErr> {
            match self {
                Val::Ref(_, name) => Ok(context.reg(&name)?.clone().as_val(context)?),
                _                 => Ok(self.clone()),
            } // match ..
        } // fn ..


        pub(crate) fn as_primitive(&self, context: &mut SyntaxContext) -> Result<Primitive, InterpreterErr> {
            match self {
                Val::Int(int)     => Ok(Primitive::Int(*int)),
                Val::Bool(bool)   => Ok(Primitive::Bool(*bool)),
                Val::Expr(expr)   => Ok(expr.as_val(context)?.as_primitive(context)?),
                Val::Ref(_, name) => Ok(context.reg(&name)?.clone()),
                Val::Void         => Ok(Primitive::Void),
            } // match ..
        } // fn ..



        fn as_int(&self, context: &mut SyntaxContext) -> Result<Self, InterpreterErr> {
            match self {
                Val::Int(int)     => Ok(Val::Int(*int)),
                Val::Bool(bool)   => Ok(Val::Int(if *bool { 1i16 } else { 0i16 })),
                Val::Expr(expr)   => Ok(expr.as_val(context)?.as_int(context)?),
                Val::Ref(_, name) => Ok(context.reg(&name)?.clone().as_val(context)?.as_int(context)?),
                Val::Void         => Ok(Val::Int(0i16)),
            } // match ..
        } // fn ..


        fn as_bool(&self, context: &mut SyntaxContext) -> Result<Self, InterpreterErr> {
            match self {
                Val::Int(int)     => Ok(Val::Bool(*int != 0i16)),
                Val::Bool(bool)   => Ok(Val::Bool(*bool)),
                Val::Expr(expr)   => Ok(expr.as_val(context)?.as_bool(context)?),
                Val::Ref(_, name) => Ok(context.reg(&name)?.clone().as_val(context)?.as_bool(context)?),
                Val::Void         => Ok(Val::Bool(false)),
            } // match ..
        } // fn ..


        fn as_void(&self, _: &mut SyntaxContext) -> Result<Self, InterpreterErr> {
            match self {
                Val::Int(..)  => Ok(Val::Void),
                Val::Bool(..) => Ok(Val::Void),
                Val::Expr(..) => Ok(Val::Void),
                Val::Ref(..)  => Ok(Val::Void),
                Val::Void     => Ok(Val::Void),
            } // match ..
        } // fn ..


        fn as_proc(&self, _: &mut SyntaxContext) -> Result<Self, InterpreterErr> {
            Ok(Val::Expr(Box::new(Expr::Val(self.clone()))))
        } // fn ..
    } // impl ..
