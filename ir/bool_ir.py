from dataclasses import dataclass

from ir.constant_ir import ConstantIR

from .ir_node import IRNode
from common.span import SourceSpan
from generated import _pb2


@dataclass
class BooleanIR(ConstantIR):
    value: bool
    span: SourceSpan

    def __repr__(self):
        return "true" if self.value is True else "false"

    def to_proto(self):
        return _pb2.ExprIR(
            constant=_pb2.ConstantIR(
                bool_lit=_pb2.BooleanIR(
                    value=self.value,
                    span=self.span.to_proto(),
                )
            )
        )

