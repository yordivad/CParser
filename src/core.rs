type ParseOutput<'a, Output> = Result<(&'a str, Output), &'a str>;

pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseOutput<'a, Output>;
}

impl<'a, F, Output> Parser<'a, Output> for F
    where F: Fn(&'a str) -> ParseOutput<Output>
{
    fn parse(&self, input: &'a str) -> ParseOutput<'a, Output> {
        self(input)
    }
}

pub fn map<'a, P, F, A, B>(parser: P, mapper: F) -> impl Parser<'a, B>
    where
        P: Parser<'a, A>,
        F: Fn(A) -> B,
{
    move |input| {
        parser.parse(input)
            .map(|(next, result)| (next, mapper(result)))
    }
}


pub fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
    where
        P1: Parser<'a, R1>,
        P2: Parser<'a, R2>,
{
    map(bind(parser1, parser2), |(left, _right)| left)
}

pub fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
    where
        P1: Parser<'a, R1>,
        P2: Parser<'a, R2>,
{
    map(bind(parser1, parser2), |(_left, right)| right)
}

pub fn zero_or_more<'a, P, R>(parser: P) -> impl Parser<'a, Vec<R>>
    where
        P: Parser<'a, R>,
{
    move |mut input| {
        let mut result: Vec<R> = Vec::new();

        while let Ok((next_input, next_response)) = parser.parse(input) {
            input = next_input;
            result.push(next_response)
        }

        Ok((input, result))
    }
}

pub fn one_or_more<'a, P, R>(parser: P) -> impl Parser<'a, Vec<R>>
    where
        P: Parser<'a, R>,
{
    move |mut input| {
        let mut results: Vec<R> = Vec::new();

        if let Ok((next, result)) = parser.parse(input) {
            input = next;
            results.push(result);
        } else {
            return Err(input);
        }

        while let Ok((next, result)) = parser.parse(input) {
            input = next;
            results.push(result)
        }

        Ok((input, results))
    }
}

pub fn predicate<'a, P, R, F>(parser: P, pred: F) -> impl Parser<'a, R>
    where
        P: Parser<'a, R>,
        F: Fn(&R) -> bool,
{
    move |input| {
        if let Ok((next, result)) = parser.parse(input) {
            if pred(&result) {
                return Ok((next, result));
            }
        }
        Err(input)
    }
}

pub fn either<'a, P1, P2, A>(parser1: P1, parser2: P2) -> impl Parser<'a, A>
where
 P1: Parser<'a, A>,
 P2: Parser<'a, A>,
{
    move |input| {
        match parser1.parse(input) {
            ok @ Ok(_) => ok,
            Err(_) => parser2.parse(input),
        }
    }
}

pub fn and_then<'a, P1, F, A, B, P2>(parser: P1, f: F) -> impl Parser<'a, B>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, B>,
    F: Fn(A) -> P2,
{
    move |input| {
        match parser.parse(input) {
            Ok((next, result)) => f(result).parse(next),
            Err(err) => Err(err)
        }
    }
}


pub fn bind<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
    where
        P1: Parser<'a, R1>,
        P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next, result1)| {
            parser2.parse(next).map(|(last, result2)| { (last, (result1, result2)) })
        })
    }
}