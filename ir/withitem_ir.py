from dataclasses import dataclass

from generated import _pb2
from ir.expr_ir import ExprIR


@dataclass
class WithItemIR:
    context_expr: ExprIR
    optional_vars: ExprIR | None

    def to_proto(self):
        return _pb2.WithItemIR(
            context_expr=self.context_expr.to_proto(),
            optional_vars=(
                self.optional_vars.to_proto()
                if self.optional_vars is not None
                else None
            ),
        )
