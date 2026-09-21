enum LexingState {
    Unquoted,
    SingleQuote,
    DoubleQuote,
    EscapedUnquoted,
    EscapedDoubleQuoted,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Word(String),
    RedirectOutput,
    Pipe,
}

pub fn lex(user_input: &str) -> Vec<Token> {
    let mut state = LexingState::Unquoted;
    let mut word_started = false;
    let mut current_word = String::new();
    let mut tokens: Vec<Token> = Vec::new();

    for char in user_input.chars() {
        match state {
            LexingState::Unquoted => match char {
                ' ' | '\t' => complete_word(&mut word_started, &mut current_word, &mut tokens),
                '|' => {
                    complete_word(&mut word_started, &mut current_word, &mut tokens);
                    tokens.push(Token::Pipe);
                }
                '>' => {
                    // `1>` is an explicit fd for stdout, same as a bare `>`.
                    if word_started && current_word == "1" {
                        current_word.clear();
                        word_started = false;
                    } else {
                        complete_word(&mut word_started, &mut current_word, &mut tokens);
                    }
                    tokens.push(Token::RedirectOutput);
                }
                '\'' => {
                    word_started = true;
                    state = LexingState::SingleQuote;
                }
                '"' => {
                    word_started = true;
                    state = LexingState::DoubleQuote;
                }
                '\\' => state = LexingState::EscapedUnquoted,
                _ => {
                    word_started = true;
                    current_word.push(char);
                }
            },
            LexingState::SingleQuote => match char {
                '\'' => state = LexingState::Unquoted,
                _ => current_word.push(char),
            },
            LexingState::DoubleQuote => match char {
                '"' => state = LexingState::Unquoted,
                '\\' => state = LexingState::EscapedDoubleQuoted,
                _ => current_word.push(char),
            },
            LexingState::EscapedUnquoted => {
                current_word.push(char);
                word_started = true;
                state = LexingState::Unquoted;
            }
            LexingState::EscapedDoubleQuoted => {
                // Inside double quotes, backslash only escapes these chars;
                // otherwise the backslash is kept literally.
                match char {
                    '\\' | '"' | '$' | '`' => current_word.push(char),
                    _ => {
                        current_word.push('\\');
                        current_word.push(char);
                    }
                }
                state = LexingState::DoubleQuote;
            }
        }
    }

    complete_word(&mut word_started, &mut current_word, &mut tokens);
    tokens
}

fn complete_word(word_started: &mut bool, current_word: &mut String, tokens: &mut Vec<Token>) {
    if *word_started {
        tokens.push(Token::Word(std::mem::take(current_word)));
        *word_started = false;
    }
}