from dataclasses import dataclass

from ir.operator.expr_context import expr_context


@dataclass
class Del(expr_context):
    pass

