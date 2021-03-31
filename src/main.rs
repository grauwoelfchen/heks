extern crate nom;

use std::num::ParseIntError;

use nom::IResult;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::combinator::map_res;
use nom::sequence::tuple;

use rustyline::Editor;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

fn from_hex_to_u8(s: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(s, 16)
}

fn is_hex(c: char) -> bool {
    c.is_digit(16)
}

fn hex(s: &str) -> IResult<&str, u8> {
    let mut f = map_res(take_while_m_n(2, 2, is_hex), from_hex_to_u8);
    f(s)
}

#[allow(clippy::many_single_char_names)]
fn parse_hex_color(s: &str) -> IResult<&str, Color> {
    let (i, _) = tag("#")(s)?;
    let (i, (r, g, b)) = tuple((hex, hex, hex))(i)?;

    let c = Color { r, g, b };
    Ok((i, c))
}

fn with_tag(s: &str) -> String {
    if !s.starts_with('#') {
        return format!("#{}", s);
    }
    s.to_string()
}

fn main() {
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let hex = with_tag(&line);
                if let Ok((_, c)) = parse_hex_color(&hex) {
                    println!("r: {} g: {} b: {}", c.r, c.g, c.b);
                } else {
                    println!("invalid value is given");
                }
            }
            _ => break,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_hex_to_u8() {
        let result = from_hex_to_u8("22").unwrap();
        assert_eq!(result, 34);
    }

    #[test]
    fn test_is_hex() {
        assert!(is_hex('0'));
        assert!(is_hex('9'));
        assert!(is_hex('a'));
        assert!(is_hex('f'));

        assert!(!is_hex('g'));
    }

    #[test]
    fn test_parse_hex_color() {
        let color = Color {
            r: 34,
            g: 34,
            b: 34,
        };

        let (_, result) = parse_hex_color("#222222").unwrap();
        assert_eq!(result, color);
    }

    #[test]
    fn test_with_tag() {
        assert_eq!("#222222", with_tag("#222222"));
        assert_eq!("#222222", with_tag("222222"));
    }
}
