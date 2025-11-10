use std::collections::HashMap;
use std::fs;
use std::io;
use std::process;

pub struct Compiler {
    pub current_token: String,  //for the current token being processed
    pub symbol_table: HashMap<String, String>,
    pub html_output: Vec<String>, //this is used to collect the html output during parsing
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            current_token: String::new(),
            symbol_table: HashMap::new(),
            html_output: Vec::new(),
        }
    }
}

pub struct LexicalAnalyzer {
    tokens: Vec<String>,
}

impl LexicalAnalyzer {  //splits the input into tokens and validates them against the keywords
    pub fn new(input: &str) -> Self {
        let mut tokens: Vec<String> = input
            .split_whitespace()
            .filter(|t| !t.is_empty())
            .map(|s| {
                let mut tok = s.trim().to_string();
                if tok.starts_with('#') {   //including the # symbol was not working for whatever reason so it was excluded in this code :(
                    tok.remove(0);
                }
                tok
            })
            .collect();

        tokens.reverse();   //to pop for the next token
        Self { tokens }
    }

    pub fn lookup(&self, word: &str) -> bool {  //check if the word is a valid token
        matches!(
            word.to_uppercase().as_str(),
            "HAI" | "KTHXBYE" | "OBTW" | "TLDR" | "MAEK" | "OIC"
                | "GIMMEH" | "MKAY" | "HEAD" | "TITLE" | "PARAGRAF"
                | "BOLD" | "ITALICS" | "LIST" | "ITEM" | "NEWLINE"
                | "SOUNDZ" | "VIDZ" | "I" | "HAZ" | "ITZ" | "LEMME" | "SEE"
                | _ if word.chars().all(|c| c.is_alphanumeric() || "-/:.,?".contains(c))
        )
    }

    pub fn start(&mut self) -> String { //returns first token from stack
        let candidate_token = self.tokens.pop().unwrap_or_default();
        if self.lookup(&candidate_token) {  //check token validity
            candidate_token
        } else if !candidate_token.is_empty() {
            eprintln!(
                "A lexical error was encountered. '{}' is not a recognized token.",
                candidate_token
            );
            process::exit(1);
        } else {
            eprintln!("A user error was encountered. The file is empty.");
            process::exit(1);
        }
    }

    pub fn next(&mut self) -> String {  //return the next token from stack and checks validity again
        let candidate_token = self.tokens.pop().unwrap_or_default();
        if candidate_token.is_empty() {
            return "".to_string();
        }
        if self.lookup(&candidate_token) {
            candidate_token
        } else {
            eprintln!(
                "A lexical error was encountered. '{}' is not a recognized token.",
                candidate_token
            );
            process::exit(1);
        }
    }
}

pub struct SyntaxAnalyzer<'a> {
    lexer: &'a mut LexicalAnalyzer, //reference to the lexer
    compiler: &'a mut Compiler, //reference to the compiler
}

impl<'a> SyntaxAnalyzer<'a> {
    pub fn new(lexer: &'a mut LexicalAnalyzer, compiler: &'a mut Compiler) -> Self {    //new parser with lexer and compiler reference
        Self { lexer, compiler }
    }

    pub fn next_token(&mut self) {
        self.compiler.current_token = self.lexer.next();
    }

    pub fn expect(&mut self, expected: &str) {  //checks if the current token matches the expected token
        if !self.compiler.current_token.eq_ignore_ascii_case(expected) {
            eprintln!(
                "A syntax error was encountered. '{}' was found when '{}' was expected.",
                self.compiler.current_token, expected
            );
            process::exit(1);
        }
        self.next_token();
    }

    pub fn process_comment(&mut self) {
        while self.compiler.current_token.eq_ignore_ascii_case("OBTW") {
            loop {
                self.next_token();
                if self.compiler.current_token.eq_ignore_ascii_case("TLDR")
                    || self.compiler.current_token.is_empty()
                {
                    break;
                }
            }
            self.next_token();
        }
    }

    pub fn lolcode(&mut self) { //lolcode grammar order
        self.process_comment();
        self.expect("HAI"); //Program NEEDS to start with HAI
        self.process_comment();
        self.head();
        self.body();
        self.process_comment();
        self.expect("KTHXBYE"); //Program NEEDS to end with KTHXBYE
    }

    pub fn head(&mut self) {    //parsing the head section
        if self.compiler.current_token.eq_ignore_ascii_case("MAEK") {
            self.next_token();
            if self.compiler.current_token.eq_ignore_ascii_case("HEAD") {
                self.next_token();
                if self.compiler.current_token.eq_ignore_ascii_case("GIMMEH") {
                    self.next_token();
                    self.expect("TITLE");
                    while !self.compiler.current_token.eq_ignore_ascii_case("MKAY")
                        && !self.compiler.current_token.is_empty()
                    {
                        self.next_token();
                    }
                    self.expect("MKAY");    //end of TITLE
                }
                self.expect("OIC"); //end of HEAD
            }
        }
    }

    pub fn body(&mut self) {    //parse the body (paragraphs, lists, variables, media)
        while !self.compiler.current_token.is_empty()
            && !self.compiler.current_token.eq_ignore_ascii_case("KTHXBYE")
        {
            self.process_comment();
            match self.compiler.current_token.to_uppercase().as_str() {
                "MAEK" => self.paragraph_or_list(),
                "GIMMEH" => self.media_or_text(),
                "I" | "LEMME" => self.variable_statement(),
                _ => self.next_token(),
            }
        }
    }

    pub fn paragraph_or_list(&mut self) {   //parse a paragraph or list, also convert to HTML
        self.next_token();
        match self.compiler.current_token.to_uppercase().as_str() {
            "PARAGRAF" => {
                self.next_token();
                while !self.compiler.current_token.eq_ignore_ascii_case("OIC")
                    && !self.compiler.current_token.is_empty()
                {
                    self.compiler
                        .html_output
                        .push(format!("<p>{}</p>", self.compiler.current_token));
                    self.next_token();
                }
                self.expect("OIC");
            }
            "LIST" => {
                self.next_token();
                self.compiler.html_output.push("<ul>".to_string());
                while self.compiler.current_token.eq_ignore_ascii_case("ITEM") {
                    self.next_token();
                    self.compiler
                        .html_output
                        .push(format!("<li>{}</li>", self.compiler.current_token));
                    self.next_token();
                }
                self.compiler.html_output.push("</ul>".to_string());
                self.expect("OIC");
            }
            _ => {
                eprintln!(
                    "A syntax error was encountered. '{}' was found when 'PARAGRAF' or 'LIST' was expected.",
                    self.compiler.current_token
                );
                process::exit(1);
            }
        }
    }

    pub fn media_or_text(&mut self) {   //parse media/text tags (bold, italics, newline, soundz, vidz)
        self.next_token();
        let media_type = self.compiler.current_token.to_uppercase();
        self.next_token();

        let mut content = String::new();
        while !self.compiler.current_token.eq_ignore_ascii_case("MKAY")
            && !self.compiler.current_token.is_empty()
        {
            content.push_str(&self.compiler.current_token);
            content.push(' ');
            self.next_token();
        }
        let content = content.trim();

        match media_type.as_str() {
            "BOLD" => self.compiler.html_output.push(format!("<b>{}</b>", content)),
            "ITALICS" => self.compiler.html_output.push(format!("<i>{}</i>", content)),
            "NEWLINE" => self.compiler.html_output.push("<br>".to_string()),
            "SOUNDZ" => self.compiler.html_output.push(format!(
                "<audio src=\"{}\" controls></audio>",
                content
            )),
            "VIDZ" => {
                let url = content.trim();
                self.compiler.html_output.push(format!(
                    "<a href=\"{}\" target=\"_blank\">{}</a>",
                    url, url
                ));
            }
            _ => {
                eprintln!(
                    "A syntax error was encountered. '{}' was found when a media keyword was expected.",
                    media_type
                );
                process::exit(1);
            }
        }

        self.expect("MKAY");
    }

    pub fn variable_statement(&mut self) {  //parses variable declarations and references
        let first = self.compiler.current_token.to_uppercase();
        self.next_token();

        match first.as_str() {
            "I" => {
                if self.compiler.current_token.eq_ignore_ascii_case("HAZ") {
                    self.next_token();
                    let var_name = self.compiler.current_token.clone();
                    self.next_token();
                    if self.compiler.current_token.eq_ignore_ascii_case("ITZ") {
                        self.next_token();
                        let value = self.compiler.current_token.clone();
                        self.next_token();
                        self.expect("MKAY");
                        self.compiler.symbol_table.insert(var_name, value); //store variables here
                    }
                }
            }
            "LEMME" => {
                if self.compiler.current_token.eq_ignore_ascii_case("SEE") {
                    self.next_token();
                    let var_name = self.compiler.current_token.clone();
                    if !self.compiler.symbol_table.contains_key(&var_name) {
                        eprintln!(
                            "A semantic error was encountered. The variable '{}' was used before being defined.",
                            var_name
                        );
                        process::exit(1);
                    }
                    self.next_token();
                    self.expect("MKAY");
                }
            }
            _ => {}
        }
    }
}

fn main() {
    println!("Please choose a test case number (1–13): ");  //I didn't know what format for choosing the file so choose between the available test cases
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice: usize = input.trim().parse().unwrap_or(1);  //parse into an integer, or default to 1

    let filename = if choice == 1 {
        "tests/Test1.txt".to_string()
    } else {
        format!("tests/Test{}.lol", choice)
    };

    if !filename.ends_with(".lol") {    //only allows files ending in .lol, .txt is a no-no
        eprintln!("A file error was encountered. Only '.lol' files are accepted.");
        process::exit(1);
    }

    let content = fs::read_to_string(&filename).unwrap_or_else(|_| {
        eprintln!("A file error was encountered. Could not read '{}'.", filename);
        process::exit(1);
    });

    let mut compiler = Compiler::new(); //initialize compiler
    let mut lexer = LexicalAnalyzer::new(&content); //initialize lexer
    compiler.current_token = lexer.start(); //starts lexing processes

    let mut parser = SyntaxAnalyzer::new(&mut lexer, &mut compiler);    //initialize parser
    parser.lolcode();   //parse the lolcode

    let html_filename = filename.replace(".lol", ".html");  //writes html output
    let mut html_file_content = "<html><body>".to_string();
    html_file_content.push_str(&compiler.html_output.join("\n"));
    html_file_content.push_str("</body></html>");
    fs::write(&html_filename, html_file_content).unwrap();

    println!("The file '{}' follows the LOLCODE grammar!", filename);

    #[cfg(target_os = "macos")] //specifically for Chrome on MacOS
    {
        use std::process::Command;
        Command::new("open")
            .arg("-a")
            .arg("Google Chrome")
            .arg(&html_filename)
            .spawn()
            .expect("Failed to open Chrome");
    }
}
