from .expr_ir import ExprIR
from common.span import SourceSpan
from generated import _pb2
from dataclasses import dataclass


@dataclass
class SubscriptIR(ExprIR):
    value: ExprIR
    slice: ExprIR
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.SubscriptIR()

        proto.value.CopyFrom(self.value.to_proto())
        proto.slice.CopyFrom(self.slice.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.subscript.CopyFrom(proto)
        return expr
