from common.span import SourceSpan
from ir.constant_ir import ConstantIR
from ir.expr_ir import ExprIR


class BytesIR(ConstantIR):
    value: bytes
    span: SourceSpan
