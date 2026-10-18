from __future__ import annotations

from dataclasses import dataclass, field

from ir.pattern.pattern import pattern


@dataclass
class MatchValue(pattern):
    value: expr
    lineno: int | None = field(default=None, kw_only=True)
    col_offset: int | None = field(default=None, kw_only=True)
    end_lineno: int | None = field(default=None, kw_only=True)
    end_col_offset: int | None = field(default=None, kw_only=True)

