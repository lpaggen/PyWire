from common.span import SourceSpan
from generated import _pb2
from ir.expr_ir import ExprIR
from ir.ir_node import IRNode


class CompIR(IRNode):
    def __init__(
        self,
        target: ExprIR,
        iterable: ExprIR,
        ifs: list[ExprIR],
        is_async: bool,
        span: SourceSpan
    ):
        self.span = span
        self.target = target
        self.iterable = iterable
        self.ifs = ifs
        self.is_async = is_async

    def to_proto(self):
        return _pb2.CompIR(
            target=self.target.to_proto(),
            iterable=self.iterable.to_proto(),
            ifs=[cond.to_proto() for cond in self.ifs],
            is_async=self.is_async,
            span=self.span.to_proto(),
        )


class ListCompIR(ExprIR):
    def __init__(
        self,
        elt: ExprIR,
        generators: list[CompIR],
        span: SourceSpan
    ):
        super().__init__(span=span, value=None)
        self.elt = elt
        self.generators = generators

    def to_proto(self):
        return _pb2.ExprIR(
            list_comp=_pb2.ListCompIR(
                elt=self.elt.to_proto(),
                generators=[gen.to_proto() for gen in self.generators],
                span=self.span.to_proto(),
            )
        )

class SetCompIR(ExprIR):
    def __init__(
        self,
        elt: ExprIR,
        generators: list[CompIR],
        span: SourceSpan,
    ):
        super().__init__(span=span, value=None)
        self.elt = elt
        self.generators = generators

    def to_proto(self):
        return _pb2.ExprIR(
            set_comp=_pb2.SetCompIR(
                elt=self.elt.to_proto(),
                generators=[gen.to_proto() for gen in self.generators],
                span=self.span.to_proto(),
            )
        )

class DictCompIR(ExprIR):
    def __init__(
        self,
        key: ExprIR,
        value: ExprIR,
        generators: list[CompIR],
        span: SourceSpan,
    ):
        super().__init__(span=span, value=None)
        self.key = key
        self.value = value
        self.generators = generators

    def to_proto(self):
        return _pb2.ExprIR(
            dict_comp=_pb2.DictCompIR(
                key=self.key.to_proto(),
                value=self.value.to_proto(),
                generators=[gen.to_proto() for gen in self.generators],
                span=self.span.to_proto(),
            )
        )

class GeneratorExprIR(ExprIR):
    def __init__(
        self,
        elt: ExprIR,
        generators: list[CompIR],
        span: SourceSpan,
    ):
        super().__init__(span=span, value=None)
        self.elt = elt
        self.generators = generators

    def to_proto(self):
        return _pb2.ExprIR(
            generator_expr=_pb2.GeneratorExprIR(
                elt=self.elt.to_proto(),
                generators=[gen.to_proto() for gen in self.generators],
                span=self.span.to_proto(),
            )
        )
