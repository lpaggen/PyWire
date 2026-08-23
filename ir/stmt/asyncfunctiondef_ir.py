from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr_ir import ExprIR
from ir.function_ir import ParamIR
from ir.stmt_ir import StmtIR


@dataclass
class AsyncFunctionDefIR(StmtIR):
    name: str
    args: list[ParamIR]
    body: list[StmtIR]
    decorators: list[ExprIR]
    returns: ExprIR | None
    type_comment: str | None
    scope_id: int
    span: SourceSpan

    def to_proto(self):
        return _pb2.StmtIR(
            async_function_def=_pb2.AsyncFunctionDefIR(
                name=self.name,
                args=[arg.to_proto() for arg in self.args],
                body=[stmt.to_proto() for stmt in self.body],
                decorators=[d.to_proto() for d in self.decorators],
                returns=self.returns.to_proto() if self.returns is not None else None,
                type_comment=self.type_comment,
                scope_id=self.scope_id,
                span=self.span.to_proto(),
            )
        )
