import ast
import inspect
import re
import unittest
from dataclasses import fields, is_dataclass
from pathlib import Path

import ir
from frontend.converter import from_python_ast
from generated import _pb2


_LOCATION_FIELDS = ("lineno", "col_offset", "end_lineno", "end_col_offset")
_FAMILIES = (
    ast.mod,
    ast.stmt,
    ast.expr,
    ast.excepthandler,
    ast.pattern,
    ast.type_param,
    ast.boolop,
    ast.operator,
    ast.unaryop,
    ast.cmpop,
    ast.expr_context,
    ast.type_ignore,
)
_HELPERS = (
    ast.comprehension,
    ast.arguments,
    ast.arg,
    ast.keyword,
    ast.alias,
    ast.withitem,
    ast.match_case,
)
_PROTO_NAMES = {
    "comprehension": "Comprehension",
    "arguments": "Arguments",
    "arg": "Arg",
    "keyword": "Keyword",
    "alias": "Alias",
    "withitem": "WithItem",
    "match_case": "MatchCase",
}
_UNDOCUMENTED_LEGACY_CLASSES = {"AugLoad", "AugStore", "Param"}
_ROOT = Path(__file__).resolve().parents[1]


def concrete_python_ast_classes():
    classes = []
    for name in dir(ast):
        value = getattr(ast, name)
        if not isinstance(value, type) or value.__module__ != "ast":
            continue
        if value.__name__ in _UNDOCUMENTED_LEGACY_CLASSES:
            continue
        if any(issubclass(value, family) for family in _FAMILIES) and value not in _FAMILIES:
            classes.append(value)
    return classes


class AstParityTests(unittest.TestCase):
    def test_packaged_rust_schema_matches_canonical_schema(self):
        self.assertEqual(
            (_ROOT / "rust/proto/python_ast.proto").read_bytes(),
            (_ROOT / "proto/.proto").read_bytes(),
        )

    def test_public_ir_classes_are_only_official_ast_classes(self):
        official = {
            ast.AST.__name__,
            *(family.__name__ for family in _FAMILIES),
            *(helper.__name__ for helper in _HELPERS),
            *(node.__name__ for node in concrete_python_ast_classes()),
        }
        actual = {
            name
            for name, value in inspect.getmembers(ir, inspect.isclass)
            if value.__module__.startswith("ir.")
        }

        self.assertEqual(actual, official)
        for name in official:
            with self.subTest(dataclass=name):
                self.assertTrue(is_dataclass(getattr(ir, name)))

    def test_every_official_node_has_exact_dataclass_fields(self):
        for python_type in [*concrete_python_ast_classes(), *_HELPERS]:
            with self.subTest(node=python_type.__name__):
                ir_type = getattr(ir, python_type.__name__)
                self.assertTrue(is_dataclass(ir_type))
                self.assertEqual(
                    ir_type.__module__.rsplit(".", 1)[-1],
                    python_type.__name__.lower(),
                )
                actual = tuple(field.name for field in fields(ir_type))
                self.assertEqual(actual[: len(python_type._fields)], python_type._fields)
                self.assertEqual(
                    actual[len(python_type._fields) :],
                    tuple(
                        name
                        for name in _LOCATION_FIELDS
                        if name in python_type._attributes
                    ),
                )

    def test_protobuf_concrete_fields_match_python_fields(self):
        for python_type in [*concrete_python_ast_classes(), *_HELPERS]:
            with self.subTest(node=python_type.__name__):
                proto_name = _PROTO_NAMES.get(python_type.__name__, python_type.__name__)
                descriptor = getattr(_pb2, proto_name).DESCRIPTOR
                proto_fields = tuple(field.name for field in descriptor.fields)
                expected = python_type._fields + tuple(
                    name for name in _LOCATION_FIELDS if name in python_type._attributes
                )
                self.assertEqual(proto_fields, expected)

    def test_rust_concrete_fields_match_python_fields(self):
        helpers = {helper.__name__: helper for helper in _HELPERS}
        python_types = {
            node.__name__: node for node in concrete_python_ast_classes()
        } | helpers

        for name, python_type in python_types.items():
            rust_name = _PROTO_NAMES.get(name, name)
            if issubclass(python_type, ast.expr):
                group = "expr"
            elif issubclass(python_type, ast.stmt):
                group = "stmt"
            elif issubclass(python_type, ast.pattern):
                group = "pattern"
            elif issubclass(python_type, ast.type_param):
                group = "type_param"
            elif issubclass(python_type, ast.excepthandler):
                group = "excepthandler"
            elif issubclass(python_type, ast.mod) or issubclass(
                python_type, ast.type_ignore
            ):
                group = "mod_"
            elif any(
                issubclass(python_type, family)
                for family in (
                    ast.boolop,
                    ast.operator,
                    ast.unaryop,
                    ast.cmpop,
                    ast.expr_context,
                )
            ):
                group = "operator"
            else:
                group = "helper"

            filename = (
                "mod_"
                if group == "operator" and rust_name == "Mod"
                else rust_name.lower()
            )
            source = (_ROOT / "rust" / "src" / "ast" / group / f"{filename}.rs").read_text()
            declaration = re.search(
                rf"pub struct {rust_name} \{{(?P<body>.*?)\}}",
                source,
                re.DOTALL,
            )
            self.assertIsNotNone(declaration, name)
            actual = tuple(
                field_name.removeprefix("r#")
                for field_name in re.findall(
                    r"^\s*pub\s+(r#\w+|\w+):",
                    declaration.group("body"),
                    re.MULTILINE,
                )
            )
            expected = python_type._fields + tuple(
                attribute
                for attribute in _LOCATION_FIELDS
                if attribute in python_type._attributes
            )
            self.assertEqual(actual, expected, name)

    def test_every_concrete_node_serializes(self):
        placeholders = {
            "expr": lambda: ir.Constant(None, None),
            "stmt": ir.Pass,
            "pattern": lambda: ir.MatchAs(None, None),
            "type_param": lambda: ir.ParamSpec("", None),
            "excepthandler": lambda: ir.ExceptHandler(None, None, []),
            "boolop": ir.And,
            "operator": ir.Add,
            "unaryop": ir.Invert,
            "cmpop": ir.Eq,
            "expr_context": ir.Load,
            "arguments": lambda: ir.arguments([], [], None, [], [], None, []),
            "arg": lambda: ir.arg("", None, None),
            "keyword": lambda: ir.keyword(None, ir.Constant(None, None)),
            "alias": lambda: ir.alias("", None),
            "withitem": lambda: ir.withitem(ir.Constant(None, None), None),
            "match_case": lambda: ir.match_case(ir.MatchAs(None, None), None, []),
            "comprehension": lambda: ir.comprehension(
                ir.Name("", ir.Load()), ir.Name("", ir.Load()), [], 0
            ),
            "str": str,
            "int": int,
            "object": lambda: None,
        }

        for python_type in [*concrete_python_ast_classes(), *_HELPERS]:
            with self.subTest(node=python_type.__name__):
                ir_type = getattr(ir, python_type.__name__)
                arguments = []
                for dataclass_field in fields(ir_type)[: len(python_type._fields)]:
                    type_name = str(dataclass_field.type)
                    if type_name.startswith("list["):
                        arguments.append([])
                    elif "| None" in type_name:
                        arguments.append(None)
                    else:
                        arguments.append(placeholders[type_name]())

                ir_type(*arguments).to_proto()

    def test_real_python_tree_serializes_with_contexts(self):
        source = """
class Box[T]:
    def method(self, value: T = None) -> T:
        self.value = value
        if value is not None:
            return self.value
        return None

del box.value
result = [item.value for item in boxes if item.value]
"""
        module = from_python_ast(ast.parse(source, type_comments=True))
        proto = module.to_proto()
        encoded = proto.SerializeToString()
        decoded = _pb2.ModNode.FromString(encoded)

        self.assertEqual(decoded.WhichOneof("kind"), "module")
        assignment = decoded.module.body[0].class_def.body[0].function_def.body[0].assign
        self.assertEqual(
            assignment.targets[0].attribute.ctx.WhichOneof("kind"), "store"
        )
        self.assertEqual(
            assignment.value.name.ctx.WhichOneof("kind"), "load"
        )

    def test_none_constants_preserve_required_wire_presence(self):
        constant = ir.Constant(None, None).to_proto().constant
        singleton = ir.MatchSingleton(None).to_proto().match_singleton

        self.assertTrue(constant.HasField("value"))
        self.assertEqual(constant.value.WhichOneof("kind"), "none")
        self.assertTrue(singleton.HasField("value"))
        self.assertEqual(singleton.value.WhichOneof("kind"), "none")


if __name__ == "__main__":
    unittest.main()
