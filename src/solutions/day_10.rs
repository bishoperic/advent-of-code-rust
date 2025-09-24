use crate::shared::Solution;

pub struct Day10 {}

impl Solution for Day10 {
    fn part_a(&self, input: String) -> String {
        let mut syntax_error_score = 0;

        for line in input.lines() {
            let tokens = Tokenizer::tokenize(line.to_string());
            let validated = Validated::parse_tokens(tokens);

            for token in validated.0 {
                if token.valid {
                    continue;
                } else {
                    syntax_error_score += match token.token {
                        Token::Parenthesis(_) => 3,
                        Token::Bracket(_) => 57,
                        Token::Braces(_) => 1197,
                        Token::Angle(_) => 25137,
                    };
                    break;
                }
            }
        }

        syntax_error_score.to_string()
    }
    fn part_b(&self, input: String) -> String {
        let mut autocomplete_scores = Vec::new();

        for line in input.lines() {
            let tokens = Tokenizer::tokenize(line.to_string());
            let (validated, mut leftover_open_tokens) = Validated::parse_tokens(tokens);

            if Validated::validate_list(validated) {
                let mut line_score: i64 = 0;

                leftover_open_tokens.reverse();
                for token in leftover_open_tokens {
                    let token_score = match token {
                        Token::Parenthesis(_) => 1,
                        Token::Bracket(_) => 2,
                        Token::Braces(_) => 3,
                        Token::Angle(_) => 4,
                    };

                    line_score *= 5;
                    line_score += token_score;
                }
                autocomplete_scores.push(line_score);
            }
        }

        autocomplete_scores.sort();
        let middle_score = autocomplete_scores
            .get(autocomplete_scores.len() / 2)
            .unwrap();

        middle_score.to_string()
    }
}

// the boolean being true indicates an opening container
#[derive(Debug)]
enum Token {
    Parenthesis(bool),
    Bracket(bool),
    Braces(bool),
    Angle(bool),
}
impl Token {
    fn is_open(&self) -> bool {
        match self {
            Self::Parenthesis(open) => *open,
            Self::Bracket(open) => *open,
            Self::Braces(open) => *open,
            Self::Angle(open) => *open,
        }
    }
}

struct Tokenizer {}
impl Tokenizer {
    fn tokenize(input: String) -> Vec<Token> {
        input
            .chars()
            .map(|symbol| match symbol {
                '(' => Token::Parenthesis(true),
                ')' => Token::Parenthesis(false),
                '[' => Token::Bracket(true),
                ']' => Token::Bracket(false),
                '{' => Token::Braces(true),
                '}' => Token::Braces(false),
                '<' => Token::Angle(true),
                '>' => Token::Angle(false),
                _ => panic!("Unexpected token encountered!"),
            })
            .collect()
    }
}

#[derive(Debug)]
struct Validated {
    token: Token,
    valid: bool,
}
impl Validated {
    fn parse_tokens(input_tokens: Vec<Token>) -> (Vec<Validated>, Vec<Token>) {
        let mut validated = Vec::new();

        let mut current_stack = Vec::new();
        for token in input_tokens {
            let current_open_token = current_stack.last();

            if token.is_open() {
                current_stack.push(token);
            } else {
                match current_open_token {
                    // check to see if the closing token matches the current open token
                    Some(current_open_token) => {
                        if std::mem::discriminant(&token)
                            == std::mem::discriminant(current_open_token)
                        {
                            validated.push(Validated { token, valid: true });
                        } else {
                            validated.push(Validated {
                                token,
                                valid: false,
                            });
                        }
                    }
                    None => panic!("Closing token without any opening tokens!"),
                };

                current_stack.pop();
            }
        }

        (validated, current_stack)
    }
    fn validate_list(validations: Vec<Validated>) -> bool {
        validations.iter().all(|x| x.valid)
    }
}
