use lispers_common::{StringInterner, Backend, Symbol};
use crate::lexer::{Token, TokenStream};
use crate::ast::{SExpression, Literal};

peg::parser!{
  pub grammar module_parser<'source>() for TokenStream<'source> {
    pub rule module<S: Symbol, B: Backend<S>>(
      interner: &mut StringInterner<B>
    ) -> Vec<SExpression<S>>
      = exprs:s_expression(interner)+
      { exprs }

    rule s_expression<S: Symbol, B: Backend<S>>(
      interner: &mut StringInterner<B>
    ) -> SExpression<S>
      = literal(interner)
      / list(interner)

    rule list<S: Symbol, B: Backend<S>>(
      interner: &mut StringInterner<B>
    ) -> SExpression<S>
      = [Token::ParenOpen] children:s_expression(interner)* [Token::ParenClose]
      { SExpression::List(children) }

    rule literal<S: Symbol, B: Backend<S>>(
      interner: &mut StringInterner<B>
    ) -> SExpression<S>
      = literal_boolean()
      / literal_integer()
      / literal_float()
      / literal_string()
      / literal_symbol(interner)

    rule literal_boolean<S: Symbol>() -> SExpression<S>
      = literal_boolean_true()
      / literal_boolean_false()

    rule literal_boolean_true<S: Symbol>() -> SExpression<S>
      = [Token::True]
      { SExpression::Literal(Literal::Boolean(true)) }

    rule literal_boolean_false<S: Symbol>() -> SExpression<S>
      = [Token::False]
      { SExpression::Literal(Literal::Boolean(false)) }

    rule literal_integer<S: Symbol>() -> SExpression<S>
      = literal_integer_2()
      / literal_integer_8()
      / literal_integer_10()
      / literal_integer_16()

    rule literal_integer_2<S: Symbol>() -> SExpression<S>
      = [Token::IntegerBase2(n)]
      { SExpression::Literal(Literal::Integer(*n)) }

    rule literal_integer_8<S: Symbol>() -> SExpression<S>
      = [Token::IntegerBase8(n)]
      { SExpression::Literal(Literal::Integer(*n)) }

    rule literal_integer_10<S: Symbol>() -> SExpression<S>
      = [Token::IntegerBase10(n)]
      { SExpression::Literal(Literal::Integer(*n)) }

    rule literal_integer_16<S: Symbol>() -> SExpression<S>
      = [Token::IntegerBase16(n)]
      { SExpression::Literal(Literal::Integer(*n)) }

    rule literal_float<S: Symbol>() -> SExpression<S>
      = [Token::Float(n)]
      { SExpression::Literal(Literal::Float(*n)) }

    rule literal_string<S: Symbol>() -> SExpression<S>
      = [Token::String(s)]
      { SExpression::Literal(Literal::String(s.clone())) }

    rule literal_symbol<S: Symbol, B: Backend<S>>(
      interner: &mut StringInterner<B>
    ) -> SExpression<S>
      = [Token::Symbol(s)]
      {
        let sym = interner.get_or_intern(s);
        SExpression::Literal(Literal::Symbol(sym))
      }
  }
}
