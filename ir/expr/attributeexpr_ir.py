from .expr_ir import ExprIR
from common.span import SourceSpan
from generated import _pb2
from dataclasses import dataclass


@dataclass
class AttributeExprIR(ExprIR):
    value: ExprIR
    attr: str
    span: SourceSpan

    def to_proto(self):
        attr_proto = _pb2.AttributeExprIR(
            attr=self.attr,
        )

        attr_proto.value.CopyFrom(self.value.to_proto())

        if self.span is not None:
            attr_proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.attribute.CopyFrom(attr_proto)
        return expr
