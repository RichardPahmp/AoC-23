mod combinator;
mod parser;
mod tuple;

pub use combinator::*;
pub use parser::*;
pub use tuple::*;

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    TagNotFound,
    NoValueFound,
    UnexpectedEnd,
    MapError,
    Unknown,
}

pub type Res<I, O> = Result<(I, O), ParseError>;

pub trait Parser<I> {
    type Output;

    fn parse(&mut self, input: I) -> Res<I, Self::Output>;
}

pub trait Input: Sized {
    type Item;

    fn take(&self, count: usize) -> Self;
    fn split(&self, count: usize) -> (Self, Self);
    fn split_item(&self) -> Res<Self::Item, Self>;
    fn split_at_position<P: Fn(Self::Item) -> bool>(&self, predicate: P) -> Res<Self, Self>;
    fn split_at_position1_complete<P: Fn(Self::Item) -> bool>(
        &self,
        predicate: P,
    ) -> Res<Self, Self>;
    fn split_at_position_complete<P: Fn(Self::Item) -> bool>(
        &self,
        predicate: P,
    ) -> Res<Self, Self>;
    fn length(&self) -> usize;
    fn empty(&self) -> bool {
        self.length() == 0
    }
}

impl<'a> Input for &'a str {
    type Item = char;

    fn split_item(&self) -> Res<Self::Item, Self> {
        let ch = self.chars().next().ok_or(ParseError::UnexpectedEnd)?;
        Ok((ch, &self[ch.len_utf8()..]))
    }

    fn take(&self, count: usize) -> Self {
        &self[..count]
    }

    fn split(&self, count: usize) -> (Self, Self) {
        self.split_at(count)
    }

    // Splitting at 0 or input.len() is an error
    fn split_at_position<P: Fn(Self::Item) -> bool>(&self, predicate: P) -> Res<Self, Self> {
        match self.find(predicate) {
            Some(0) => Err(ParseError::NoValueFound),
            Some(i) => Ok(self.split(i)),
            None => Err(ParseError::UnexpectedEnd),
        }
    }

    // Consumes 1 or more input
    fn split_at_position1_complete<P: Fn(Self::Item) -> bool>(
        &self,
        predicate: P,
    ) -> Res<Self, Self> {
        match self.find(predicate) {
            Some(0) => Err(ParseError::NoValueFound),
            Some(i) => Ok(self.split(i)),
            None => {
                if self.is_empty() {
                    Err(ParseError::UnexpectedEnd)
                } else {
                    Ok((self, &""))
                }
            }
        }
    }

    /// Might consume all or no input
    fn split_at_position_complete<P: Fn(Self::Item) -> bool>(
        &self,
        predicate: P,
    ) -> Res<Self, Self> {
        match self.find(predicate) {
            Some(i) => Ok(self.split(i)),
            // None => Ok((&self[self.len()..], &self[..self.len()]))
            None => Ok((self, &"")),
        }
    }

    fn length(&self) -> usize {
        str::len(self)
    }
}

impl<'a> Input for &'a [u8] {
    type Item = u8;

    fn take(&self, count: usize) -> Self {
        &self[..count]
    }

    fn split(&self, count: usize) -> (Self, Self) {
        self.split_at(count)
    }

    fn length(&self) -> usize {
        <[u8]>::len(self)
    }

    fn split_item(&self) -> Res<Self::Item, Self> {
        let (item, rem) = self.split_first().ok_or(ParseError::UnexpectedEnd)?;
        Ok((*item, rem))
    }

    fn split_at_position<P: Fn(Self::Item) -> bool>(&self, predicate: P) -> Res<Self, Self> {
        match self
            .iter()
            .enumerate()
            .find_map(|(i, b)| predicate(*b).then_some(i))
        {
            Some(0) => Err(ParseError::NoValueFound),
            Some(i) => Ok(self.split(i)),
            None => Err(ParseError::UnexpectedEnd),
        }
    }

    fn split_at_position1_complete<P: Fn(Self::Item) -> bool>(
        &self,
        predicate: P,
    ) -> Res<Self, Self> {
        match self
            .iter()
            .enumerate()
            .find_map(|(i, b)| predicate(*b).then_some(i))
        {
            Some(0) => Err(ParseError::NoValueFound),
            Some(i) => Ok(self.split(i)),
            None => {
                if self.is_empty() {
                    Err(ParseError::UnexpectedEnd)
                } else {
                    Ok((self, &[]))
                }
            }
        }
    }

    fn split_at_position_complete<P: Fn(Self::Item) -> bool>(
        &self,
        predicate: P,
    ) -> Res<Self, Self> {
        match self
            .iter()
            .enumerate()
            .find_map(|(i, b)| predicate(*b).then_some(i))
        {
            Some(i) => Ok(self.split(i)),
            None => Ok((self, &[])),
        }
    }
}

pub trait Compare<T> {
    /// Check if other starts with self
    fn compare(&self, other: T) -> bool;
}

impl Compare<&str> for &str {
    fn compare(&self, other: &str) -> bool {
        self.starts_with(other)
    }
}

impl Compare<&[u8]> for &[u8] {
    fn compare(&self, other: &[u8]) -> bool {
        self.starts_with(other)
    }
}

pub trait ParseTo<T> {
    fn parse_to(&self) -> Option<T>;
}

impl<'a, T: FromStr> ParseTo<T> for &'a str {
    fn parse_to(&self) -> Option<T> {
        self.parse().ok()
    }
}

impl<I, O, F> Parser<I> for F
where
    F: FnMut(I) -> Res<I, O>,
{
    type Output = O;

    fn parse(&mut self, input: I) -> Res<I, O> {
        self(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag() {
        assert_eq!(Ok(("hejsan", "tjo")), tag("tjo")("tjohejsan"));
        assert_eq!(Ok(("josan", 't')), 't'.parse("tjosan"));
        assert_eq!(
            Ok((&b"hejsan"[..], &b"tjo"[..])),
            tag(&b"tjo"[..])(&b"tjohejsan"[..])
        );
        assert_eq!(Ok((&b"josan"[..], b't')), b't'.parse(&b"tjosan"[..]));
    }

    #[test]
    fn test_take_while() {
        assert_eq!(
            Ok(("123", "tjena")),
            take_while(char::is_alphabetic)("tjena123")
        )
    }

    #[test]
    fn test_pair() {
        assert_eq!(
            Ok(("va", ("tjo", "234"))),
            pair(tag("tjo"), take_while(char::is_numeric))("tjo234va")
        );
    }

    #[test]
    fn test_map() {
        let mut num = map_res(take_while1(char::is_numeric), str::parse::<u32>);
        assert_eq!(Ok(("tjena", 345)), num("345tjena"));
    }

    #[test]
    fn test_separated_list() {
        let num = map_res(take_while(char::is_numeric), str::parse::<u32>);
        assert_eq!(
            Ok(("", vec![1, 22, 333, 4444, 55555])),
            separated_list(num, tag(", "))("1, 22, 333, 4444, 55555")
        )
    }

    #[test]
    fn test_separated_list_whitespace() {
        let num = map_res(take_while1(char::is_numeric), str::parse::<u32>);
        assert_eq!(
            Ok(("", vec![1, 22, 333, 4444, 55555])),
            separated_list(num, take_while1(char::is_whitespace))("1  22  333  4444  55555")
        )
    }
}
