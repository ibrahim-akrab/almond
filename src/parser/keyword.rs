//! Parsing for JS keywords

#![allow(dead_code)] // some keyword_* functions are never used but are included for future use cases and for consistency.

use crate::parser::util::*;
use crate::parser::*;
use logos::Logos;

/// A finite state automata for checking if next token is a keyword. Not actually used for parsing.
///
/// **Important implementation note**: When adding new keywords, make sure it is added both to `KeywordLex` and a `parse_*` function.
#[derive(Logos)]
enum KeywordLex {
    // parse_keyword
    #[token("break")]
    Break,
    #[token("do")]
    Do,
    #[token("instanceof")]
    Instanceof,
    #[token("typeof")]
    Typeof,
    #[token("case")]
    Case,
    #[token("else")]
    Else,
    #[token("new")]
    New,
    #[token("var")]
    Var,
    #[token("catch")]
    Catch,
    #[token("finally")]
    Finally,
    #[token("return")]
    Return,
    #[token("void")]
    Void,
    #[token("continue")]
    Continue,
    #[token("for")]
    For,
    #[token("switch")]
    Switch,
    #[token("while")]
    While,
    #[token("debugger")]
    Debugger,
    #[token("function")]
    Function,
    #[token("this")]
    This,
    #[token("with")]
    With,
    #[token("default")]
    Default,
    #[token("if")]
    If,
    #[token("throw")]
    Throw,
    #[token("delete")]
    Delete,
    #[token("in")]
    In,
    #[token("try")]
    Try,
    // parse_future_reserved_word_strict
    #[token("implements")]
    Implements,
    #[token("let")]
    Let,
    #[token("private")]
    Private,
    #[token("public")]
    Public,
    #[token("interface")]
    Interface,
    #[token("package")]
    Package,
    #[token("protected")]
    Protected,
    #[token("static")]
    Static,
    #[token("yield")]
    Yield,
    // parse_future_reserved_word_lax
    #[token("class")]
    Class,
    #[token("enum")]
    Enum,
    #[token("extends")]
    Extends,
    #[token("super")]
    Super,
    #[token("const")]
    Const,
    #[token("export")]
    Export,
    #[token("import")]
    Import,
    #[token("await")]
    Await,
    // parse_reserved_word
    #[token("null")]
    Null,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[error]
    Error,
}

/// Returns `true` if next token is a reserved word.
pub fn reserved_word(s: Span) -> ParseResult<()> {
    let mut lexer = KeywordLex::lexer(&s);
    let next = lexer.next();
    match next {
        None | Some(KeywordLex::Error) => Err(nom::Err::Error(nom::error::Error {
            code: nom::error::ErrorKind::Tag,
            input: s,
        })),
        Some(_token) => {
            let (s, _) = take(lexer.span().len())(s)?;
            not(identifier_continue)(s)
        }
    }
}

/// Succeeds if parsed a reserved word. Should be used with `not` to check if an identifier is not a reserved word.
#[deprecated = "use reserved_word"]
pub fn parse_reserved_word(s: Span) -> ParseResult<()> {
    alt((
        parse_keyword,
        parse_future_reserved_word,
        value((), ws0(null_lit)),
        value((), ws0(bool_lit)),
    ))(s)
}

/// Succeeds if parsed is a keyword. Use `reserved_word` with `not` instead to check if an identifier is not a reserved word.
pub fn parse_keyword(s: Span) -> ParseResult<()> {
    ws0(alt((
        keyword_break,
        keyword_do,
        keyword_instanceof,
        keyword_typeof,
        keyword_case,
        keyword_else,
        keyword_new,
        keyword_var,
        keyword_catch,
        keyword_finally,
        keyword_return,
        keyword_void,
        keyword_continue,
        keyword_for,
        keyword_switch,
        keyword_while,
        keyword_debugger,
        keyword_function,
        keyword_this,
        keyword_with,
        alt((
            keyword_default,
            keyword_if,
            keyword_throw,
            keyword_delete,
            keyword_in,
            keyword_try,
        )),
    )))(s)
}

pub fn parse_future_reserved_word_lax(s: Span) -> ParseResult<()> {
    ws0(alt((
        keyword_class,
        keyword_enum,
        keyword_extends,
        keyword_super,
        keyword_const,
        keyword_export,
        keyword_import,
        keyword_await,
    )))(s)
}

pub fn parse_future_reserved_word_strict(s: Span) -> ParseResult<()> {
    ws0(alt((
        parse_future_reserved_word_lax,
        keyword_implements,
        keyword_let,
        keyword_private,
        keyword_public,
        keyword_interface,
        keyword_package,
        keyword_protected,
        keyword_static,
        keyword_yield,
    )))(s)
}

pub fn parse_future_reserved_word(s: Span) -> ParseResult<()> {
    parse_future_reserved_word_strict(s)
}

pub fn keyword_break(s: Span) -> ParseResult<()> {
    value((), pair(tag("break"), not(identifier_continue)))(s)
}
pub fn keyword_do(s: Span) -> ParseResult<()> {
    value((), pair(tag("do"), not(identifier_continue)))(s)
}
pub fn keyword_instanceof(s: Span) -> ParseResult<()> {
    value((), pair(tag("instanceof"), not(identifier_continue)))(s)
}
pub fn keyword_typeof(s: Span) -> ParseResult<()> {
    value((), pair(tag("typeof"), not(identifier_continue)))(s)
}
pub fn keyword_case(s: Span) -> ParseResult<()> {
    value((), pair(tag("case"), not(identifier_continue)))(s)
}
pub fn keyword_else(s: Span) -> ParseResult<()> {
    value((), pair(tag("else"), not(identifier_continue)))(s)
}
pub fn keyword_new(s: Span) -> ParseResult<()> {
    value((), pair(tag("new"), not(identifier_continue)))(s)
}
pub fn keyword_var(s: Span) -> ParseResult<()> {
    value((), pair(tag("var"), not(identifier_continue)))(s)
}
pub fn keyword_catch(s: Span) -> ParseResult<()> {
    value((), pair(tag("catch"), not(identifier_continue)))(s)
}
pub fn keyword_finally(s: Span) -> ParseResult<()> {
    value((), pair(tag("finally"), not(identifier_continue)))(s)
}
pub fn keyword_return(s: Span) -> ParseResult<()> {
    value((), pair(tag("return"), not(identifier_continue)))(s)
}
pub fn keyword_void(s: Span) -> ParseResult<()> {
    value((), pair(tag("void"), not(identifier_continue)))(s)
}
pub fn keyword_continue(s: Span) -> ParseResult<()> {
    value((), pair(tag("continue"), not(identifier_continue)))(s)
}
pub fn keyword_for(s: Span) -> ParseResult<()> {
    value((), pair(tag("for"), not(identifier_continue)))(s)
}
pub fn keyword_switch(s: Span) -> ParseResult<()> {
    value((), pair(tag("switch"), not(identifier_continue)))(s)
}
pub fn keyword_while(s: Span) -> ParseResult<()> {
    value((), pair(tag("while"), not(identifier_continue)))(s)
}
pub fn keyword_debugger(s: Span) -> ParseResult<()> {
    value((), pair(tag("debugger"), not(identifier_continue)))(s)
}
pub fn keyword_function(s: Span) -> ParseResult<()> {
    value((), pair(tag("function"), not(identifier_continue)))(s)
}
pub fn keyword_this(s: Span) -> ParseResult<()> {
    value((), pair(tag("this"), not(identifier_continue)))(s)
}
pub fn keyword_with(s: Span) -> ParseResult<()> {
    value((), pair(tag("with"), not(identifier_continue)))(s)
}
pub fn keyword_default(s: Span) -> ParseResult<()> {
    value((), pair(tag("default"), not(identifier_continue)))(s)
}
pub fn keyword_if(s: Span) -> ParseResult<()> {
    value((), pair(tag("if"), not(identifier_continue)))(s)
}
pub fn keyword_throw(s: Span) -> ParseResult<()> {
    value((), pair(tag("throw"), not(identifier_continue)))(s)
}
pub fn keyword_delete(s: Span) -> ParseResult<()> {
    value((), pair(tag("delete"), not(identifier_continue)))(s)
}
pub fn keyword_of(s: Span) -> ParseResult<()> {
    value((), pair(tag("of"), not(identifier_continue)))(s)
}
pub fn keyword_in(s: Span) -> ParseResult<()> {
    value((), pair(tag("in"), not(identifier_continue)))(s)
}
pub fn keyword_try(s: Span) -> ParseResult<()> {
    value((), pair(tag("try"), not(identifier_continue)))(s)
}
pub fn keyword_get(s: Span) -> ParseResult<()> {
    value((), pair(tag("get"), not(identifier_continue)))(s)
}
pub fn keyword_set(s: Span) -> ParseResult<()> {
    value((), pair(tag("set"), not(identifier_continue)))(s)
}
pub fn keyword_class(s: Span) -> ParseResult<()> {
    value((), pair(tag("class"), not(identifier_continue)))(s)
}
pub fn keyword_enum(s: Span) -> ParseResult<()> {
    value((), pair(tag("enum"), not(identifier_continue)))(s)
}
pub fn keyword_extends(s: Span) -> ParseResult<()> {
    value((), pair(tag("extends"), not(identifier_continue)))(s)
}
pub fn keyword_super(s: Span) -> ParseResult<()> {
    value((), pair(tag("super"), not(identifier_continue)))(s)
}
pub fn keyword_const(s: Span) -> ParseResult<()> {
    value((), pair(tag("const"), not(identifier_continue)))(s)
}
pub fn keyword_export(s: Span) -> ParseResult<()> {
    value((), pair(tag("export"), not(identifier_continue)))(s)
}
pub fn keyword_import(s: Span) -> ParseResult<()> {
    value((), pair(tag("import"), not(identifier_continue)))(s)
}
pub fn keyword_implements(s: Span) -> ParseResult<()> {
    value((), pair(tag("implements"), not(identifier_continue)))(s)
}
pub fn keyword_let(s: Span) -> ParseResult<()> {
    value((), pair(tag("let"), not(identifier_continue)))(s)
}
pub fn keyword_private(s: Span) -> ParseResult<()> {
    value((), pair(tag("private"), not(identifier_continue)))(s)
}
pub fn keyword_public(s: Span) -> ParseResult<()> {
    value((), pair(tag("public"), not(identifier_continue)))(s)
}
pub fn keyword_interface(s: Span) -> ParseResult<()> {
    value((), pair(tag("interface"), not(identifier_continue)))(s)
}
pub fn keyword_package(s: Span) -> ParseResult<()> {
    value((), pair(tag("package"), not(identifier_continue)))(s)
}
pub fn keyword_protected(s: Span) -> ParseResult<()> {
    value((), pair(tag("protected"), not(identifier_continue)))(s)
}
pub fn keyword_static(s: Span) -> ParseResult<()> {
    value((), pair(tag("static"), not(identifier_continue)))(s)
}
pub fn keyword_yield(s: Span) -> ParseResult<()> {
    value((), pair(tag("yield"), not(identifier_continue)))(s)
}
// es2017
pub fn keyword_async(s: Span) -> ParseResult<()> {
    value((), pair(tag("async"), not(identifier_continue)))(s)
}
pub fn keyword_await(s: Span) -> ParseResult<()> {
    value((), pair(tag("await"), not(identifier_continue)))(s)
}
