from .stmt_ir import StmtIR
from .expr_ir import ExprIR
from common.span import SourceSpan
from .pattern_ir import PatternIR

from typing import List


class MatchIR(StmtIR):
    def __init__(self, subject: ExprIR, cases: List["MatchCaseIR"], span: SourceSpan):
        super().__init__(span=span)
        self.subject = subject
        self.cases = cases


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
