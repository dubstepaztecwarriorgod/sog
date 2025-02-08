pub struct Args {
    expression: String,
    output: String, 
    debug: bool
}

impl Args {
    pub fn new() -> Self {
        let args = std::env::args()
            .skip(1)
            .collect::<Vec<String>>();

        Args { 
            expression: args.get(0).expect("Expected a math expression").to_owned(),
            output: args.get(1).expect("Expected a output file name").to_owned(), 
            // TODO change this to a flag later
            debug: args.get(2).is_some()
        }    
    }

    pub fn expression(&self) -> &str {
        &self.expression
    }

    pub fn output(&self) -> &str {
        &self.output
    }

    pub const fn is_debug(&self) -> bool {
        self.debug
    }
}