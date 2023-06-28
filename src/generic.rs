//#########################
// D E P E N D E N C I E S
//#########################

    use crate::{
        WsToken,
        InterpreterErr,
        Val, Decl, Stm, SyntaxContext,
    }; // use ..


//#######################
// D E F I N I T I O N S
//#######################

    #[derive(Debug, Hash, PartialEq, Eq, Clone)]
    pub enum Primitive {
        Int(i16),
        Bool(bool),
        Proc(Decl),
        Void,
    } // enum ..


    #[derive(Debug, Hash, PartialEq, Eq, Clone)]
    pub enum PrimitiveId {
        Int,
        Bool,
        Proc,
        Void,
    } // enum ..


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl TryFrom<&WsToken> for PrimitiveId {
        type Error = InterpreterErr;
        fn try_from(value: &WsToken) -> Result<Self, Self::Error> {
            match value {
                WsToken::Int  => Ok(PrimitiveId::Int),
                WsToken::Bool => Ok(PrimitiveId::Bool),
                WsToken::Void => Ok(PrimitiveId::Void),
                _             => Err(InterpreterErr::FailedToReadPrimitive),
            } // match ..
        } // fn ..
    } // impl ..


    impl Primitive {
        pub(crate) fn as_val(&self, context: &mut SyntaxContext) -> Result<Val, InterpreterErr> {
            match self {
                Primitive::Int(int)   => Ok(Val::Int(*int)),
                Primitive::Bool(bool) => Ok(Val::Bool(*bool)),
                Primitive::Proc(decl) => Ok(Val::Expr(Box::new(Stm::run(context, decl)?.unwrap()))),
                Primitive::Void       => Ok(Val::Void),
            } // match ..
        } // fn ..
    } // impl ..
