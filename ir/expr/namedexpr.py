from __future__ import annotations

from dataclasses import dataclass, field

from ir.expr.expr import expr


@dataclass
class NamedExpr(expr):
    target: expr
    value: expr
    lineno: int | None = field(default=None, kw_only=True)
    col_offset: int | None = field(default=None, kw_only=True)
    end_lineno: int | None = field(default=None, kw_only=True)
    end_col_offset: int | None = field(default=None, kw_only=True)

