from rust_rule_engine.rust_rule_engine import engine as rust_engine
from rule_engine import engine as py_engine
import timeit

rule_texts = [
    ('23482.324123512 == true', None),
    # ('name == "John" and age > 18', {'name': 'John', 'age': 19}),
    # ('num1 > num2 or num3 < num4', {'num1': 1, 'num2': 2, 'num3': 3, 'num4': 4}),
]
rust_rules = [(rust_engine.Rule(rule_text), rule_input) for rule_text, rule_input in rule_texts]
py_rules = [(py_engine.Rule(rule_text), rule_input) for rule_text, rule_input in rule_texts]

def test_rust_rule_engine():
    for (rule, thing) in rust_rules:
        rule.matches(thing)

def test_py_rule_engine():
    for (rule, thing) in py_rules:
        rule.matches(thing)


print("Starting Rust profile")
print(f"Rust function time: {timeit.timeit(test_rust_rule_engine, number=1_000_000)}")
print("Starting Python profile")
print(f"Python function time: {timeit.timeit(test_py_rule_engine, number=1_000_000)}")

