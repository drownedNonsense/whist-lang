//#########################
// D E P E N D E N C I E S
//#########################

    use whist_lang::{
        WsToken,
        Stm, Decl,
        SyntaxContext, SyntaxElement,
    }; // use ..

    use std::env;
    use std::fs::read_to_string;
    use std::io::stdin;


//#######################
// D E F I N I T I O N S
//#######################

    fn main() {

        let mut context = SyntaxContext::default();
        let     args    = env::args().collect::<Vec<String>>();


        if let Some(action) = args.get(1usize) {
            match action.as_str() {
                "read" => {
                    if let Some(arg) = args.get(2usize) {
                        match match arg.as_str() {
                            "line" => { context.push(args.get(3usize).unwrap_or(&String::from(""))) },
                            "file" => { context.push(&read_to_string(args.get(3usize).unwrap_or(&String::from(""))).unwrap_or_default()) }
                            _      => ( Ok(()) ),
                        } {
                            Ok(_)    => (),
                            Err(err) => eprintln!("ERROR: {}", err),
                        } // match ..
                    } // if ..
                }, // => ..
                _=> (),
            } // match ..
        } // if ..

        
        match match Stm::scan(&mut context, Vec::default(), WsToken::Eof) {
            Ok(stms) => { Stm::run(&mut context, &Decl::Closure(stms)) },
            Err(err) => { Err(err) },
        } {
            Ok(_)    => (),
            Err(err) => eprintln!("ERROR: {}", err),
        } // match ..


        let mut buffer  = String::new();
        while buffer   != "quit" {

            stdin().read_line(&mut buffer).expect("ERROR: Failed to read line!");

            match context.push(&buffer) {
                Ok(_)    => (),
                Err(err) => { eprintln!("ERROR: {}", err); break; },
            } // match ..

            match match Stm::scan(&mut context, Vec::default(), WsToken::Eof) {
                Ok(stms) => { Stm::run(&mut context, &Decl::Closure(stms)) },
                Err(err) => { Err(err) },
            } {
                Ok(_)    => (),
                Err(err) => { eprintln!("ERROR: {}", err); break; }
            } // match ..


            buffer = String::new();

        } // while ..
    } // fn ..
