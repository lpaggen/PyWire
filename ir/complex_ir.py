from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.constant_ir import ConstantIR


@dataclass
class ComplexIR(ConstantIR):
    real: float
    imag: float
    span: SourceSpan | None

    def to_proto(self):
        return _pb2.ExprIR(ComplexIR(real=self.real, imag=self.imag))
