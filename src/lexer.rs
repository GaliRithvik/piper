#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(f64),
    Str(String),
    Bool(bool),
    FStringLit(String),

    // Identifier
    Ident(String),

    // Keywords
    Let, Fn, Return,
    If, Elif, Else,
    For, While, In,
    And, Or, Not,
    None,

    // Operators
    Plus, Minus, Star, Slash, Percent, StarStar,
    PlusAssign, MinusAssign, StarAssign, SlashAssign,
    Eq, NotEq, Lt, Gt, LtEq, GtEq,
    Assign,
    Pipe,

    // Punctuation
    LParen, RParen,
    LBracket, RBracket,
    Comma, Colon, Dot,

    // Layout
    Newline, Indent, Dedent, EOF,
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut indent_stack: Vec<usize> = vec![0];
    let mut bracket_depth: usize = 0;

    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if bracket_depth == 0 {
            let indent = line.len() - trimmed.len();
            let current = *indent_stack.last().unwrap();
            if indent > current {
                indent_stack.push(indent);
                tokens.push(Token::Indent);
            } else if indent < current {
                while *indent_stack.last().unwrap() > indent {
                    indent_stack.pop();
                    tokens.push(Token::Dedent);
                }
            }
        }

        tokenize_line(trimmed, &mut tokens, &mut bracket_depth);

        if bracket_depth == 0 {
            tokens.push(Token::Newline);
        }
    }

    while indent_stack.len() > 1 {
        indent_stack.pop();
        tokens.push(Token::Dedent);
    }

    tokens.push(Token::EOF);
    tokens
}

fn tokenize_line(line: &str, tokens: &mut Vec<Token>, bracket_depth: &mut usize) {
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' => { i += 1; }
            '#' => break,

            '[' => { tokens.push(Token::LBracket); *bracket_depth += 1; i += 1; }
            ']' => { tokens.push(Token::RBracket); if *bracket_depth > 0 { *bracket_depth -= 1; } i += 1; }
            '(' => { tokens.push(Token::LParen); i += 1; }
            ')' => { tokens.push(Token::RParen); i += 1; }
            ',' => { tokens.push(Token::Comma); i += 1; }
            ':' => { tokens.push(Token::Colon); i += 1; }
            '.' => { tokens.push(Token::Dot); i += 1; }
            ';' => { i += 1; } // semicolons are optional statement terminators

            '+' => {
                if i + 1 < chars.len() && chars[i+1] == '=' { tokens.push(Token::PlusAssign); i += 2; }
                else { tokens.push(Token::Plus); i += 1; }
            }
            '-' => {
                if i + 1 < chars.len() && chars[i+1] == '=' { tokens.push(Token::MinusAssign); i += 2; }
                else { tokens.push(Token::Minus); i += 1; }
            }
            '*' => {
                if i + 1 < chars.len() && chars[i+1] == '*' { tokens.push(Token::StarStar); i += 2; }
                else if i + 1 < chars.len() && chars[i+1] == '=' { tokens.push(Token::StarAssign); i += 2; }
                else { tokens.push(Token::Star); i += 1; }
            }
            '/' => {
                if i + 1 < chars.len() && chars[i+1] == '=' { tokens.push(Token::SlashAssign); i += 2; }
                else { tokens.push(Token::Slash); i += 1; }
            }
            '%' => { tokens.push(Token::Percent); i += 1; }
            '|' => {
                if i + 1 < chars.len() && chars[i+1] == '>' { tokens.push(Token::Pipe); i += 2; }
                else { i += 1; }
            }
            '=' => {
                if i + 1 < chars.len() && chars[i+1] == '=' { tokens.push(Token::Eq); i += 2; }
                else { tokens.push(Token::Assign); i += 1; }
            }
            '!' => {
                if i + 1 < chars.len() && chars[i+1] == '=' { tokens.push(Token::NotEq); i += 2; }
                else { i += 1; }
            }
            '<' => {
                if i + 1 < chars.len() && chars[i+1] == '=' { tokens.push(Token::LtEq); i += 2; }
                else { tokens.push(Token::Lt); i += 1; }
            }
            '>' => {
                if i + 1 < chars.len() && chars[i+1] == '=' { tokens.push(Token::GtEq); i += 2; }
                else { tokens.push(Token::Gt); i += 1; }
            }

            '"' | '\'' => {
                let quote = chars[i]; i += 1;
                let start = i;
                while i < chars.len() && chars[i] != quote { i += 1; }
                let s: String = chars[start..i].iter().collect();
                tokens.push(Token::Str(s));
                i += 1;
            }

            c if c.is_ascii_digit() => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') { i += 1; }
                let num: String = chars[start..i].iter().collect();
                tokens.push(Token::Number(num.parse().unwrap()));
            }

            c if c.is_alphabetic() || c == '_' => {
                // f-string: f" or f'
                if c == 'f' && i + 1 < chars.len() && (chars[i+1] == '"' || chars[i+1] == '\'') {
                    let quote = chars[i+1];
                    i += 2;
                    let start = i;
                    while i < chars.len() && chars[i] != quote { i += 1; }
                    let raw: String = chars[start..i].iter().collect();
                    tokens.push(Token::FStringLit(raw));
                    i += 1;
                } else {
                    let start = i;
                    while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') { i += 1; }
                    let word: String = chars[start..i].iter().collect();

                    // p(...) shorthand: read raw unquoted text inside parens
                    if word == "p" && i < chars.len() && chars[i] == '(' {
                        i += 1; // skip '('
                        let raw_start = i;
                        while i < chars.len() && chars[i] != ')' { i += 1; }
                        let raw: String = chars[raw_start..i].iter().collect();
                        if i < chars.len() { i += 1; } // skip ')'
                        tokens.push(Token::Ident("p".to_string()));
                        tokens.push(Token::LParen);
                        tokens.push(Token::Str(raw.trim().to_string()));
                        tokens.push(Token::RParen);
                        continue;
                    }
                    let tok = match word.as_str() {
                        "let"    => Token::Let,
                        "fn"     => Token::Fn,
                        "return" => Token::Return,
                        "if"     => Token::If,
                        "elif"   => Token::Elif,
                        "else"   => Token::Else,
                        "for"    => Token::For,
                        "while"  => Token::While,
                        "in"     => Token::In,
                        "and"    => Token::And,
                        "or"     => Token::Or,
                        "not"    => Token::Not,
                        "true"   => Token::Bool(true),
                        "false"  => Token::Bool(false),
                        "none" | "null" | "nil" => Token::None,
                        _        => Token::Ident(word),
                    };
                    tokens.push(tok);
                }
            }

            _ => { i += 1; }
        }
    }
}
