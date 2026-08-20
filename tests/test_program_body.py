import ast
import unittest

from frontend.semantic_visitor import SemanticBuilder
from ir.binding_ir import BindingIR
from ir.function_ir import FunctionIR, ReturnIR
from ir.if_ir import IfIR
from ir.import_ir import ImportIR
from ir.match_ir import MatchIR


def build(source: str):
    return SemanticBuilder("test_module", "test_module.py").build(ast.parse(source))


class ProgramBodyTests(unittest.TestCase):
    def test_module_body_preserves_source_order(self):
        program = build(
            """
import package
x = 0
if condition:
    x = 1
else:
    x = 2.0
"""
        )

        self.assertEqual(
            [type(statement) for statement in program.body],
            [ImportIR, BindingIR, IfIR],
        )
        self.assertIsInstance(program.body[2].body[0], BindingIR)
        self.assertIsInstance(program.body[2].orelse[0], BindingIR)

    def test_function_body_retains_bindings_and_returns(self):
        program = build(
            """
def choose(condition):
    if condition:
        result = 1
    else:
        result = 2.0
    return result
"""
        )

        function = program.body[0]
        self.assertIsInstance(function, FunctionIR)
        self.assertIsInstance(function.body[0], IfIR)
        self.assertIsInstance(function.body[0].body[0], BindingIR)
        self.assertIsInstance(function.body[0].orelse[0], BindingIR)
        self.assertIsInstance(function.body[1], ReturnIR)

    def test_statement_tree_serializes_to_protobuf(self):
        program = build(
            """
if condition:
    import package

    def nested():
        value = 1
        return value
"""
        )

        proto = program.to_proto()
        self.assertEqual(proto.body[0].WhichOneof("kind"), "if_stmt")

        branch = proto.body[0].if_stmt.body
        self.assertEqual(
            [statement.WhichOneof("kind") for statement in branch],
            ["import_stmt", "function"],
        )
        self.assertEqual(
            [statement.WhichOneof("kind") for statement in branch[1].function.body],
            ["binding", "return_stmt"],
        )

    def test_chained_assignments_are_flat_statements(self):
        program = build("a = b = 1")

        self.assertEqual(len(program.body), 2)
        self.assertTrue(
            all(isinstance(statement, BindingIR) for statement in program.body)
        )
        self.assertEqual(
            [statement.WhichOneof("kind") for statement in program.to_proto().body],
            ["binding", "binding"],
        )

    def test_match_with_ellipsis_body_serializes_to_protobuf(self):
        program = build(
            """
match value:
    case [item] as matched:
        ...
    case _:
        ...
"""
        )

        self.assertIsInstance(program.body[0], MatchIR)

        match_proto = program.to_proto().body[0]
        self.assertEqual(match_proto.WhichOneof("kind"), "match")
        self.assertEqual(
            match_proto.match.cases[0].body[0].WhichOneof("kind"),
            "expr_stmt",
        )
        self.assertEqual(
            match_proto.match.cases[0].body[0].expr_stmt.value.WhichOneof("kind"),
            "ellipsis",
        )


if __name__ == "__main__":
    unittest.main()
