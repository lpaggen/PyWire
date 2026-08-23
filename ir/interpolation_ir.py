from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr_ir import ExprIR
from ir.fstring_ir import Conversion


@dataclass
class InterpolationIR(ExprIR):
    value: ExprIR
    source: str
    conversion: Conversion
    format_spec: ExprIR | None
    span: SourceSpan | None

    def to_proto(self):
        return _pb2.ExprIR(
            interpolation=_pb2.InterpolationIR(
                value=self.value.to_proto(),
                source=self.source,
                conversion=int(self.conversion),
                format_spec=(
                    self.format_spec.to_proto()
                    if self.format_spec is not None
                    else None
                ),
                span=self.span.to_proto(),
            )
        )
