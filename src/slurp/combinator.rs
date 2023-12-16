use super::{Input, Parser, Res};

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
            let Ok((rem, value)) = parser.parse(remainder.clone()) else {
                break remainder;
            };
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
