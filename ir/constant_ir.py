from dataclasses import dataclass

from generated import _pb2
from ir.expr_ir import ExprIR


@dataclass
class ConstantIR(ExprIR):
    value: ExprIR
    # not including "kind" field because it seems like legacy metadata

    def to_proto(self):
        return _pb2.ExprIR(
            constant=self.to_proto()
        )
