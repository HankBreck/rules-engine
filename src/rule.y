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
      Additive { Ok(ComparisonExpression::Additive($1?)) }
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
      'FLOAT' { Ok(PrimaryExpression::Float($span)) }
      | 'TRUE' { Ok(PrimaryExpression::True) }
      | 'FALSE' { Ok(PrimaryExpression::False) }
    ;

Unmatched -> ():
      "UNMATCHED" { }
    ; 
%%

use crate::ast::*;

// use cfgrammar::Span;

// pub enum Expr {
//     EqExpr {
//         lhs: Box<Expr>,
//         rhs: Box<Expr>,
//     },
//     NeqExpr {
//         lhs: Box<Expr>,
//         rhs: Box<Expr>,
//     },
//     FloatExpr {
//         value: Span,
//     },
//     BooleanExpr {
//         value: bool,
//     },
// }
