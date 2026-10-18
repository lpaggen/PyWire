from __future__ import annotations

import ast
from dataclasses import fields

import ir


def from_python_ast(value):
    if isinstance(value, list):
        return [from_python_ast(item) for item in value]
    if not isinstance(value, ast.AST):
        return value

    try:
        node_type = getattr(ir, type(value).__name__)
    except AttributeError as error:
        raise NotImplementedError(
            f"Unsupported Python AST node: {type(value).__name__}"
        ) from error

    field_names = {dataclass_field.name for dataclass_field in fields(node_type)}
    arguments = {
        name: from_python_ast(getattr(value, name))
        for name in value._fields
        if name in field_names
    }

    for name in value._attributes:
        if name in field_names:
            arguments[name] = getattr(value, name, None)

    return node_type(**arguments)
