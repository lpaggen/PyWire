from __future__ import annotations

from dataclasses import dataclass, field

from ir.expr.expr import expr


@dataclass
class Yield(expr):
    value: expr | None
    lineno: int | None = field(default=None, kw_only=True)
    col_offset: int | None = field(default=None, kw_only=True)
    end_lineno: int | None = field(default=None, kw_only=True)
    end_col_offset: int | None = field(default=None, kw_only=True)

