from common.span import SourceSpan
from ir.arg.param_ir import ParamIR
from ..ir_node import IRNode
from generated import _pb2
from .stmt_ir import stmt_to_proto
from .decl_ir import DeclIR


class FunctionDefIR(DeclIR):
    def __init__(
        self,
        id: int,  # symbol id -> name of function
        symbol_id: int,
        name: str,
        scope_id: int,  # parent scope where function name is bound
        body_scope_id: int,  # function-local scope
        params: list[ParamIR],
        body: list[IRNode],
        returns,
        decorators,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.id = id
        self.symbol_id = symbol_id
        self.name = name
        self.scope_id = scope_id
        self.body_scope_id = body_scope_id
        self.params = params
        self.body = body
        self.returns = returns
        self.decorators = decorators
        self.span = span

    def to_proto(self):
        proto = _pb2.FunctionIR(
            id=self.id,
            symbol_id=self.symbol_id,
            name=self.name,
            scope_id=self.scope_id,
            body_scope_id=self.body_scope_id,
        )

        proto.params.extend([p.to_proto() for p in self.params])
        proto.body.extend([stmt_to_proto(stmt) for stmt in self.body])
        proto.decorators.extend([d.to_proto() for d in self.decorators])

        if self.returns is not None:
            proto.returns.CopyFrom(self.returns.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.DeclIR()
        stmt.function.CopyFrom(proto)
        return stmt

    def to_stmt_proto(self):
        decl = self.to_proto()
        stmt = _pb2.StmtIR()
        stmt.function.CopyFrom(decl.function)
        return stmt
