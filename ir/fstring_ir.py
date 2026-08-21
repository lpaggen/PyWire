from ir.expr_ir import ExprIR


class FStringIR(ExprIR):
    values: list[FStringPartIR]


class FStringPartIR(FStringIR):
    ...