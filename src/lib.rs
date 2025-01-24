#[derive(Debug, Clone, PartialEq)]
enum Token {
    Add,
    Sub,
    Mul,
    Div,
    Open,
    Close,
    Ident(char),
}

impl TryFrom<char> for Token {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '+' => Ok(Token::Add),
            '-' => Ok(Token::Sub),
            '*' => Ok(Token::Mul),
            '/' => Ok(Token::Div),
            '(' => Ok(Token::Open),
            ')' => Ok(Token::Close),
            c if c.is_ascii_alphabetic() => Ok(Token::Ident(c)),
            c => Err(format!("invalid char is found: {}", c)),
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
            Token::Open => Err("close paren is not found".into()),
            Token::Close => Err("invalid token is found".into()),
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

        match Token::try_from(c)? {
            Token::Open => stack.push(Token::Open),
            Token::Close => loop {
                match stack.pop() {
                    Some(Token::Open) => break,
                    Some(token) => tokens.push(token.try_into()?),
                    None => return Err("open paren is not found".into()),
                }
            },
            Token::Ident(c) => tokens.push(c),
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

    for token in stack.iter().rev() {
        tokens.push(token.clone().try_into()?);
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
            Err("close paren is not found".into())
        );

        assert_eq!(
            convert("a + b)".into()),
            Err("open paren is not found".into())
        );
    }
}
