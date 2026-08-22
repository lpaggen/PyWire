from common.span import SourceSpan
from ir.expr_ir import ExprIR
from ir.function_ir import ParamIR


class LambdaIR(ExprIR):
    def __init__(
        self,
        args: list[ParamIR],
        body: ExprIR,
        scope_id: int,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.args = args
        self.body = body
        self.scope_id = scope_id
