from common.span import SourceSpan
from ir.expr_ir import ExprIR


class SetIR(ExprIR):
    def __init__(
        self,
        elements: list[ExprIR],
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.elements = elements
