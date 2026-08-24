from common.span import SourceSpan
from .ir_node import IRNode
from .expr_ir import ExprIR
from generated import _pb2


class TupleIR(ExprIR):
    def __init__(self, elts: tuple[ExprIR], span: SourceSpan = None):
        super().__init__(span=span, value=None)
        self.span = span
        self.elts = elts

    def __repr__(self):
        return "TupleIR<" + str(self.elts) + ">"

    def to_proto(self):
        tuple_proto = _pb2.TupleIR()

        tuple_proto.elts.extend([elt.to_proto() for elt in self.elts])

        if self.span is not None:
            tuple_proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.tuple.CopyFrom(tuple_proto)
        return expr
