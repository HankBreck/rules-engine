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

    def test_equality(self):
        self.assertTrue(engine.Rule("age == 1").evaluate({"age": 1}))

    def test_equality_with_string_literal(self):
        self.assertTrue(engine.Rule("name == \"Hank\"").evaluate({"name": "Hank"}))

    def test_equality_with_string_literal_is_case_sensitive(self):
        self.assertFalse(engine.Rule("name == \"Hank\"").evaluate({"name": "hank"}))

    def test_equality_between_float_and_int(self):
        self.assertTrue(engine.Rule("age == 1.0").evaluate({"age": 1}))

    def test_comparison_between_float_and_int(self):
        self.assertTrue(engine.Rule("age > 0").evaluate({"age": 1.0}))
        self.assertTrue(engine.Rule("1 >= age").evaluate({"age": 1.0}))
        self.assertFalse(engine.Rule("age < 0").evaluate({"age": 0.0}))
        self.assertFalse(engine.Rule("0 <= age").evaluate({"age": -0.001}))

    def test_symbol_resolution_fails_on_nonexistent_symbol(self):
        self.assertRaises(ValueError, engine.Rule("1 == age").evaluate, {"name": "Hank"})

    def test_parsing_fails_on_invalid_symbol_name(self):
        self.assertRaises(ValueError, engine.Rule, ".identifier == 1")

class AttributeResolutionTests(unittest.TestCase):

    def test_attribute_resolution(self):
        self.assertTrue(engine.Rule("person.age == 1").evaluate({"person": {"age": 1}}))

    def test_attribute_resolution_with_string_literal(self):
        self.assertTrue(engine.Rule("person.name == \"Hank\"").evaluate({"person": {"name": "Hank"}}))

    def test_attribute_resolution_with_string_literal_is_case_sensitive(self):
        self.assertFalse(engine.Rule("person.name == \"Hank\"").evaluate({"person": {"name": "hank"}}))

    def test_attribute_resolution_deeply_nested(self):
        self.assertTrue(engine.Rule("l1.l2.l3.l4.l5.l6 == 1").evaluate({"l1": {"l2": {"l3": {"l4": {"l5": {"l6": 1}}}}}}))

    def test_attribute_resolution_fails_on_nonexistent_attr(self):
        self.assertRaises(ValueError, engine.Rule("1 == person.age").evaluate, {"person": {"name": "Hank"}})

    def test_parsing_fails_on_invalid_attr_name(self):
        self.assertRaises(ValueError, engine.Rule, "person.1abc == 1")


