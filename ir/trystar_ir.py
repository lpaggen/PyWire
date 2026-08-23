from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.excepthandler_ir import ExceptHandlerIR
from ir.stmt_ir import StmtIR


@dataclass
class TryStarIR(StmtIR):
    body: list[StmtIR]
    handlers: list[ExceptHandlerIR]
    orelse: list[StmtIR]
    finalbody: list[StmtIR]
    span: SourceSpan

    def to_proto(self):
        return _pb2.StmtIR(
            try_star_stmt=_pb2.TryStarIR(
                body=[stmt.to_proto() for stmt in self.body],
                handlers=[handler.to_proto() for handler in self.handlers],
                orelse=[stmt.to_proto() for stmt in self.orelse],
                finalbody=[stmt.to_proto() for stmt in self.finalbody],
                span=self.span.to_proto(),
            )
        )
