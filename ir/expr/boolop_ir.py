from ir.expr.constant_ir import BooleanIR
from ir.expr.expr_ir import ExprIR
from common.span import SourceSpan
from common.operators import Operator
from generated import _pb2


class BoolOpIR(ExprIR):
    def __init__(
        self, left: BooleanIR, right: BooleanIR, op: Operator, span: SourceSpan = None
    ):
        super().__init__(span=span)
        self.span = span
        self.left = left
        self.right = right
        self.op = op

    def to_proto(self):
        proto = _pb2.BoolOpIR(
            op=self.op,
        )

        proto.values.extend([v.to_proto() for v in self.values])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.boolop.CopyFrom(proto)
        return expr
