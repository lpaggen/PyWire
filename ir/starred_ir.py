from common.span import SourceSpan
from generated import _pb2
from ir.expr_ir import ExprIR


class StarredIR(ExprIR):
    def __init__(
        self,
        value: ExprIR,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.value = value

    def to_proto(self):
        proto = _pb2.StarredIR()
        proto.value.CopyFrom(self.value.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.starred.CopyFrom(proto)  # use actual oneof field name
        return expr
