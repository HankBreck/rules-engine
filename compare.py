from rust_rule_engine.rust_rule_engine import engine as rust_engine
from rule_engine import engine as py_engine
import timeit

input = '23482.324123512 == true'
rust_rule = rust_engine.Rule(input)
py_rule = py_engine.Rule(input)

def test_rust_rule_engine():
    rust_rule.matches(None)


def test_python_rule_engine():
    py_rule.matches(None)


print("Starting Rust profile")
print(f"Rust function time: {timeit.timeit(test_rust_rule_engine, number=1_000_000)}")
print("Starting Python profile")
print(f"Python function time: {timeit.timeit(test_python_rule_engine, number=1_000_000)}")

