from common.span import SourceSpan
from generated import _pb2
from ir.expr_ir import ExprIR


class SetIR(ExprIR):
    def __init__(
        self,
        elts: list[ExprIR],
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.elts = elts

    def to_proto(self):
        proto = _pb2.SetIR()

        proto.elts.extend([elt.to_proto() for elt in self.elts])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.set.CopyFrom(proto)
        return expr
