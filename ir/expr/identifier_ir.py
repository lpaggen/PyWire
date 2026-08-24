from common.span import SourceSpan
from generated import _pb2
from .expr_ir import ExprIR


class IdentifierIR(ExprIR):
    def __init__(self, id: str, use_scope_id: int, span: SourceSpan):
        super().__init__(value=id, span=span)
        self.id = id
        self.use_scope_id = use_scope_id
        self.span = span

    def __repr__(self):
        return self.id

    def to_proto(self):
        proto = _pb2.IdentifierIR(
            id=self.id,
            use_scope_id=self.use_scope_id,
        )

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.identifier.CopyFrom(proto)
        return expr
