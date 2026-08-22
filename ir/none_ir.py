from dataclasses import dataclass

from common.span import SourceSpan
from ir.constant_ir import ConstantIR
from .expr_ir import ExprIR
from generated import _pb2


@dataclass
class NoneIR(ConstantIR):
    value: None
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.NoneIR()
        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.none_lit.CopyFrom(proto)
        return expr
