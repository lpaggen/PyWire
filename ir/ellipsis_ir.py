from dataclasses import dataclass

from common.span import SourceSpan
from ir.constant_ir import ConstantIR
from generated import _pb2


@dataclass
class EllipsisIR(ConstantIR):
    value: ellipsis
    span: SourceSpan

    def to_proto(self):
        return _pb2.ExprIR(
            constant=_pb2.ConstantIR(
                ellipsis_lit=_pb2.EllipsisIR(
                    span=self.span.to_proto(),
                )
            )
        )
