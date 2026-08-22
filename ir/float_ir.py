from dataclasses import dataclass

from common.span import SourceSpan
from ir.constant_ir import ConstantIR
from .expr_ir import ExprIR
from generated import _pb2


@dataclass
class FloatIR(ConstantIR):
    value: float
    span: SourceSpan

    def __repr__(self):
        return str(self.value)

    def to_proto(self):
        return _pb2.ExprIR(float_lit=_pb2.FloatIR(value=self.value))
