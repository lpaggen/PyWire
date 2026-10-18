from __future__ import annotations

from dataclasses import dataclass, field

from ir.stmt.stmt import stmt


@dataclass
class Assign(stmt):
    targets: list[expr]
    value: expr
    type_comment: str | None
    lineno: int | None = field(default=None, kw_only=True)
    col_offset: int | None = field(default=None, kw_only=True)
    end_lineno: int | None = field(default=None, kw_only=True)
    end_col_offset: int | None = field(default=None, kw_only=True)

