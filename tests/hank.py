import unittest
from rust_rule_engine.rust_rule_engine import engine

class LogicalExpressionTests(unittest.TestCase):

        def test_and(self):
            self.assertTrue(engine.Rule("1 == 1 and 2 == 2").evaluate(None))
            self.assertFalse(engine.Rule("1 == 1 and 2 == 3").evaluate(None))

        def test_and_on_non_booleans(self):
            invalid_rules = [
                "\"foo\" and \"bar\"",
                "1 and 2",
                "1.2345 and 2.3456",
                "true and \"false\""
            ]
            for rule in invalid_rules:
                self.assertRaises(
                    ValueError,
                    engine.Rule(rule).evaluate,
                    None
                )

        def test_or(self):
            self.assertTrue(engine.Rule("1 == 1 or 2 == 3").evaluate(None))
            self.assertFalse(engine.Rule("1 == 2 or 2 == 3").evaluate(None))

        def test_or_on_non_booleans(self):
            invalid_rules = [
                "\"foo\" or \"bar\"",
                "1 or 2",
                "1.2345 or 2.3456",
                "true or \"false\""
            ]
            for rule in invalid_rules:
                self.assertRaises(
                    ValueError,
                    engine.Rule(rule).evaluate,
                    None
                )

        def test_or_on_strings(self):
            self.assertRaises(
                ValueError,
                engine.Rule("\"foo\" or \"bar\"").evaluate,
                None
            )

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

