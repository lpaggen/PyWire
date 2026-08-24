from .stmt_ir import StmtIR
from common.span import SourceSpan
from .expr_ir import ExprIR
from common.operators import Operator
from generated import _pb2
from dataclasses import dataclass


@dataclass
class UnaryOpIR(ExprIR):
    op: Operator
    operand: ExprIR
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.UnaryOpIR(
            op=self.op.value,
        )

        proto.operand.CopyFrom(self.operand.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.unaryop.CopyFrom(proto)
        return expr
