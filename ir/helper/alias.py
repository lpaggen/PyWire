from __future__ import annotations

from dataclasses import dataclass, field

from ir.ast import AST


@dataclass
class alias(AST):
    name: str
    asname: str | None
    lineno: int | None = field(default=None, kw_only=True)
    col_offset: int | None = field(default=None, kw_only=True)
    end_lineno: int | None = field(default=None, kw_only=True)
    end_col_offset: int | None = field(default=None, kw_only=True)

