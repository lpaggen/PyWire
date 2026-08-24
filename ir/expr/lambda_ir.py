from common.span import SourceSpan
from ir.expr_ir import ExprIR
from ir.arg.param_ir import ArgIR
from dataclasses import dataclass


@dataclass
class LambdaIR(ExprIR):
    args: list[ArgIR]
    body: ExprIR
    scope_id: int
    span: SourceSpan
