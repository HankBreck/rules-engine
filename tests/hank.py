import unittest
from rust_rule_engine.rust_rule_engine import engine

class ComparisonExpressionTests(unittest.TestCase):

    def test_GT(self):
        self.assertTrue(engine.Rule("1 > 0").evaluate(None))
        self.assertFalse(engine.Rule("0 > 1").evaluate(None))

    def test_GTE(self):
        self.assertTrue(engine.Rule("1 >= 0").evaluate(None))
        self.assertTrue(engine.Rule("1 >= 1").evaluate(None))
        self.assertFalse(engine.Rule("0 >= 1").evaluate(None))

    def test_LT(self):
        self.assertTrue(engine.Rule("0 < 1").evaluate(None))
        self.assertFalse(engine.Rule("1 < 0").evaluate(None))

    def test_LTE(self):
        self.assertTrue(engine.Rule("0 <= 1").evaluate(None))
        self.assertTrue(engine.Rule("1 <= 1").evaluate(None))
        self.assertFalse(engine.Rule("1 <= 0").evaluate(None))

class SymbolResolutionTests(unittest.TestCase):

    def test_comparison(self):
        rule = engine.Rule("age == 1")
        self.assertTrue(rule.evaluate({"age": 1}))

    def test_comparison_with_string_literal(self):
        rule = engine.Rule("name == \"Hank\"")
        self.assertTrue(rule.evaluate({"name": "Hank"}))

    def test_comparison_with_string_literal_is_case_sensitive(self):
        rule = engine.Rule("name == \"Hank\"")
        self.assertFalse(rule.evaluate({"name": "hank"}))

