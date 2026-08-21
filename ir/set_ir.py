from common.span import SourceSpan
from generated import _pb2
from ir.expr_ir import ExprIR


class SetIR(ExprIR):
    def __init__(
        self,
        elements: list[ExprIR],
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.elements = elements

    def to_proto(self):
        proto = _pb2.SetIR()

        proto.elements.extend([i.to_proto() for i in self.elements])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.set.CopyFrom(proto)
        return expr
