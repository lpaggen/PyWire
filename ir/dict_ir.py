from common.span import SourceSpan
from ir.expr_ir import ExprIR
from ir.ir_node import IRNode


class DictEntryIR(IRNode):
    def __init__(
        self,
        key: ExprIR | None,
        value: ExprIR,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.key = key
        self.value = value


class DictIR(ExprIR):
    def __init__(
        self,
        entries: list[DictEntryIR],
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.entries = entries
