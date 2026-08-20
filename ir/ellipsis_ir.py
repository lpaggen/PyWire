from common.span import SourceSpan
from .expr_ir import ExprIR
from generated import _pb2


class EllipsisIR(ExprIR):
    def __init__(self, span: SourceSpan = None):
        super().__init__(span=span, value=Ellipsis)

    def to_proto(self):
        proto = _pb2.EllipsisIR()

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.ellipsis.CopyFrom(proto)
        return expr
