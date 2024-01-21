from rust_rule_engine.rust_rule_engine import engine as rust_engine
from rule_engine import engine as py_engine
import timeit

# Create a large dictionary for more realistic performance testing
big_dict = {f'num{i}': i for i in range(1000)}
big_dict["dict2"] = {f'num{i}': i for i in range(1000)}
big_dict["dict3"] = {f'num{i}': i for i in range(1000)}

rule_texts = [
    ('num1 > num2 or num3 < num4', {**big_dict, 'num1': 1, 'num2': 2, 'num3': 3, 'num4': 4}),
    # ('1.2412435 == 2.31213', None),
]

rust_rules = [(rust_engine.Rule(rule_text), rule_input) for rule_text, rule_input in rule_texts]
py_rules = [(py_engine.Rule(rule_text), rule_input) for rule_text, rule_input in rule_texts]

def test_rust_rule_engine():
    for (rule, thing) in rust_rules:
        rule.matches(thing)

def test_py_rule_engine():
    for (rule, thing) in py_rules:
        rule.matches(thing)

def test_eval():
    for (rule, thing) in rule_texts:
        eval(rule, thing)


print("Starting eval profile")
eval_runtime = timeit.timeit(test_eval, number=1_000_000)
print(f"Eval time: {eval_runtime}")
print()
print("Starting Rust profile")
rust_runtime = timeit.timeit(test_rust_rule_engine, number=1_000_000)
print(f"Rust function time: {rust_runtime}")
print()
print("Starting Python profile")
py_runtime = timeit.timeit(test_py_rule_engine, number=1_000_000)
print(f"Python function time: {py_runtime}")
print()
if rust_runtime < py_runtime:
    print("Rust is {:.2f} faster!".format(py_runtime / rust_runtime))
