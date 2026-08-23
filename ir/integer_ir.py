from dataclasses import dataclass

from common.span import SourceSpan
from ir.constant_ir import ConstantIR
from .expr_ir import ExprIR
from generated import _pb2


@dataclass
class IntegerIR(ConstantIR):
    value: int
    span: SourceSpan

    def __repr__(self):
        return str(self.value)

    def to_proto(self):
        return _pb2.ExprIR(
            constant=_pb2.ConstantIR(
                integer_lit=_pb2.IntegerIR(
                    value=self.value,
                    span=self.span.to_proto(),
                )
            )
        )
