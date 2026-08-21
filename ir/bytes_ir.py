from common.span import SourceSpan
from ir.expr_ir import ExprIR


class BytesIR(ExprIR):
    def __init__(self, value: ExprIR, span: SourceSpan):
        super().__init__(value=value, span=span)
