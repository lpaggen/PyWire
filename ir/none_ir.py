from dataclasses import dataclass

from common.span import SourceSpan
from ir.constant_ir import ConstantIR
from .expr_ir import ExprIR
from generated import _pb2


@dataclass
class NoneIR(ConstantIR):
    value: None
    span: SourceSpan | None

    def to_proto(self):
        return _pb2.ExprIR(
            constant=_pb2.ConstantIR(
                none_lit=_pb2.NoneIR(
                    span=self.span.to_proto(),
                )
            )
        )
