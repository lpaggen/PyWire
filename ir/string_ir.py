from dataclasses import dataclass

from ir.constant_ir import ConstantIR

from .expr_ir import ExprIR
from common.span import SourceSpan
from generated import _pb2


@dataclass
class StringIR(ConstantIR):
    value: str
    span: SourceSpan

    def __repr__(self):
        return self.value

    def to_proto(self):
        return _pb2.ExprIR(
            constant=_pb2.ConstantIR(
                string_lit=_pb2.StringIR(
                    value=self.value,
                    span=self.span.to_proto(),
                )
            )
        )
