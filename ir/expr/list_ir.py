from common.span import SourceSpan
from .expr_ir import ExprIR
from generated import _pb2
from dataclasses import dataclass


@dataclass
class ListIR(ExprIR):
    elts: list[ExprIR]
    span: SourceSpan | None = None

    def to_proto(self):
        proto = _pb2.ListIR()

        proto.elts.extend([elt.to_proto() for elt in self.elts])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.list.CopyFrom(proto)
        return expr
