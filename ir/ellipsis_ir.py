from dataclasses import dataclass

from common.span import SourceSpan
from ir.constant_ir import ConstantIR
from generated import _pb2


@dataclass
class EllipsisIR(ConstantIR):
    value: ellipsis
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.EllipsisIR()

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.ellipsis.CopyFrom(proto)
        return expr
