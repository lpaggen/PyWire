from dataclasses import dataclass

from ir.expr_ir import ExprIR


@dataclass
class ConstantIR(ExprIR):
    value: ExprIR
    # not including "kind" field because it seems like legacy metadata
