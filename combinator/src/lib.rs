//! 解析器组合器
//! 将两个解析器作为输入并返回一个新的解析器，并按照顺序解析它们
//! 原文地址：https://bodil.lol/parser-combinators/

#![allow(dead_code)]

type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;

    // /// 如果可以这样做，那么所有的parser都可以使用`parser.map`的方式使用`map`方法
    // /// 这样在使用上可以更加直观，方便
    // /// 但问题是，在trait中`impl Parser<'a, Output>`无法作为返回值
    // fn map<B, F>(&self, map_fn: F) -> impl Parser<'a, Output>
    // where
    //     F: Fn(Output) -> B,
    // {
    //     move |input| match self.parse(input) {
    //         Ok((next_input, result)) => Ok((next_input, map_fn(result))),
    //         Err(err) => Err(err),
    //     }
    // }
    fn map<F, NewOutput>(self, map_fn: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        F: Fn(Output) -> NewOutput + 'a,
    {
        BoxedParser::new(map(self, map_fn))
    }

    fn pred<F>(self, pred_fn: F) -> BoxedParser<'a, Output>
    where
        Self: Sized + 'a,
        Output: 'a,
        F: Fn(&Output) -> bool + 'a,
    {
        BoxedParser::new(pred(self, pred_fn))
    }

    fn and_then<F, NextParser, NewOutput>(self, f: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        NextParser: Parser<'a, NewOutput> + 'a,
        F: Fn(Output) -> NextParser + 'a,
    {
        BoxedParser::new(and_then(self, f))
    }
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<'a, Output>,
{
    /// parse的返回值： ParseResult<Output>
    /// 必须与F的返回值： ParseResult<'a, Output>拥有相同的生命周期.
    /// 也就是要将这两个返回值的生命周期相关联: 'a
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}

// fn the_letter_a(input: &str) -> Result<(&str, ()), &str> {
//     match input.chars().next() {
//         Some('a') => Ok((&input['a'.len_utf8()..], ())),
//         _ => Err(input),
//     }
// }
fn the_letter_a(input: &str) -> ParseResult<()> {
    match input.chars().next() {
        Some('a') => Ok((&input['a'.len_utf8()..], ())),
        _ => Err(input),
    }
}

// fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ()), &str> {
//     move |input| match input.get(0..expected.len()) {
//         Some(next) if next == expected => {
//             Ok((&input[expected.len()..], ()))
//         }
//         _ => Err(input)
//     }
// }
// fn match_literal(expected: &'static str) -> impl Fn(&str) -> ParseResult<()> {
//     move |input| match input.get(0..expected.len()) {
//         Some(next) if next == expected => {
//             Ok((&input[expected.len()..], ()))
//         }
//         _ => Err(input)
//     }
// }
fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

// /// 元素名称标志符的规则: 首位是字母, 后跟零个或多个字母, 数字或-
// fn identifier(input: &str) -> Result<(&str, String), &str> {
//     let mut matched = String::new();
//     let mut chars = input.chars();
//
//     match chars.next() {
//         // 第一个是字母
//         Some(next) if next.is_alphabetic() => matched.push(next),
//         _ => return Err(input),
//     }
//
//     while let Some(next) = chars.next() {
//         // 字母或者数字
//         if next.is_alphanumeric() || next == '-' {
//             matched.push(next);
//         } else {
//             break;
//         }
//     }
//
//     let next_index = matched.len();
//     Ok((&input[next_index..], matched))
// }
/// 元素名称标志符的规则: 首位是字母, 后跟零个或多个字母, 数字或-
fn identifier(input: &str) -> ParseResult<String> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        // 第一个是字母
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input),
    }

    while let Some(next) = chars.next() {
        // 字母或者数字
        if next.is_alphanumeric() || next == '-' {
            matched.push(next);
        } else {
            break;
        }
    }

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
}

// /// 解析器组合器
// /// 将两个解析器作为输入并返回一个新的解析器，并按照顺序解析它们
// fn pair<P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Fn(&str) -> Result<(&str, (R1, R2)),
// &str>     where
//         P1: Fn(&str) -> Result<(&str, R1), &str>,
//         P2: Fn(&str) -> Result<(&str, R2), &str>,
// {
//     move |input| match parser1(input) {
//         Ok((next_input, result1)) => match parser2(next_input) {
//             Ok((final_input, result2)) => Ok((final_input, (result1, result2))),
//             Err(err) => Err(err),
//         }
//         Err(err) => Err(err),
//     }
// }
// /// 解析器组合器
// /// 将两个解析器作为输入并返回一个新的解析器，并按照顺序解析它们
// fn pair<P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Fn(&str) -> ParseResult<(R1, R2)>
//     where
//         P1: Fn(&str) -> ParseResult<R1>,
//         P2: Fn(&str) -> ParseResult<R2>,
// {
//     move |input| match parser1(input) {
//         Ok((next_input, result1)) => match parser2(next_input) {
//             Ok((final_input, result2)) => Ok((final_input, (result1, result2))),
//             Err(err) => Err(err),
//         }
//         Err(err) => Err(err),
//     }
// }
/// 解析器组合器
/// 将两个解析器作为输入并返回一个新的解析器，并按照顺序解析它们
fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| match parser1.parse(input) {
        Ok((next_input, result1)) => match parser2.parse(next_input) {
            Ok((final_input, result2)) => Ok((final_input, (result1, result2))),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }

    // `.map`函数消耗自身`self`，所以parser2需要被消耗掉
    // 但`and_then`函数传入的参数是`Fn`而不是`FnOnce`，所以不允许消耗parser2
    // 所以这两者之间存在冲突，导致是个方法无法使用
    // parser1.and_then(|result1| parser2.map(move |result2| (result1.clone(), result2)))
}

// /// 这个解析器组合器目的是：改变结果的类型
// /// 例如有一个解析器返回((), String), 但你希望能够将其返回类型修改为String
// /// 这种模式在Haskell以及范畴论(category theory)中被称为"函子(functor)"
// fn map<P, F, A, B>(parser: P, map_fn: F) -> impl Fn(&str) -> Result<(&str, B), &str>
//     where
//         P: Fn(&str) -> Result<(&str, A), &str>,
//         F: Fn(A) -> B,
// {
//     move |input| match parser(input) {
//         Ok((next_input, result)) => Ok((next_input, map_fn(result))),
//         Err(err) => Err(err),
//     }
// }
// /// 这个解析器组合器目的是：改变结果的类型
// /// 例如有一个解析器返回((), String), 但你希望能够将其返回类型修改为String
// /// 这种模式在Haskell以及范畴论(category theory)中被称为"函子(functor)"
// fn map<P, F, A, B>(parser: P, map_fn: F) -> impl Fn(&str) -> ParseResult<B>
//     where
//         P: Fn(&str) -> ParseResult<A>,
//         F: Fn(A) -> B,
// {
//     move |input| match parser(input) {
//         Ok((next_input, result)) => Ok((next_input, map_fn(result))),
//         Err(err) => Err(err),
//     }
// }
/// 这个解析器组合器目的是：改变结果的类型
/// 例如有一个解析器返回((), String), 但你希望能够将其返回类型修改为String
/// 这种模式在Haskell以及范畴论(category theory)中被称为"函子(functor)"
fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| match parser.parse(input) {
        Ok((next_input, result)) => Ok((next_input, map_fn(result))),
        Err(err) => Err(err),
    }
}

/// left组合器
fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}

/// right组合器
fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_left, right)| right)
}

/// 在我们得到第一个可选属性对之前我们必须处理一些事情：空格
/// 需要处理一个或多个空格，因为<element attributes="value"/>也是一个合法的语法，即使它的空格很多
/// 编写一个组合器来表示一个或多个解析器
fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        match parser.parse(input) {
            // 解析第一个元素，如果不存在，则返回一个错误
            Ok((next_input, first_result)) => {
                input = next_input;
                result.push(first_result);
            }
            Err(err) => return Err(err),
        }

        // 尽可能多的解析元素，直到解析失败
        while let Ok((next_input, next_result)) = parser.parse(input) {
            input = next_input;
            result.push(next_result);
        }

        Ok((input, result))
    }
}
// /// 在我们得到第一个可选属性对之前我们必须处理一些事情：空格
// /// 需要处理一个或多个空格，因为<element attributes="value"/>也是一个合法的语法，即使它的空格很多
// /// 编写一个组合器来表示一个或多个解析器
// fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
//     where
//         P: Parser<'a, A>,
// {
//     // 此处触犯了所有权，parser被传递了两次
//     // parser解析器是函数，所以没有实现Clone
//     map(pair(parser, zero_or_more(parser)), |(head, mut tail)| {
//         tail.insert(0, head);
//         tail
//     })
// }

/// 支持解析零次或多次的解析器
fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        // 尽可能多的解析元素，直到解析失败
        while let Ok((next_input, next_result)) = parser.parse(input) {
            input = next_input;
            result.push(next_result);
        }

        Ok((input, result))
    }
}

/// 只要输入中还剩下一个字符，它就返回一个字符
fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..], next)),
        _ => Err(input),
    }
}

/// 谓词组合器
/// 我们调用解析器，然后在解析器成功时对值调用谓词函数
/// 只有当返回 true 时我们才真正返回成功，否则我们返回与解析失败一样多的错误
fn pred<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
{
    move |input| {
        if let Ok((next_input, result)) = parser.parse(input) {
            if predicate(&result) {
                return Ok((next_input, result));
            }
        }

        Err(input)
    }
}

/// 一个用于单个空白项的解析器
fn whitespace_char<'a>() -> impl Parser<'a, char> {
    pred(any_char, |c| c.is_whitespace())
}

/// 一个或多个空白
fn space1<'a>() -> impl Parser<'a, Vec<char>> {
    one_or_more(whitespace_char())
}

/// 零个或多个空白
fn space0<'a>() -> impl Parser<'a, Vec<char>> {
    zero_or_more(whitespace_char())
}

/// 带引号的字符串，去掉引号并取回引号中间的值
/// 1. 一个引号
/// 2. 后跟零个或多个非引号的字符
/// 3. 接着是另一个引号
fn quoted_string<'a>() -> impl Parser<'a, String> {
    // map(
    //     // 保留右边，也就是left
    //     right(
    //         match_literal("\""),
    //         // 保留左边，也就是zero_or_more
    //         left(zero_or_more(pred(any_char, |c| *c != '"')), match_literal("\"")),
    //     ),
    //     // 将Vec<char>转为String
    //     |chars| chars.into_iter().collect(),
    // )

    right(
        match_literal("\""),
        left(zero_or_more(any_char.pred(|c| *c != '"')), match_literal("\"")),
    )
    .map(|chars| chars.into_iter().collect())
}

/// 属性解析器
fn attribute_pair<'a>() -> impl Parser<'a, (String, String)> {
    // 去掉=，获取attribute元组
    pair(identifier, right(match_literal("="), quoted_string()))
}

/// 一个或多个属性的解析器
fn attributes<'a>() -> impl Parser<'a, Vec<(String, String)>> {
    // 不要忘记添加attribute之间的空格（至少有一个空格）
    zero_or_more(right(space1(), attribute_pair()))
}

/// < and element_name and attributes
fn element_start<'a>() -> impl Parser<'a, (String, Vec<(String, String)>)> {
    right(match_literal("<"), pair(identifier, attributes()))
}

/// 为单个元素创建一个解析器
fn single_element<'a>() -> impl Parser<'a, Element> {
    left(element_start(), match_literal("/>")).map(|(name, attributes)| Element {
        name,
        attributes,
        children: vec![],
    })
}

fn open_element<'a>() -> impl Parser<'a, Element> {
    left(element_start(), match_literal(">")).map(|(name, attributes)| Element {
        name,
        attributes,
        children: vec![],
    })
}

fn either<'a, P1, P2, A>(parser1: P1, parser2: P2) -> impl Parser<'a, A>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, A>,
{
    move |input| match parser1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => parser2.parse(input),
    }
}

fn element<'a>() -> impl Parser<'a, Element> {
    whitespace_wrap(either(single_element(), parent_element()))
}

/// 结束标记的解析器
fn close_element<'a>(expected_name: String) -> impl Parser<'a, String> {
    // 因为`right`整个是需要返回的，所以所有权必须转移出去
    // 也就是说`expected_name`的所有权也必须一起转移出去
    right(match_literal("</"), left(identifier, match_literal(">")))
        .pred(move |name| name == &expected_name)
}

fn parent_element<'a>() -> impl Parser<'a, Element> {
    // pair(open_element(), left(zero_or_more(element()), close_element(..oops)))

    open_element().and_then(|el| {
        left(zero_or_more(element()), close_element(el.name.clone())).map(move |children| {
            // 这里必须使用`clone`，因为`Fn`闭包不能改变它们捕获的变量，所以无法直接给`el.
            // children`赋值
            let mut el = el.clone();
            el.children = children;
            el
        })
    })
}

/// 如果你有一个`Thing<A>`，并且你有一个可用的`and_then`函数
/// 你可以将一个函数从`A`传递给`Thing<B>`，这样现在你就有了一个新的`Thing<B>`，这是一个单子
/// `map`被称为函子，它将结果进行第二次转换
/// `and_then`被称为单子，它将执行的函数进行第二次转换（链式执行函数）
fn and_then<'a, P, F, A, B, NextP>(parser: P, f: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    NextP: Parser<'a, B>,
    F: Fn(A) -> NextP,
{
    move |input| match parser.parse(input) {
        Ok((next_input, result)) => f(result).parse(next_input),
        Err(err) => Err(err),
    }
}

/// 它将忽略`element`周围的所有前导和尾随空格
/// 这意味着我们可以随意使用任意数量的换行符和缩进。
fn whitespace_wrap<'a, P, A>(parser: P) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
{
    // 两头都可以有0个或多个空格
    right(space0(), left(parser, space0()))
}

/// 'a不单止与Parser<'a>相关联，也与dyn Parser<'a, Output>相关联
/// 这使我们能够将解析器函数放入Box中，并且BoxedParser将像函数一样用作解析器
/// 这意味着将装箱的解析器移动到堆中并且必须取消引用指针才能到达它，这可能会花费我们几个宝贵的纳秒
/// 因此我们实际上可能想要推迟装箱所有内容。只需装箱一些比较流行的组合器就足够了
struct BoxedParser<'a, Output> {
    /// 因为trait对象可以包含引用，所以这些引用的生存期需要表示为trait对象的一部分
    parser: Box<dyn Parser<'a, Output> + 'a>,
}

impl<'a, Output> BoxedParser<'a, Output> {
    fn new<P>(parser: P) -> Self
    where
        P: Parser<'a, Output> + 'a,
    {
        Self { parser: Box::new(parser) }
    }
}

impl<'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self.parser.parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_parser() {
        let parse_joe = match_literal("Hello Joe!");
        assert_eq!(Ok(("", ())), parse_joe.parse("Hello Joe!"));
        assert_eq!(Ok((" Hello Robert!", ())), parse_joe.parse("Hello Joe! Hello Robert!"));
        assert_eq!(Err("Hello Mike!"), parse_joe.parse("Hello Mike!"));
    }

    #[test]
    fn identifier_parser() {
        assert_eq!(Ok(("", "i-am-an-identifier".to_owned())), identifier("i-am-an-identifier"));
        assert_eq!(
            Ok((" entirely an identifier", "not".to_owned())),
            identifier("not entirely an identifier")
        );
        assert_eq!(Err("!not at all an identifier"), identifier("!not at all an identifier"));
    }

    #[test]
    fn pair_combinator() {
        let tag_opener = pair(match_literal("<"), identifier);
        assert_eq!(
            // 其实我们并不关心第一个解释器的结果（也就是这里的()）
            // 我们很多解析器只匹配输入中的模式但不产生值，因此可以安全地忽略它们的输出
            Ok(("/>", ((), "my-first-element".to_owned()))),
            tag_opener.parse("<my-first-element/>")
        );
        assert_eq!(Err("oops"), tag_opener.parse("oops"));
        assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
    }

    #[test]
    fn right_combinator() {
        let tag_opener = right(match_literal("<"), identifier);
        assert_eq!(
            // 此处与pair_combinator比较
            // 结果中没有了那个不需要的值()
            Ok(("/>", "my-first-element".to_owned())),
            tag_opener.parse("<my-first-element/>")
        );
        assert_eq!(Err("oops"), tag_opener.parse("oops"));
        assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
    }

    #[test]
    fn one_or_more_combinator() {
        let parser = one_or_more(match_literal("ha"));
        assert_eq!(Ok(("", vec![(), (), ()])), parser.parse(r"hahaha"));
        // 此处会返回错误，因为无法解析第一个元素
        assert_eq!(Err(r"ahah"), parser.parse(r"ahah"));
        // 此处会返回错误，因为无法解析第一个元素
        assert_eq!(Err(""), parser.parse(""));
    }

    #[test]
    fn zero_or_more_combinator() {
        let parser = zero_or_more(match_literal("ha"));
        assert_eq!(Ok(("", vec![(), (), ()])), parser.parse(r"hahaha"));
        // 此处返回成功，但是解析的结果为空数组
        assert_eq!(Ok((r"ahah", vec![])), parser.parse(r"ahah"));
        // 此处返回成功，但是解析的结果为空数组
        assert_eq!(Ok(("", vec![])), parser.parse(""));
    }

    #[test]
    fn predicate_combinator() {
        let parser = pred(any_char, |c| *c == 'o');
        assert_eq!(Ok(("mg", 'o')), parser.parse("omg"));
        assert_eq!(Err("lol"), parser.parse("lol"));
    }

    #[test]
    fn quoted_string_parser() {
        assert_eq!(Ok(("", "Hello Joe!".to_owned())), quoted_string().parse("\"Hello Joe!\""));
    }

    #[test]
    fn attribute_parser() {
        assert_eq!(
            Ok(("", vec![("one".to_owned(), "1".to_owned()), ("two".to_owned(), "2".to_owned())])),
            attributes().parse(" one=\"1\" two=\"2\"")
        );
    }

    #[test]
    fn single_element_parser() {
        assert_eq!(
            Ok((
                "",
                Element {
                    name: "div".to_owned(),
                    attributes: vec![("class".to_owned(), "float".to_owned())],
                    children: vec![]
                }
            )),
            single_element().parse("<div class=\"float\"/>")
        );
    }

    #[test]
    fn xml_parser() {
        let doc = r#"
            <top label="Top">
                <semi-bottom label="Bottom"/>
                <middle>
                    <bottom label="Another bottom"/>
                </middle>
            </top>"#;
        let parsed_doc = Element {
            name: "top".to_owned(),
            attributes: vec![("label".to_owned(), "Top".to_owned())],
            children: vec![
                Element {
                    name: "semi-bottom".to_string(),
                    attributes: vec![("label".to_string(), "Bottom".to_string())],
                    children: vec![],
                },
                Element {
                    name: "middle".to_string(),
                    attributes: vec![],
                    children: vec![Element {
                        name: "bottom".to_string(),
                        attributes: vec![("label".to_string(), "Another bottom".to_string())],
                        children: vec![],
                    }],
                },
            ],
        };
        assert_eq!(Ok(("", parsed_doc)), element().parse(doc))
    }

    #[test]
    fn mismatched_closing_tag() {
        let doc = r#"
            <top>
                <bottom/>
            </middle>"#;
        assert_eq!(Err("</middle>"), element().parse(doc));
    }
}
