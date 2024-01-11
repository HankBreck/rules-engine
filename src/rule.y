%start Expr
%avoid_insert "FLOAT"
%expect-unused Unmatched "UNMATCHED"
%%
Expr -> Result<Expression, ()>:
      Equality { Ok(Expression::Equality($1?)) }
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
      Factor { Ok(AdditiveExpression::Factor($1?)) }
    ;

Factor -> Result<FactorExpression, ()>:
      Unary { Ok(FactorExpression::Unary($1?)) }
    ;

Unary -> Result<UnaryExpression, ()>:
      Primary { Ok(UnaryExpression::Primary($1?)) }
    ;

Primary -> Result<PrimaryExpression, ()>:
      'FLOAT' { Ok(PrimaryExpression::Float(
            $lexer.span_str($span).parse::<f64>().unwrap()
        )) }
      | 'TRUE' { Ok(PrimaryExpression::True) }
      | 'FALSE' { Ok(PrimaryExpression::False) }
      | 'SYMBOL'  { Ok(PrimaryExpression::Symbol($lexer.span_str($span).to_string())) }
    ;

Unmatched -> ():
      "UNMATCHED" { }
    ; 
%%

use crate::ast::*;