from __future__ import annotations

from dataclasses import dataclass, field

from ir.expr.expr import expr


@dataclass
class DictComp(expr):
    key: expr
    value: expr
    generators: list[comprehension]
    lineno: int | None = field(default=None, kw_only=True)
    col_offset: int | None = field(default=None, kw_only=True)
    end_lineno: int | None = field(default=None, kw_only=True)
    end_col_offset: int | None = field(default=None, kw_only=True)

