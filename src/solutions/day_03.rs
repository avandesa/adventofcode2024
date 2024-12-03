use crate::solver::Solver;

#[derive(Clone, Copy, Debug)]
enum Command {
    Mul(u32, u32),
    Do,
    Dont,
}

#[derive(Clone, Copy, Debug, strum::EnumDiscriminants)]
#[strum_discriminants(name(TokenKind))]
enum Token {
    Mul,
    Do,
    Dont,
    LeftParen,
    Comma,
    RightParen,
    Number(u32),
    Other,
}

impl Token {
    fn unwrap_number(self) -> u32 {
        if let Self::Number(n) = self {
            n
        } else {
            panic!("`{self:?}` is not a number")
        }
    }
}

struct Scanner {
    input: Vec<char>,
    current_position: usize,
}

impl Scanner {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            current_position: 0,
        }
    }

    fn scan(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(t) = self.next_token() {
            tokens.push(t);
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        let t = match self.pop()? {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            ',' => Token::Comma,
            d if d.is_ascii_digit() => {
                let num = self
                    .pop_while(char::is_ascii_digit)
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();
                Token::Number(num)
            }
            'm' => {
                if !self.pop_if_matches('u') {
                    return Some(Token::Other);
                }
                if !self.pop_if_matches('l') {
                    return Some(Token::Other);
                }
                Token::Mul
            }
            'd' => {
                if !self.pop_if_matches('o') {
                    return Some(Token::Other);
                }
                if self.pop_if_matches('n') {
                    if !self.pop_if_matches('\'') {
                        return Some(Token::Other);
                    }
                    if !self.pop_if_matches('t') {
                        return Some(Token::Other);
                    }
                    Token::Dont
                } else {
                    Token::Do
                }
            }
            _ => Token::Other,
        };

        Some(t)
    }

    fn pop(&mut self) -> Option<char> {
        let c = self.input.get(self.current_position)?;
        self.current_position += 1;
        Some(*c)
    }

    fn pop_while<F: FnMut(&char) -> bool>(&mut self, mut f: F) -> &[char] {
        let start = self.current_position - 1;
        while let Some(c) = self.input.get(self.current_position) {
            if !f(c) {
                break;
            }
            self.current_position += 1;
        }
        &self.input[start..self.current_position]
    }

    fn pop_if_matches(&mut self, c: char) -> bool {
        if self.input.get(self.current_position) == Some(&c) {
            self.current_position += 1;
            true
        } else {
            false
        }
    }
}

enum ParseResult {
    Command(Command),
    Discard,
    Eof,
}

struct Parser {
    tokens: Vec<Token>,
    current_position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_position: 0,
        }
    }

    fn parse(mut self) -> Vec<Command> {
        let mut commands = Vec::new();
        loop {
            match self.next_command() {
                ParseResult::Command(c) => commands.push(c),
                ParseResult::Discard => continue,
                ParseResult::Eof => break,
            }
        }
        commands
    }

    fn next_command(&mut self) -> ParseResult {
        let Some(t) = self.pop() else {
            return ParseResult::Eof;
        };
        match t {
            Token::Mul => {
                if self.pop_if_matches(TokenKind::LeftParen).is_none() {
                    return ParseResult::Discard;
                };
                let Some(a) = self
                    .pop_if_matches(TokenKind::Number)
                    .map(Token::unwrap_number)
                else {
                    return ParseResult::Discard;
                };
                if self.pop_if_matches(TokenKind::Comma).is_none() {
                    return ParseResult::Discard;
                };
                let Some(b) = self
                    .pop_if_matches(TokenKind::Number)
                    .map(Token::unwrap_number)
                else {
                    return ParseResult::Discard;
                };
                if self.pop_if_matches(TokenKind::RightParen).is_none() {
                    return ParseResult::Discard;
                };

                ParseResult::Command(Command::Mul(a, b))
            }
            Token::Do | Token::Dont => {
                if self.pop_if_matches(TokenKind::LeftParen).is_none() {
                    return ParseResult::Discard;
                };
                if self.pop_if_matches(TokenKind::RightParen).is_none() {
                    return ParseResult::Discard;
                };

                ParseResult::Command(match t {
                    Token::Do => Command::Do,
                    Token::Dont => Command::Dont,
                    _ => unreachable!(),
                })
            }
            _ => ParseResult::Discard,
        }
    }

    fn pop(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.current_position)?;
        self.current_position += 1;
        Some(*t)
    }

    fn pop_if_matches(&mut self, to_match: TokenKind) -> Option<Token> {
        let t = self.tokens.get(self.current_position)?;
        if <&Token as Into<TokenKind>>::into(t) == to_match {
            self.current_position += 1;
            Some(*t)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum State {
    Do,
    Dont,
}

struct Executor {
    state: State,
    sum: u32,
}

impl Default for Executor {
    fn default() -> Self {
        Self {
            state: State::Do,
            sum: 0,
        }
    }
}

impl Executor {
    fn execute(&mut self, cmd: Command) {
        match cmd {
            Command::Mul(a, b) => {
                if self.state == State::Do {
                    self.sum += a * b;
                }
            }
            Command::Do => self.state = State::Do,
            Command::Dont => self.state = State::Dont,
        }
    }
}

pub struct Solver03(Vec<Command>);

impl Solver for Solver03 {
    fn new(input: &str) -> Self
    where
        Self: Sized,
    {
        let tokens = Scanner::new(input).scan();
        let commands = Parser::new(tokens).parse();

        Self(commands)
    }

    fn part_01(&self) -> String {
        self.0
            .iter()
            .filter_map(|c| {
                if let Command::Mul(a, b) = c {
                    Some((a, b))
                } else {
                    None
                }
            })
            .map(|(a, b)| *a * *b)
            .sum::<u32>()
            .to_string()
    }

    fn part_02(&self) -> String {
        let mut e = Executor::default();
        for c in &self.0 {
            e.execute(*c);
        }
        e.sum.to_string()
    }
}
