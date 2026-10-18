from __future__ import annotations

from dataclasses import dataclass, field

from ir.stmt.stmt import stmt


@dataclass
class If(stmt):
    test: expr
    body: list[stmt]
    orelse: list[stmt]
    lineno: int | None = field(default=None, kw_only=True)
    col_offset: int | None = field(default=None, kw_only=True)
    end_lineno: int | None = field(default=None, kw_only=True)
    end_col_offset: int | None = field(default=None, kw_only=True)

