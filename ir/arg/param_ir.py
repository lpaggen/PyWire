from enum import Enum

from common.span import SourceSpan
from generated import _pb2
from ir.annotation_ir import AnnotationIR
from ir.ir_node import IRNode


class ParamKind(Enum):
    POSITIONAL_ONLY = 1
    POSITIONAL_OR_KEYWORD = 2
    VAR_POSITIONAL = 3
    KEYWORD_ONLY = 4
    VAR_KEYWORD = 5


class ParamIR(IRNode):
    def __init__(
        self,
        symbol_id: int,
        name: str,
        kind: ParamKind,
        annotation: AnnotationIR,
        default,
        span: SourceSpan,
    ):
        self.symbol_id = symbol_id
        self.name = name
        self.kind = kind
        self.annotation = annotation
        self.default = default
        self.span = span

    def to_proto(self):
        proto = _pb2.ParamIR(
            symbol_id=self.symbol_id,
            name=self.name,
            kind=self.kind.value
        )

        if self.annotation is not None:
            proto.annotation.CopyFrom(self.annotation.to_proto())

        if self.default is not None:
            proto.default_value.CopyFrom(self.default.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        return proto
