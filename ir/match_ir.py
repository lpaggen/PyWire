from .stmt_ir import StmtIR
from .expr_ir import ExprIR
from common.span import SourceSpan
from .pattern_ir import PatternIR

from generated import _pb2

from typing import List


class MatchIR(StmtIR):
    def __init__(
        self,
        subject: ExprIR,
        cases: List["MatchCaseIR"],
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.subject = subject
        self.cases = cases

    def to_proto(self):
        proto = _pb2.MatchIR()

        proto.subject.CopyFrom(self.subject.to_proto())

        proto.cases.extend([
            case.to_proto()
            for case in self.cases
        ])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.StmtIR()
        stmt.match.CopyFrom(proto)

        return stmt


class MatchCaseIR(StmtIR):
    def __init__(
        self,
        pattern: PatternIR,
        guard: ExprIR | None,
        body: List[StmtIR],
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.pattern = pattern
        self.guard = guard
        self.body = body

    def to_proto(self):
        proto = _pb2.MatchCaseIR()

        proto.pattern.CopyFrom(self.pattern.to_proto())

        if self.guard is not None:
            proto.guard.CopyFrom(self.guard.to_proto())

        proto.body.extend([
            stmt.to_proto()
            for stmt in self.body
        ])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        return proto
