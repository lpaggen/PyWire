from common.span import SourceSpan
from generated import _pb2
from ir.expr_ir import ExprIR


class ComplexIR(ExprIR):
    def __init__(self, value: ExprIR, span: SourceSpan):
        super().__init__(value=value, span=span)

    def to_proto(self):
        return _pb2.ExprIR(ComplexIR(value=self.value))
