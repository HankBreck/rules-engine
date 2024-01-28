import unittest
from rust_rule_engine.rust_rule_engine import engine


class LogicalExpressionTests(unittest.TestCase):

        def test_and(self):
            self.assertTrue(engine.Rule("1 == 1 and 2 == 2").evaluate(None))
            self.assertFalse(engine.Rule("1 == 1 and 2 == 3").evaluate(None))

        def test_and_on_truthy_values(self):
            self.assertTrue(engine.Rule("1 and 2").evaluate(None))
            self.assertFalse(engine.Rule("1 and 0").evaluate(None))
            self.assertFalse(engine.Rule("0 and 1").evaluate(None))
            self.assertFalse(engine.Rule("0 and 0").evaluate(None))

        def test_or(self):
            self.assertTrue(engine.Rule("1 == 1 or 2 == 3").evaluate(None))
            self.assertFalse(engine.Rule("1 == 2 or 2 == 3").evaluate(None))

        def test_or_on_truthy_values(self):
            self.assertTrue(engine.Rule("1 or 2").evaluate(None))
            self.assertTrue(engine.Rule("1 or 0").evaluate(None))
            self.assertTrue(engine.Rule("0 or 1").evaluate(None))
            self.assertFalse(engine.Rule("0 or 0").evaluate(None))
            self.assertTrue(engine.Rule("\"foo\" or false").evaluate(None))
            self.assertTrue(engine.Rule("false or 'foo'").evaluate(None))


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

class AdditiveExpressionTests(unittest.TestCase):

    def test_int_addition(self):
        self.assertEqual(engine.Rule("1 + 1").evaluate(None), 2)
        self.assertEqual(engine.Rule("255 + 1").evaluate(None), 256)

    def test_float_addition(self):
        self.assertEqual(engine.Rule("1.0 + 1.0").evaluate(None), 2.0)

    def test_float_int_addition(self):
        self.assertEqual(engine.Rule("1.5 + 1").evaluate(None), 2.5)
        self.assertEqual(engine.Rule("1 + 2.3").evaluate(None), 3.3)

    def test_int_subtraction(self):
        self.assertEqual(engine.Rule("1 - 1").evaluate(None), 0)
        self.assertEqual(engine.Rule("-1 - 1").evaluate(None), -2)

    def test_float_subtraction(self):
        self.assertEqual(engine.Rule("1.0 - 1.0").evaluate(None), 0.0)
        self.assertEqual(engine.Rule("-10.0 - 1.0").evaluate(None), -11.0)

    def test_float_int_subtraction(self):
        self.assertEqual(engine.Rule("1.5 - 1").evaluate(None), 0.5)
        self.assertEqual(engine.Rule("1 - 2.5").evaluate(None), -1.5)


class FactorExpressionTests(unittest.TestCase):

    def test_int_multiplication(self):
        self.assertEqual(engine.Rule("1 * 1").evaluate(None), 1)
        self.assertEqual(engine.Rule("255 * 1").evaluate(None), 255)
        self.assertEqual(engine.Rule("255 * 0").evaluate(None), 0)

    def test_float_multiplication(self):
        self.assertEqual(engine.Rule("2.0 * 1.5").evaluate(None), 3.0)
        self.assertEqual(engine.Rule("1.0 * 0.0").evaluate(None), 0.0)

    def test_float_int_multiplication(self):
        self.assertEqual(engine.Rule("1.5 * 1").evaluate(None), 1.5)
        self.assertEqual(engine.Rule("1 * 2.5").evaluate(None), 2.5)

    def test_int_division(self):
        self.assertEqual(engine.Rule("1 / 1").evaluate(None), 1)
        self.assertEqual(engine.Rule("255 / 1").evaluate(None), 255)
        self.assertRaises(ValueError, engine.Rule("255 / 0").evaluate, None)

    def test_float_division(self):
        self.assertEqual(engine.Rule("3.0 / 1.5").evaluate(None), 2.0)
        self.assertRaises(ValueError, engine.Rule("1.0 / 0.0").evaluate, None)

    def test_float_int_division(self):
        self.assertEqual(engine.Rule("1.5 / 1").evaluate(None), 1.5)
        self.assertEqual(engine.Rule("1 / 2.5").evaluate(None), 0.4)

    def test_int_modulo(self):
        self.assertEqual(engine.Rule("1 % 1").evaluate(None), 0)
        self.assertEqual(engine.Rule("255 % 2").evaluate(None), 1)
        self.assertRaises(ValueError, engine.Rule("255 % 0").evaluate, None)

    def test_float_modulo(self):
        self.assertEqual(engine.Rule("3.0 % 1.5").evaluate(None), 0.0)
        self.assertAlmostEquals(engine.Rule("1.0 % 0.3").evaluate(None), 0.1)
        self.assertRaises(ValueError, engine.Rule("1.0 % 0.0").evaluate, None)

    def test_float_int_modulo(self):
        self.assertEqual(engine.Rule("1.5 % 1").evaluate(None), 0.5)
        self.assertEqual(engine.Rule("1 % 2.5").evaluate(None), 1.0)


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


class PrimaryExpressionTests(unittest.TestCase):

    def test_grouping_valid(self):
        self.assertTrue(engine.Rule("(1 == 1)").evaluate(None))

    def test_grouping(self):
        self.assertTrue(engine.Rule("false == (1 >= 2)").evaluate(None))

    def test_grouping_with_symbol(self):
        self.assertEqual(engine.Rule("(age + 3) / 2").evaluate({"age": 1}), 2)
