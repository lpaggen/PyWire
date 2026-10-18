from __future__ import annotations

from dataclasses import dataclass, field

from ir.pattern.pattern import pattern


@dataclass
class MatchAs(pattern):
    pattern: pattern | None
    name: str | None
    lineno: int | None = field(default=None, kw_only=True)
    col_offset: int | None = field(default=None, kw_only=True)
    end_lineno: int | None = field(default=None, kw_only=True)
    end_col_offset: int | None = field(default=None, kw_only=True)

