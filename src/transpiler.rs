use crate::lexer::{LexerToken, LexerTokenType};
use std::{collections::{VecDeque, HashMap}, io::Write, hash::Hash};

struct Transpiler {
    pub output_code: String,
    definitions: HashMap<String, String>
}

impl Transpiler {
    fn new() -> Transpiler {
        Transpiler {
            output_code: String::new(),
            definitions: HashMap::new()
        }
    }

    fn add_word(&mut self, word: &String) {
        if !self.definitions.contains_key(word) {
            let mut value = String::from("_");
            for _idx in 0..(self.definitions.len()+10) {
                value.push('_');
            }
            self.definitions.insert(word.clone(), value.clone());
        }
        
        self.output_code += self.definitions.get(word).unwrap();
        self.output_code.push('\n');
    }

    fn write_string(&mut self, word: &String) {
        self.output_code += word;
    }

    fn output(&self, file_path: &str) {
        let mut full_file = String::from("");

        for (k, v) in &self.definitions {
            let mut define = String::from("#define ");
            define += &v;
            define.push_str(" ");
            define += &k;
            define.push('\n');
            full_file += &define;
        }

        full_file += &self.output_code;

        let mut file = std::fs::File::create(file_path).unwrap();
        file.write_all(full_file.as_bytes()).unwrap();
    }
}


pub fn transpile(tokens: &VecDeque<LexerToken>) {
    let mut transpiler = Transpiler::new();

    let mut iter = tokens.iter().peekable();

    'main_loop: loop {
        let next = iter.next();
        if let None = next {
            break 'main_loop;
        }
        let tk = next.unwrap();

        if let LexerTokenType::Keyword(kw) = &tk.token {
            if kw == "#include" {
                let mut include = String::from("#include ");
                
                let n = iter.next().unwrap();
                // "" includes
                if let LexerTokenType::Value(value) = &n.token {
                    include += &value.to_string();
                }
                // <> includes
                else {
                    let n = iter.next().unwrap();
                    iter.next().unwrap();

                    include.push('<');
                    if let LexerTokenType::Identifier(name) = &n.token {
                        include += name;
                    }
                    include.push('>');
                }
                include.push('\n');
                transpiler.write_string(&include);
                
            } 
            else if kw == "#define" {
                let mut definition = String::from("#define ");
                while iter.peek().unwrap().line == tk.line {
                    let next = iter.next().unwrap();
                    if let LexerTokenType::Identifier(name) = &next.token {
                        definition += name;
                    }
                    else if let LexerTokenType::Value(value) = &next.token {
                        definition += &value.to_string();
                    }
                    definition.push(' ');
                }
                definition.push('\n');
                transpiler.write_string(&definition); 
            }
            else {
                transpiler.add_word(kw);
            }
        }
        if let LexerTokenType::Operator(op) = &tk.token {
            transpiler.add_word(op);
        }
        if let LexerTokenType::Identifier(id) = &tk.token {
            transpiler.add_word(id);
        }
        if let LexerTokenType::Symbol(symbol) = &tk.token {
            transpiler.add_word(&symbol.to_string());
        }
        if let LexerTokenType::Value(value) = &tk.token {
            transpiler.add_word(&value.to_string());
        }
    }
    
    transpiler.output("output.c");
}
