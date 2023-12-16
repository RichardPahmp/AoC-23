use super::{opt, pair, Compare, Input, ParseError, Parser, Res};

impl<'a> Parser<&'a [u8]> for u8 {
    type Output = u8;

    fn parse(&mut self, input: &'a [u8]) -> Res<&'a [u8], Self::Output> {
        let (ch, rem) = input.split_item()?;
        if ch == *self {
            Ok((rem, ch))
        } else {
            Err(ParseError::TagNotFound)
        }
    }
}

impl<'a> Parser<&'a [u8]> for &'static [u8] {
    type Output = &'a [u8];

    fn parse(&mut self, input: &'a [u8]) -> Res<&'a [u8], Self::Output> {
        let len = self.len();
        match input.compare(self) {
            true => {
                let (prefix, suffix) = Input::split(&input, len);
                Ok((suffix, prefix))
            }
            false => Err(ParseError::TagNotFound),
        }
    }
}

impl<'a> Parser<&'a str> for &'static str {
    type Output = &'a str;

    fn parse(&mut self, input: &'a str) -> Res<&'a str, Self::Output> {
        let len = self.len();
        match input.compare(self) {
            true => {
                let (prefix, suffix) = Input::split(&input, len);
                Ok((suffix, prefix))
            }
            false => Err(ParseError::TagNotFound),
        }
    }
}

impl<'a> Parser<&'a str> for char {
    type Output = char;

    fn parse(&mut self, input: &'a str) -> Res<&'a str, Self::Output> {
        let (ch, rem) = input.split_item()?;
        if ch == *self {
            Ok((rem, ch))
        } else {
            Err(ParseError::TagNotFound)
        }
    }
}

pub fn chr() -> impl FnMut(&str) -> Res<&str, char> {
    move |input| {
        let mut chars = input.chars();
        match chars.next() {
            Some(c) => Ok((chars.as_str(), c)),
            None => Err(ParseError::NoValueFound),
        }
    }
}

pub fn nl<'a>() -> impl FnMut(&'a str) -> Res<&'a str, (Option<char>, char)> {
    pair(opt('\r'), '\n')
}

pub fn tag<I, T>(tag: T) -> impl Fn(I) -> Res<I, I>
where
    I: Input + Compare<T>,
    T: Input + Clone,
{
    move |input| {
        let tag = tag.clone();
        let len = tag.length();
        match input.compare(tag) {
            true => {
                let (prefix, suffix) = input.split(len);
                Ok((suffix, prefix))
            }
            false => Err(ParseError::TagNotFound),
        }
    }
}

pub fn take_while<P, I>(predicate: P) -> impl Fn(I) -> Res<I, I>
where
    I: Input,
    P: Fn(I::Item) -> bool,
{
    move |input: I| {
        let (prefix, suffix) = input.split_at_position_complete(|i| !predicate(i))?;
        Ok((suffix, prefix))
    }
}

pub fn take_while1<P, I>(predicate: P) -> impl Fn(I) -> Res<I, I>
where
    I: Input,
    P: Fn(I::Item) -> bool,
{
    move |input: I| {
        let (prefix, suffix) = input.split_at_position1_complete(|i| !predicate(i))?;
        Ok((suffix, prefix))
    }
}

pub fn map<I, P, M, O>(mut parser: P, mapper: M) -> impl FnMut(I) -> Res<I, O>
where
    I: Input,
    P: Parser<I>,
    M: Fn(P::Output) -> O,
{
    move |input: I| {
        let (remainder, output) = parser.parse(input)?;
        let mapped = mapper(output);
        Ok((remainder, mapped))
    }
}

pub fn map_res<I, P, M, O, E>(mut parser: P, mapper: M) -> impl FnMut(I) -> Res<I, O>
where
    I: Input,
    P: Parser<I>,
    M: Fn(P::Output) -> Result<O, E>,
{
    move |input: I| {
        let (remainder, output) = parser.parse(input)?;
        let mapped = mapper(output).map_err(|_| ParseError::MapError)?;
        Ok((remainder, mapped))
    }
}
