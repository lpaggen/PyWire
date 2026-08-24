from .expr_ir import ExprIR
from common.span import SourceSpan
from generated import _pb2


class SubscriptIR(ExprIR):
    def __init__(self, value: ExprIR, slice: ExprIR, span: SourceSpan):
        super().__init__(span=span, value=None)
        self.value = value
        self.slice = slice
        self.span = span

    def to_proto(self):
        proto = _pb2.SubscriptIR()

        proto.value.CopyFrom(self.value.to_proto())
        proto.slice.CopyFrom(self.slice.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.subscript.CopyFrom(proto)
        return expr
