from common.span import SourceSpan
from ir.expr_ir import ExprIR


class DictIR(ExprIR):
    def __init__(
        self,
        keys: list[ExprIR | None],
        values: list[ExprIR],
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.keys = keys
        self.values = values
