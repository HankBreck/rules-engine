# Rule Engine

## Getting started

### Rust

If you do not have it installed already you must install Cargo. Check the [Rust Documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html) for instructions on how to do this

Build the project

```bash
cargo build
```

Optimized build

```bash
cargo build --release
```

### Python

Install the dependencies

```bash
pipenv sync --dev
```

Update from source

```bash
pipenv run refresh
```

Run the tests

```bash
pipenv run test
```

## Performance Analysis

### Profile Rust

We can use Instruments to profile the Rust code to help us find bottlenecks. To do this we need to build the project with debug symbols.

```bash
cargo build --release
```

Then we can use the `cargo-instruments` tool to profile the binary. You can install it with cargo like this:

```bash
cargo install cargo-instruments
```

Finally, we can run the profile. This will open Instruments and start recording.

```bash
cargo instruments -t "CPU Profiler" --bin profiler --release
```

### Rule Engine Comparisons

Currently, we have comparisons for the following rule engine implementations:
- Rust [rule-engine](.)
- Python [rule-engine](https://github.com/zeroSteiner/rule-engine/)
- GoRules [zen](https://github.com/gorules/zen/)
- Python eval

You can run the script to compare the performance of these implementations. You can also modify the rules each engine
will evaluate by modifying the [compare.py](compare.py) script.

```bash
pipenv run compare
```

## Grammar

This is the grammar for the rule engine. It does not reflect the current state of the implementation, but it can help
new engineers understand the language. This grammar is not complete and will be updated as the language evolves.

```ebnf
expression = logical_expression;

logical_expression = equality_expression { ("and" | "or") equality_expression };

equality_expression = comparison_expression { ("!=" | "==") comparison_expression };

comparison_expression = additive_expression { (">" | ">=" | "<" | "<=" ) additive_expression };

additive_expression = factor_expression { ("+" | "-" ) factor_expression };

factor_expression = unary_expression { ("/" | "*" ) unary_expression };

unary_expression = ( "not" | "-" ) unary_expression | primary_expression;

primary_expression = IDENTIFIER | STRING | INTEGER | "true" | "false" | "null"| 
                     list_literal | function_call | "(", expression, ")"

list_literal = '[', expression,  { ',', expression } ']'; 

function_call = IDENTIFIER, '(', argument_list , ')'

argument_list = [ expression , { ',' , expression } ]

type_expression = 'int' | 'string' | 'bool' | 'object' | 'list' [, '<' , type_expression, '>']
```