from __future__ import annotations

from dataclasses import dataclass

from ir.ast import AST


@dataclass
class withitem(AST):
    context_expr: expr
    optional_vars: expr | None

