%%
\( "LPAREN"
\) "RPAREN"
\[ "LBRACKET"
\] "RBRACKET"
\{ "LBRACE"
\} "RBRACE"

\, "COMMA"
\? "QMARK"
\: "COLON"

\== "EQ"
\!= "NEQ"
\> "GT"
\>= "GTE"
\< "LT"
\<= "LTE"

\+ "ADD"
\- "SUB"
\* "MUL"
\/ "DIV"
\% "MOD"

true "TRUE"
false "FALSE"

and "AND"
or "OR"
not "NOT"

0(b[01]+|o[0-7]+|x[0-9a-fA-F]+)|[0-9]+(\.[0-9]*)?([eE][+-]?[0-9]+)?|\.[0-9]+([eE][+-]?[0-9]+)? "FLOAT"
[a-zA-Z_][a-zA-Z0-9_]* "SYMBOL"
[a-zA-Z_][a-zA-Z0-9_]*(\.[a-zA-Z_][a-zA-Z0-9_]*)+ "ATTRIBUTE"

\"(\\.|[^\\"])*?\" "STRING_DOUBLE"
\'(\\.|[^\\"])*?\' "STRING_SINGLE"

[ \t\n\r]+ ;
. "UNMATCHED"
