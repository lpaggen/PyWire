from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.constant_ir import ConstantIR
from ir.expr_ir import ExprIR


@dataclass
class ComplexIR(ConstantIR):
    value: complex
    span: SourceSpan

    def to_proto(self):
        return _pb2.ExprIR(ComplexIR(value=self.value))
