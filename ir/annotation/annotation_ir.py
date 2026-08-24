from dataclasses import dataclass
from typing import List
from common.span import SourceSpan
from ir.expr.expr_ir import ExprIR
from generated import _pb2
from ir.ir_node import IRNode


@dataclass
class AnnotationHeadIR(IRNode):
    root: str
    attrs: list[str]
    scope_id: int
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.AnnotationHeadIR(
            root=self.root,
            attrs=self.attrs,
            scope_id=self.scope_id,
        )

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        return proto


@dataclass
class AnnotationIR(IRNode):
    head: AnnotationHeadIR
    args: List[ExprIR]

    def to_proto(self):
        proto = _pb2.AnnotationIR()

        if self.head is not None:
            proto.head.CopyFrom(self.head.to_proto())

        proto.args.extend([arg.to_proto() for arg in self.args])

        return proto
