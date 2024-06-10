use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, digit1};
use nom::combinator::{recognize, value};
use nom::multi::many0_count;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;

#[derive(Clone)]
enum Expression {
    True,
    False,
    Num(i32),
    Var(Variable),
    Nil,
    Let(Definition, Expression),
    Not(Expression),
    If(Expression, Expression, Expression),
    Succ(Expression),
    Pred(Expression),
    Fst(Expression),
    Snd(Expression),
    Hd(Expression),
    Tl(Expression),
    Pair(Expression, Expression),
    Fn(Variable, Expression),
    Eq(Expression, Expression),
    Cons(Expression, Expression),
    And(Expression, Expression),
    Add(Expression, Expression),
    Apply(Expression, Expression)
}

#[derive(Clone)]
struct Variable {
    ident: String
}


#[derive(Clone)]
struct Definition {
    name: Variable,
    value: Expression
}

fn parser(input: &str) -> IResult<&str, Expression> {
    parse_e_top(input)
}

fn parse_variable(input: &str) -> IResult<&str, Variable> {
    // x = [a-zA-Z_][a-zA-Z0-9]*
    let (remainder, s) = recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))
    .parse(input)?;
    let v = Variable { ident: s.to_string() };
    Ok((remainder, v))
}

fn parse_e_variable(input: &str) -> IResult<&str, Expression> {
    let (remainder, v) = parse_variable(input)?;
    let e = Expression::Var(v);
    Ok((remainder, e))
}

fn parse_bool(input: &str) -> IResult<&str, Expression> {
    alt((value(Expression::True, tag("true")), value(Expression::False, tag("false"))))(input)
}

fn parse_num(input: &str) -> IResult<&str, Expression> {
    digit1(input)
}

fn parse_nil(input: &str) -> IResult<&str, Expression> {
    value(Expression::Nil, tag("nil"))(input)
}

fn parse_let(input: &str) -> IResult<&str, Expression> {
    let (remainder, _) = tag("let")(input)?;
    let (remainder, def) = parse_def(remainder)?;
    let (remainder, _) = tag("in")(remainder)?;
    let (remainder, e) = parse_e_top(remainder)?;
    let l = Expression::Let(def, e);
    Ok((remainder, l))
}

fn parse_def(input: &str) -> IResult<&str, Definition> {
    let (remainder, var) = parse_variable(input)?;
    let (remainder, _) = tag("=")(remainder)?;
    let (remainder, e) = parse_e_top(remainder)?;
    let def = Definition { name: var, value: e};
    Ok((remainder, def))
}

fn parse_not(input: &str) -> IResult<&str, Expression> {
    let (remainder, _) = tag("not")(input)?;
    let (remainder, _) = tag("(")(remainder)?;
    let (remainder, e) = parse_e_top(remainder)?;
    let (remainder, _) = tag(")")(remainder)?;
    let n = Expression::Not(e);
    Ok((remainder, n))
}

fn parse_if(input: &str) -> IResult<&str, Expression> {
    let (remainder, _) = tag("if")(input)?;
    let (remainder, cond) = parse_e_top(remainder)?;
    let (remainder, _) = tag("then")(remainder)?;
    let (remainder, e_true) = parse_e_top(remainder)?;
    let (remainder, _) = tag("else")(remainder)?;
    let (remainder, e_false) = parse_e_top(remainder)?;
    let i = Expression::If(cond, e_true, e_false);
    Ok((remainder, i))
}

fn parse_succ(input: &str) -> IResult<&str, Expression> {
    let (remainder, _) = tag("succ")(input)?;
    let (remainder, _) = tag("(")(remainder)?;
    let (remainder, e) = parse_e_top(remainder)?;
    let (remainder, _) = tag(")")(remainder)?;
    let s = Expression::Succ(e);
    Ok((remainder, s))
}

fn parse_pair(input: &str) -> IResult<&str, Expression> {
    let (remainder, _) = tag("<")(input)?;
    let (remainder, e1) = parse_e_top(remainder)?;
    let (remainder, _) = tag(",")(remainder)?;
    let (remainder, e2) = parse_e_top(remainder)?;
    let (remainder, _) = tag(")")(remainder)?;
    let p = Expression::Pair(e1, e2);
    Ok((remainder, p))
}

fn parse_fst(input: &str) -> IResult<&str, Expression> {
    let (remainder, _) = tag("fst")(input)?;
    let (remainder, _) = tag("(")(remainder)?;
    let (remainder, e) = parse_e_top(remainder)?;
    let (remainder, _) = tag(")")(remainder)?;
    let f = Expression::Fst(e);
    Ok((remainder, f))
}

fn parse_snd(input: &str) -> IResult<&str, Expression> {
    let (remainder, _) = tag("snd")(input)?;
    let (remainder, _) = tag("(")(remainder)?;
    let (remainder, e) = parse_e_top(remainder)?;
    let (remainder, _) = tag(")")(remainder)?;
    let f = Expression::Snd(e);
    Ok((remainder, f))
}

fn parse_hd(input: &str) -> IResult<&str, Expression> {
    let (remainder, _) = tag("hd")(input)?;
    let (remainder, _) = tag("(")(remainder)?;
    let (remainder, e) = parse_e_top(remainder)?;
    let (remainder, _) = tag(")")(remainder)?;
    let h = Expression::Hd(e);
    Ok((remainder, h))
}

fn parse_tl(input: &str) -> IResult<&str, Expression> {
    let (remainder, _) = tag("tl")(input)?;
    let (remainder, _) = tag("(")(remainder)?;
    let (remainder, e) = parse_e_top(remainder)?;
    let (remainder, _) = tag(")")(remainder)?;
    let t = Expression::Tl(e);
    Ok((remainder, t))
}

fn parse_pred(input: &str) -> IResult<&str, Expression> {
    let (remainder, _) = tag("pred")(input)?;
    let (remainder, _) = tag("(")(remainder)?;
    let (remainder, e) = parse_e_top(remainder)?;
    let (remainder, _) = tag(")")(remainder)?;
    let p = Expression::Succ(e);
    Ok((remainder, p))
}

fn parse_e_null(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_e_variable,
        parse_bool,
        parse_num,
        parse_let,
        parse_not,
        parse_if,
        parse_succ,
        parse_pair,
        parse_fst,
        parse_snd,
        parse_nil,
        parse_hd,
        parse_tl,
        parse_pred,
    ))(input)
}

fn parse_fn(input: &str) -> IResult<&str, Expression> {
    let (remainder, _) = tag("fn")(input)?;
    let (remainder, v) = parse_variable(remainder)?;
    let (remainder, _) = tag(".")(remainder)?;
    let (remainder, e) = parse_e_top(remainder)?;
    let f = Expression::Fn(v, e);
    Ok((remainder, f))
}

fn parse_e_fifth(input: &str) -> IResult<&str, Expression> {
    alt((parse_fn, parse_e_null))(input)
}

fn parse_eq(input: &str) -> IResult<&str, Expression> {
    let (remainder, e1) = parse_e_top(input)?;
    let (remainder, _) = tag("==")(remainder)?;
    let (remainder, e2) = parse_e_top(remainder)?;
    let eq = Expression::Eq(e1, e2);
    Ok((remainder, eq))
}

fn parse_e_fourth(input: &str) -> IResult<&str, Expression> {
    alt((parse_eq, parse_e_fifth))(input)
}

fn parse_cons(input: &str) -> IResult<&str, Expression> {
    let (remainder, e1) = parse_e_top(input)?;
    let (remainder, _) = tag("::")(remainder)?;
    let (remainder, e2) = parse_e_top(remainder)?;
    let eq = Expression::Cons(e1, e2);
    Ok((remainder, eq))
}

fn parse_e_third(input: &str) -> IResult<&str, Expression> {
    alt((parse_cons, parse_e_fourth))(input)
}

fn parse_and(input: &str) -> IResult<&str, Expression> {
    let (remainder, e1) = parse_e_top(input)?;
    let (remainder, _) = tag("and")(remainder)?;
    let (remainder, e2) = parse_e_top(remainder)?;
    let eq = Expression::And(e1, e2);
    Ok((remainder, eq))
}

fn parse_e_second(input: &str) -> IResult<&str, Expression> {
    alt((parse_and, parse_e_third))(input)
}

fn parse_add(input: &str) -> IResult<&str, Expression> {
    let (remainder, e1) = parse_e_top(input)?;
    let (remainder, _) = tag("+")(remainder)?;
    let (remainder, e2) = parse_e_top(remainder)?;
    let eq = Expression::Add(e1, e2);
    Ok((remainder, eq))
}

fn parse_e_first(input: &str) -> IResult<&str, Expression> {
    alt((parse_add, parse_e_second))(input)
}

fn parse_apply_1(input: &str) -> IResult<&str, Expression> {
    let (remainder, e1) = parse_e_top(input)?;
    let (remainder, _) = tag("(")(remainder)?;
    let (remainder, e2) = parse_e_top(remainder)?;
    let (remainder, _) = tag(")")(remainder)?;
    let a = Expression::Apply(e1, e2);
    Ok((remainder, a))
}

fn parse_apply_2(input: &str) -> IResult<&str, Expression> {
    let (remainder, e1) = parse_e_top(input)?;
    let (remainder, e2) = parse_e_top(remainder)?;
    let a = Expression::Apply(e1, e2);
    Ok((remainder, a))
}

fn parse_e_zeroth(input: &str) -> IResult<&str, Expression> {
    alt((parse_apply_1, parse_apply_2, parse_e_first))(input)
}

fn parse_e_top_bracket(input: &str) -> IResult<&str, Expression> {
    let (remainder, _) = tag("(")(input)?;
    let (remainder, e) = parse_e_top(remainder)?;
    let (remainder, _) = tag(")")(remainder)?;
    Ok((remainder, e))
}

fn parse_e_top(input: &str) -> IResult<&str, Expression> {
    alt((parse_e_top_bracket, parse_e_zeroth))(input)
}
