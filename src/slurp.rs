use std::str::FromStr;

// type Res<'a, T> = Result<(T, &'a str), ()>;
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
    fn split_at_position<P: Fn(Self::Item) -> bool>(&self, predicate: P) -> Res<Self, Self>;
    fn split_at_position1_complete<P: Fn(Self::Item) -> bool>(
        &self,
        predicate: P,
    ) -> Res<Self, Self>;
    fn split_at_position_complete<P: Fn(Self::Item) -> bool>(
        &self,
        predicate: P,
    ) -> Res<Self, Self>;
    fn len(&self) -> usize;
}

impl<'a> Input for &'a str {
    type Item = char;

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

    fn len(&self) -> usize {
        str::len(self)
    }
}

// impl<'a> Input for &'a [u8] {
//     type Item = u8;

//     fn take(&self, count: usize) -> Self {
//         &self[..count]
//     }

//     fn split(&self, count: usize) -> (Self, Self) {
//         self.split_at(count)
//     }

//     fn len(&self) -> usize {
//         <[u8]>::len(self)
//     }
// }

pub trait Compare<T> {
    /// Check if other starts with self
    fn compare(&self, other: T) -> bool;
}

impl Compare<&str> for &str {
    fn compare(&self, other: &str) -> bool {
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

pub fn tag<I, T>(tag: T) -> impl Fn(I) -> Res<I, I>
where
    I: Input + Compare<T>,
    T: Input + Clone,
{
    move |input| {
        let tag = tag.clone();
        let len = tag.len();
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

pub fn separated_list<P, S, I>(
    mut parser: P,
    mut separator: S,
) -> impl FnMut(I) -> Res<I, Vec<P::Output>>
where
    I: Input + Clone,
    P: Parser<I>,
    S: Parser<I>,
{
    move |input: I| {
        let mut values = Vec::new();
        let mut remainder = input;

        let r = loop {
            let (rem, value) = parser.parse(remainder)?;
            values.push(value);
            match separator.parse(rem.clone()) {
                Ok((rem, _)) => remainder = rem,
                Err(_) => break rem,
            }
        };
        Ok((r, values))
    }
}

pub fn pair<I, A, B>(mut left: A, mut right: B) -> impl FnMut(I) -> Res<I, (A::Output, B::Output)>
where
    A: Parser<I>,
    B: Parser<I>,
{
    move |input: I| {
        let (remainder, value_left) = left.parse(input)?;
        let (remainder, value_right) = right.parse(remainder)?;
        Ok((remainder, (value_left, value_right)))
    }
}
pub fn separated_pair<I, A, B, S>(
    mut left: A,
    mut separator: S,
    mut right: B,
) -> impl FnMut(I) -> Res<I, (A::Output, B::Output)>
where
    A: Parser<I>,
    B: Parser<I>,
    S: Parser<I>,
{
    move |input: I| {
        let (remainder, value_left) = left.parse(input)?;
        let (remainder, _) = separator.parse(remainder)?;
        let (remainder, value_right) = right.parse(remainder)?;
        Ok((remainder, (value_left, value_right)))
    }
}

pub trait Tuple<I, O> {
    fn parse(&mut self, input: I) -> Res<I, O>;
}

impl<I, A, B, C> Tuple<I, (A::Output, B::Output, C::Output)> for (A, B, C)
where
    A: Parser<I>,
    B: Parser<I>,
    C: Parser<I>,
{
    fn parse(&mut self, input: I) -> Res<I, (A::Output, B::Output, C::Output)> {
        let (a, b, c) = self;
        let (rem, ra) = a.parse(input)?;
        let (rem, rb) = b.parse(rem)?;
        let (rem, rc) = c.parse(rem)?;
        Ok((rem, (ra, rb, rc)))
    }
}

impl<I, A, B, C, D> Tuple<I, (A::Output, B::Output, C::Output, D::Output)> for (A, B, C, D)
where
    A: Parser<I>,
    B: Parser<I>,
    C: Parser<I>,
    D: Parser<I>,
{
    fn parse(&mut self, input: I) -> Res<I, (A::Output, B::Output, C::Output, D::Output)> {
        let (a, b, c, d) = self;
        let (rem, ra) = a.parse(input)?;
        let (rem, rb) = b.parse(rem)?;
        let (rem, rc) = c.parse(rem)?;
        let (rem, rd) = d.parse(rem)?;
        Ok((rem, (ra, rb, rc, rd)))
    }
}

impl<I, A, B, C, D, E> Tuple<I, (A::Output, B::Output, C::Output, D::Output, E::Output)>
    for (A, B, C, D, E)
where
    A: Parser<I>,
    B: Parser<I>,
    C: Parser<I>,
    D: Parser<I>,
    E: Parser<I>,
{
    fn parse(
        &mut self,
        input: I,
    ) -> Res<I, (A::Output, B::Output, C::Output, D::Output, E::Output)> {
        let (a, b, c, d, e) = self;
        let (rem, ra) = a.parse(input)?;
        let (rem, rb) = b.parse(rem)?;
        let (rem, rc) = c.parse(rem)?;
        let (rem, rd) = d.parse(rem)?;
        let (rem, re) = e.parse(rem)?;
        Ok((rem, (ra, rb, rc, rd, re)))
    }
}

pub fn tuple<I, O>(mut parsers: impl Tuple<I, O>) -> impl FnMut(I) -> Res<I, O> {
    move |input: I| parsers.parse(input)
}

pub fn delimited<I, D, P>(mut delimiter: D, mut parser: P) -> impl FnMut(I) -> Res<I, P::Output>
where
    D: Parser<I>,
    P: Parser<I>,
{
    move |input: I| {
        let (rem, _) = delimiter.parse(input)?;
        let (rem, value) = parser.parse(rem)?;
        let (rem, _) = delimiter.parse(rem)?;
        Ok((rem, value))
    }
}

pub fn opt<I, P>(mut parser: P) -> impl FnMut(I) -> Res<I, Option<P::Output>>
where
    I: Clone,
    P: Parser<I>,
{
    move |input: I| {
        let i = input.clone();
        match parser.parse(input) {
            Ok((rem, value)) => Ok((rem, Some(value))),
            Err(_) => Ok((i, None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag() {
        assert_eq!(Ok(("hejsan", "tjo")), tag("tjo")("tjohejsan"));
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
