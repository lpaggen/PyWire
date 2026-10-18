from __future__ import annotations

from dataclasses import fields

from generated import _pb2


_MOD_FIELDS = {
    "Module": "module",
    "Interactive": "interactive",
    "Expression": "expression",
    "FunctionType": "function_type",
    "Suite": "suite",
}

_STMT_FIELDS = {
    "FunctionDef": "function_def",
    "AsyncFunctionDef": "async_function_def",
    "ClassDef": "class_def",
    "Return": "return_stmt",
    "Delete": "delete_stmt",
    "Assign": "assign",
    "TypeAlias": "type_alias",
    "AugAssign": "aug_assign",
    "AnnAssign": "ann_assign",
    "For": "for_stmt",
    "AsyncFor": "async_for",
    "While": "while_stmt",
    "If": "if_stmt",
    "With": "with_stmt",
    "AsyncWith": "async_with",
    "Match": "match_stmt",
    "Raise": "raise_stmt",
    "Try": "try_stmt",
    "TryStar": "try_star",
    "Assert": "assert_stmt",
    "Import": "import_stmt",
    "ImportFrom": "import_from",
    "Global": "global_stmt",
    "Nonlocal": "nonlocal_stmt",
    "Expr": "expr_stmt",
    "Pass": "pass_stmt",
    "Break": "break_stmt",
    "Continue": "continue_stmt",
}

_EXPR_FIELDS = {
    "BoolOp": "bool_op",
    "NamedExpr": "named_expr",
    "BinOp": "bin_op",
    "UnaryOp": "unary_op",
    "Lambda": "lambda",
    "IfExp": "if_exp",
    "Dict": "dict",
    "Set": "set",
    "ListComp": "list_comp",
    "SetComp": "set_comp",
    "DictComp": "dict_comp",
    "GeneratorExp": "generator_exp",
    "Await": "await_expr",
    "Yield": "yield_expr",
    "YieldFrom": "yield_from",
    "Compare": "compare",
    "Call": "call",
    "FormattedValue": "formatted_value",
    "Interpolation": "interpolation",
    "JoinedStr": "joined_str",
    "TemplateStr": "template_str",
    "Constant": "constant",
    "Attribute": "attribute",
    "Subscript": "subscript",
    "Starred": "starred",
    "Name": "name",
    "List": "list",
    "Tuple": "tuple",
    "Slice": "slice",
}

_PATTERN_FIELDS = {
    "MatchValue": "match_value",
    "MatchSingleton": "match_singleton",
    "MatchSequence": "match_sequence",
    "MatchMapping": "match_mapping",
    "MatchClass": "match_class",
    "MatchStar": "match_star",
    "MatchAs": "match_as",
    "MatchOr": "match_or",
}

_TYPE_PARAM_FIELDS = {
    "TypeVar": "type_var",
    "ParamSpec": "param_spec",
    "TypeVarTuple": "type_var_tuple",
}

_OPERATOR_WRAPPERS = {
    "boolop": (_pb2.BoolOperator, {"And": "and_op", "Or": "or_op"}),
    "operator": (
        _pb2.BinaryOperator,
        {
            "Add": "add",
            "Sub": "sub",
            "Mult": "mult",
            "MatMult": "mat_mult",
            "Div": "div",
            "Mod": "mod",
            "Pow": "pow",
            "LShift": "l_shift",
            "RShift": "r_shift",
            "BitOr": "bit_or",
            "BitXor": "bit_xor",
            "BitAnd": "bit_and",
            "FloorDiv": "floor_div",
        },
    ),
    "unaryop": (
        _pb2.UnaryOperator,
        {"Invert": "invert", "Not": "not_op", "UAdd": "u_add", "USub": "u_sub"},
    ),
    "cmpop": (
        _pb2.ComparisonOperator,
        {
            "Eq": "eq",
            "NotEq": "not_eq",
            "Lt": "lt",
            "LtE": "lt_e",
            "Gt": "gt",
            "GtE": "gt_e",
            "Is": "is_op",
            "IsNot": "is_not",
            "In": "in_op",
            "NotIn": "not_in",
        },
    ),
    "expr_context": (
        _pb2.ExprContext,
        {"Load": "load", "Store": "store", "Del": "del"},
    ),
}

_HELPER_MESSAGES = {
    "comprehension": _pb2.Comprehension,
    "arguments": _pb2.Arguments,
    "arg": _pb2.Arg,
    "keyword": _pb2.Keyword,
    "alias": _pb2.Alias,
    "withitem": _pb2.WithItem,
    "match_case": _pb2.MatchCase,
    "TypeIgnore": _pb2.TypeIgnore,
}


def _constant_value(value):
    if value is None:
        return _pb2.ConstantValue(none=_pb2.NoneValue())
    if value is Ellipsis:
        return _pb2.ConstantValue(ellipsis=_pb2.EllipsisValue())
    if isinstance(value, bool):
        return _pb2.ConstantValue(boolean=value)
    if isinstance(value, int):
        return _pb2.ConstantValue(integer=str(value))
    if isinstance(value, float):
        return _pb2.ConstantValue(float=value)
    if isinstance(value, complex):
        return _pb2.ConstantValue(
            complex=_pb2.Complex(real=value.real, imag=value.imag)
        )
    if isinstance(value, str):
        return _pb2.ConstantValue(string=value)
    if isinstance(value, bytes):
        return _pb2.ConstantValue(bytes=value)
    raise TypeError(f"Unsupported Constant value: {value!r}")


def _base_name(node) -> str:
    return type(node).__mro__[1].__name__


def _serialize_value(value):
    if hasattr(value, "to_proto"):
        return value.to_proto()
    return value


def _message_kwargs(node):
    result = {}
    for dataclass_field in fields(node):
        name = dataclass_field.name
        value = getattr(node, name)

        if type(node).__name__ in {"Constant", "MatchSingleton"} and name == "value":
            result[name] = _constant_value(value)
            continue

        if value is None:
            continue

        if type(node).__name__ == "Dict" and name == "keys":
            result[name] = [
                _pb2.OptionalExpr(value=key.to_proto()) if key is not None else _pb2.OptionalExpr()
                for key in value
            ]
        elif type(node).__name__ == "arguments" and name == "kw_defaults":
            result[name] = [
                _pb2.OptionalExpr(value=default.to_proto())
                if default is not None
                else _pb2.OptionalExpr()
                for default in value
            ]
        elif isinstance(value, list):
            result[name] = [_serialize_value(item) for item in value]
        else:
            result[name] = _serialize_value(value)
    return result


def to_proto(node):
    name = type(node).__name__
    base_name = _base_name(node)

    if base_name in _OPERATOR_WRAPPERS:
        wrapper_type, field_names = _OPERATOR_WRAPPERS[base_name]
        concrete = getattr(_pb2, name)()
        return wrapper_type(**{field_names[name]: concrete})

    if name in _HELPER_MESSAGES:
        return _HELPER_MESSAGES[name](**_message_kwargs(node))

    concrete_type = getattr(_pb2, name)
    concrete = concrete_type(**_message_kwargs(node))

    if base_name == "mod":
        return _pb2.ModNode(**{_MOD_FIELDS[name]: concrete})
    if base_name == "stmt":
        return _pb2.Stmt(**{_STMT_FIELDS[name]: concrete})
    if base_name == "expr":
        return _pb2.ExprNode(**{_EXPR_FIELDS[name]: concrete})
    if base_name == "pattern":
        return _pb2.Pattern(**{_PATTERN_FIELDS[name]: concrete})
    if base_name == "type_param":
        return _pb2.TypeParam(**{_TYPE_PARAM_FIELDS[name]: concrete})
    if base_name == "excepthandler":
        return _pb2.ExceptHandlerNode(except_handler=concrete)

    raise TypeError(f"Unsupported AST node: {name}")
