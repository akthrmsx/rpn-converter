#[derive(Debug, Clone, PartialEq)]
enum Token {
    Add,
    Sub,
    Mul,
    Div,
    Open,
    Close,
    Ident(char),
    Invalid(char),
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '+' => Self::Add,
            '-' => Self::Sub,
            '*' => Self::Mul,
            '/' => Self::Div,
            '(' => Self::Open,
            ')' => Self::Close,
            c if c.is_ascii_alphabetic() => Self::Ident(c),
            c => Self::Invalid(c),
        }
    }
}

impl TryFrom<Token> for char {
    type Error = String;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Add => Ok('+'),
            Token::Sub => Ok('-'),
            Token::Mul => Ok('*'),
            Token::Div => Ok('/'),
            Token::Ident(c) => Ok(c),
            _ => Err("invalid token is found".into()),
        }
    }
}

impl Token {
    fn precedence(&self) -> usize {
        match self {
            Self::Mul | Self::Div => 2,
            Self::Add | Self::Sub => 1,
            _ => 0,
        }
    }
}

pub fn convert(source: String) -> Result<String, String> {
    let mut tokens = vec![];
    let mut stack = vec![];

    for c in source.chars() {
        if c.is_ascii_whitespace() {
            continue;
        }

        match Token::from(c) {
            Token::Open => stack.push(Token::Open),
            Token::Close => loop {
                match stack.pop() {
                    Some(Token::Open) => break,
                    Some(token) => tokens.push(token.try_into()?),
                    None => return Err("open paren is not found".into()),
                }
            },
            Token::Ident(c) => tokens.push(c),
            Token::Invalid(c) => return Err(format!("invalid char is found: {}", c)),
            current => {
                while let Some(top) = stack.pop() {
                    if current.precedence() <= top.precedence() {
                        tokens.push(top.try_into()?);
                    } else {
                        stack.push(top);
                        break;
                    }
                }
                stack.push(current);
            }
        }
    }

    if !stack.is_empty() {
        for token in stack.iter().rev() {
            tokens.push(token.clone().try_into()?);
        }
    }

    Ok(tokens
        .iter()
        .map(char::to_string)
        .collect::<Vec<String>>()
        .join(" "))
}

#[cfg(test)]
mod tests {
    use crate::convert;

    #[test]
    fn test_convert_success() {
        assert_eq!(
            convert("(a + b) * (c - d)".into()),
            Ok("a b + c d - *".into())
        );

        assert_eq!(
            convert("(a / (b * (c + d)))".into()),
            Ok("a b c d + * /".into())
        );
    }

    #[test]
    fn test_convert_failure() {
        assert_eq!(
            convert("a + 1".into()),
            Err("invalid char is found: 1".into())
        );

        assert_eq!(
            convert("(a + b".into()),
            Err("invalid token is found".into())
        );

        assert_eq!(
            convert("a + b)".into()),
            Err("open paren is not found".into())
        );
    }
}
