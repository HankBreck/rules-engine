%start Expr
%expect-unused Unmatched "UNMATCHED"
%%
Expr -> Result<Expression, ()>:
      Logical { Ok(Expression::Logical($1?)) }
;

Logical -> Result<LogicalExpression, ()>:
      Equality 'AND' Equality {
        Ok(LogicalExpression::And(Box::new($1?), Box::new($3?)) )
      }
    | Equality 'OR' Equality {
        Ok(LogicalExpression::Or(Box::new($1?), Box::new($3?)) )
      }
    | Equality { Ok(LogicalExpression::Equality($1?)) }
;

Equality -> Result<EqualityExpression, ()>:
      Comparison 'EQ' Comparison {
        Ok(EqualityExpression::Equal(Box::new($1?), Box::new($3?)) )
      }
    | Comparison 'NEQ' Comparison { 
        Ok(EqualityExpression::NotEqual(Box::new($1?), Box::new($3?)) )
      }
    | Comparison { Ok(EqualityExpression::Comparison($1?)) }
;

Comparison -> Result<ComparisonExpression, ()>:
       Additive 'LT' Additive {
        Ok(ComparisonExpression::LessThan(Box::new($1?), Box::new($3?)) )
       }
     | Additive 'GT' Additive {
        Ok(ComparisonExpression::GreaterThan(Box::new($1?), Box::new($3?)) )
       }
     | Additive 'LTE' Additive {
        Ok(ComparisonExpression::LessThanOrEqual(Box::new($1?), Box::new($3?)) )
       }
     | Additive 'GTE' Additive {
        Ok(ComparisonExpression::GreaterThanOrEqual(Box::new($1?), Box::new($3?)) )
       }
     | Additive { Ok(ComparisonExpression::Additive($1?)) }
;

Additive -> Result<AdditiveExpression, ()>:
    Factor 'ADD' Factor { Ok(AdditiveExpression::Add($1?, $3?)) }
    | Factor 'SUB' Factor { Ok(AdditiveExpression::Subtract($1?, $3?)) }
    | Factor { Ok(AdditiveExpression::Factor($1?)) }
;

Factor -> Result<FactorExpression, ()>:
    Unary 'MUL' Unary { Ok(FactorExpression::Multiply($1?, $3?)) }
    | Unary 'DIV' Unary { Ok(FactorExpression::Divide($1?, $3?)) }
    | Unary 'MOD' Unary { Ok(FactorExpression::Modulo($1?, $3?)) }
    | Unary { Ok(FactorExpression::Unary($1?)) }
;

Unary -> Result<UnaryExpression, ()>:
    'NOT' Primary { Ok(UnaryExpression::Not($2?)) }
    | 'SUB' Primary { Ok(UnaryExpression::Minus($2?)) }
    | Primary { Ok(UnaryExpression::Primary($1?)) }
;

Primary -> Result<PrimaryExpression, ()>:
    'FLOAT' { Ok(PrimaryExpression::Float(
        $lexer.span_str($span).parse::<f64>().unwrap()
    ))}
    | 'TRUE' { Ok(PrimaryExpression::True) }
    | 'FALSE' { Ok(PrimaryExpression::False) }
    | 'SYMBOL'  { Ok(PrimaryExpression::Symbol($lexer.span_str($span).to_string())) }
    | 'ATTRIBUTE'  { Ok(PrimaryExpression::Attribute($lexer.span_str($span).to_string())) }
    | 'STRING_DOUBLE'  { Ok(PrimaryExpression::String(
        $lexer.span_str($span).to_string().trim_matches('"').to_string()
    ))}
    | 'STRING_SINGLE'  { Ok(PrimaryExpression::String(
        $lexer.span_str($span).to_string().trim_matches('"').to_string()
    ))}
    | 'LPAREN' Expr 'RPAREN' { Ok(PrimaryExpression::Grouping(Box::new($2?))) }
    | 'LBRACKET' ExprList 'RBRACKET' { Ok(PrimaryExpression::List($2?)) }
;

ExprList -> Result<Vec<Expression>, ()>:
    /* Empty list */
    { Ok(Vec::new()) }
    | NonEmptyExprList { $1 }
;

NonEmptyExprList -> Result<Vec<Expression>, ()>:
    Expr { Ok(vec![$1?]) }
    | NonEmptyExprList 'COMMA' Expr {
        let mut vec = $1?;
        vec.push($3?);
        Ok(vec)
    }
;

Unmatched -> ():
      "UNMATCHED" { }
    ; 
%%

use crate::ast::*;