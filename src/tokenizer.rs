#[derive(Debug)]
enum TokenKind {
    KeyLet,
    
    SymbolEqual,
    SymbolSemicolon,
    SymbolColon,
    SymbolComma,
    SymbolPeriod,
    SymbolParenthesesOpen,
    SymbolParenthesesClose,
    SymbolSquareBracketOpen,
    SymbolSquareBracketClose,
    SymbolCurlyBracketOpen,
    SymbolCurlyBracketClose,
    
    Name,
    Number,
    Comment,
}


#[derive(Debug)]
struct Token {
    start: usize,
    end: usize,
    kind: TokenKind,
}

impl Token {
    pub fn new(start: usize, end: usize, kind: TokenKind) -> Self {
        Token { start, end, kind }
    }
}

#[derive(Debug)]
pub struct Tokens {
    tokens: Vec<Token>
}

#[derive(Debug)]
pub struct TokenizeError {
    errors: Vec<TokenizeErrorItem>
}

#[derive(Debug)]
struct TokenizeErrorItem {
    start: usize,
    end: usize,
    kind: TokenizeErrorKind,
}

#[derive(Debug)]
enum TokenizeErrorKind {
    UnknownToken(char)
}

impl TryFrom<&str> for Tokens {
    type Error = TokenizeError;
    fn try_from(value: &str) -> Result<Self, TokenizeError> {
        let chars: Vec<char> = value.chars().collect();
        let mut errors = Vec::new();
        let mut state = TokenizerState::Idle;
        let mut tokens = Vec::new();
        let mut iter = chars.into_iter().enumerate();
        
        let mut char = iter.next();
        
        let mut start = 0;
        
        while !char.is_none() {
            let (i, c) = char.unwrap();

            match state {
                TokenizerState::Idle => {
                    start = i;
                    match c {
                        '0'..='9' => state = TokenizerState::Number,
                        '_' | 'A'..='Z' | 'a'..='z' => state = TokenizerState::Name,
                        '/' => state = TokenizerState::CommentStart,
                        '{' => tokens.push(Token::new(start, i, TokenKind::SymbolCurlyBracketOpen)),
                        '}' => tokens.push(Token::new(start, i, TokenKind::SymbolCurlyBracketClose)),
                        '(' => tokens.push(Token::new(start, i, TokenKind::SymbolParenthesesOpen)),
                        ')' => tokens.push(Token::new(start, i, TokenKind::SymbolParenthesesClose)),
                        '[' => tokens.push(Token::new(start, i, TokenKind::SymbolSquareBracketOpen)),
                        ']' => tokens.push(Token::new(start, i, TokenKind::SymbolSquareBracketClose)),
                        '=' => tokens.push(Token::new(start, i, TokenKind::SymbolEqual)),
                        ',' => tokens.push(Token::new(start, i, TokenKind::SymbolComma)),
                        ';' => tokens.push(Token::new(start, i, TokenKind::SymbolSemicolon)),
                        ':' => tokens.push(Token::new(start, i, TokenKind::SymbolColon)),
                        '.' => tokens.push(Token::new(start, i, TokenKind::SymbolPeriod)),
                        '\n' | ' ' | '\t' | '\r' => (),
                        _ => errors.push(TokenizeErrorItem {start, end: i, kind: TokenizeErrorKind::UnknownToken(c)}), 
                    }
                    char = iter.next();
                }
                TokenizerState::CommentStart => {
                    match c {
                        '/' => {
                            state = TokenizerState::LineComment;
                            char = iter.next();
                        }
                        '*' => {
                            state = TokenizerState::BlockComment;
                            char = iter.next();
                        }
                        _ => {
                            state = TokenizerState::Idle;
                            errors.push(TokenizeErrorItem {start, end: i, kind: TokenizeErrorKind::UnknownToken('/')})
                        }
                    }
                }
                TokenizerState::LineComment => {
                    if c == '\n' {
                        state = TokenizerState::Idle;
                        tokens.push(Token::new(start, i-1, TokenKind::Comment));
                    }
                    else {
                        char = iter.next();
                    }
                }
                TokenizerState::BlockComment => {
                    if c == '/' && &value[i-1..i] == "*" {
                        state = TokenizerState::Idle;
                        tokens.push(Token::new(start, i-1, TokenKind::Comment));
                    }
                    else {
                        char = iter.next();
                    }
                }
                TokenizerState::Number => {
                    if !c.is_numeric() {
                        state = TokenizerState::Idle;
                        tokens.push(Token::new(start, i-1, TokenKind::Number));
                    }
                    else {
                        char = iter.next();
                    }
                }
                TokenizerState::Name => {
                    if !(c.is_alphanumeric() || c == '_') {
                        state = TokenizerState::Idle;
                        match &value[start..i] {
                            "let" => tokens.push(Token::new(start, i-1, TokenKind::KeyLet)),
                            _ => tokens.push(Token::new(start, i-1, TokenKind::Name)),
                        }
                    }
                    else {
                        char = iter.next();
                    }
                }
            }
        }
        if errors.len() == 0 {
            Ok (Tokens { tokens })
        }
        else {
            Err(TokenizeError { errors })
        }
        
    }
}

#[derive(Debug)]
enum TokenizerState {
    Idle,
    Name,
    Number,
    LineComment,
    BlockComment,
    CommentStart,
}