use super::{Parser, Res};

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
