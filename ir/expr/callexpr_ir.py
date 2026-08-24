from .expr_ir import ExprIR
from common.span import SourceSpan
from generated import _pb2


class KeywordArgIR(ExprIR):
    def __init__(self, arg: str | None, value: ExprIR, span: SourceSpan = None):
        super().__init__(span=span, value=value)  # TODO check if value should be IdentifierIR or not
        self.arg = arg
        self.value = value
        self.span = span

    def to_proto(self):
        proto = _pb2.KeywordArgIR()
        if self.arg is not None:
            proto.arg = self.arg
        proto.value.CopyFrom(self.value.to_proto())
        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())
        return proto


class CallExprIR(ExprIR):
    def __init__(
        self, func, args: list[ExprIR], keywords: list[KeywordArgIR], span=None
    ):
        super().__init__(span=span, value=func)
        self.span = span
        self.func = func
        self.args = args
        self.keywords = keywords

    def to_proto(self):
        return _pb2.ExprIR(
            call=_pb2.CallExprIR(
                func=self.func.to_proto(),
                args=[arg.to_proto() for arg in self.args],
                keywords=[keyword.to_proto() for keyword in self.keywords],
            )
        )
