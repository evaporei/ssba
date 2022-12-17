import ast
import unittest

def evaluate(node):
    # TODO
    return 0

class TestEvaluate(unittest.TestCase):
    def test_evaluate(self):
        test_cases = [
            ('1 + 1', 2),
            ('1 + 2 * 3', 7),
            ('1 + 2 * (3 - 4)', -1),
            ('1 + (2 - 3) * 4 + 5', 2),
        ]
        for s, expected in test_cases:
            expr = ast.parse(s, mode='eval')
            # Uncomment to print AST for expression
            # print(ast.dump(expr, indent=2))
            actual = evaluate(expr)
            if actual != expected:
                raise AssertionError(f'"{s}": expected {expected}, got {actual}')

if __name__ == '__main__':
    unittest.main()
