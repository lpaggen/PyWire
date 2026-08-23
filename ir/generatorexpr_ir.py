from dataclasses import dataclass

from common.span import SourceSpan
from ir.comprehension_ir import CompIR
from ir.expr_ir import ExprIR


@dataclass
class GeneratorExpIR(ExprIR):
    elt: ExprIR
    generators: list[CompIR]
    span: SourceSpan | None
