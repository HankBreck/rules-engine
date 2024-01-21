from rust_rule_engine.rust_rule_engine import engine as rust_engine
from rule_engine import engine as py_engine
import zen
import timeit

zen_engine = zen.ZenEngine()

small_dict = {'num1': 1, 'num2': 2, 'num3': 3, 'num4': 4}

# Create a large dictionary for more realistic performance testing
big_dict = {f'num{i}': i for i in range(100)}
big_dict["dict2"] = {f'num{i}': i for i in range(100)}
big_dict["dict3"] = {f'num{i}': i for i in range(100)}

rule_texts = [
    ('num1 > num2 or num3 < num4', big_dict),
]

rust_rules = [(rust_engine.Rule(rule_text), rule_input) for rule_text, rule_input in rule_texts]
py_rules = [(py_engine.Rule(rule_text), rule_input) for rule_text, rule_input in rule_texts]
with open("../examples/zen-gtorlt.json") as f:
    content = f.read()
zen_rules = [(zen_engine.create_decision(content), rule_texts[0][1])]

def test_rust_rule_engine():
    for (rule, thing) in rust_rules:
        rule.matches(thing)

def test_py_rule_engine():
    for (rule, thing) in py_rules:
        rule.matches(thing)

def test_eval():
    for (rule, thing) in rule_texts:
        eval(rule, thing)

def test_zen_rule_engine():
    for (rule, thing) in zen_rules:
        rule.evaluate(thing)


assert len(zen_rules) == len(py_rules) == len(rust_rules)

ITERATION_COUNT = 100_000
print("Starting Rust profile")
rust_runtime = timeit.timeit(test_rust_rule_engine, number=ITERATION_COUNT)
print("Starting Zen profile")
zen_runtime = timeit.timeit(test_zen_rule_engine, number=ITERATION_COUNT)
print("Starting Python profile")
py_runtime = timeit.timeit(test_py_rule_engine, number=ITERATION_COUNT)
print("Starting eval profile")
eval_runtime = timeit.timeit(test_eval, number=ITERATION_COUNT)
print()

if rust_runtime < py_runtime:
    print("Rust is {:.2f} faster than Python!".format(py_runtime / rust_runtime))
if rust_runtime < zen_runtime:
    print("Rust is {:.2f} faster than Zen!".format(zen_runtime / rust_runtime))
if zen_runtime < py_runtime:
    print("Zen is {:.2f} faster than Python!".format(py_runtime / zen_runtime))
print()

print("Rust runtime: {:.2f}s".format(rust_runtime))
print("Zen runtime: {:.2f}s".format(zen_runtime))
print("Python runtime: {:.2f}s".format(py_runtime))
print("Eval runtime: {:.2f}s".format(eval_runtime))
