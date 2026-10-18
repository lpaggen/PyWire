from __future__ import annotations

from dataclasses import dataclass, field

from ir.pattern.pattern import pattern


@dataclass
class MatchSingleton(pattern):
    value: object
    lineno: int | None = field(default=None, kw_only=True)
    col_offset: int | None = field(default=None, kw_only=True)
    end_lineno: int | None = field(default=None, kw_only=True)
    end_col_offset: int | None = field(default=None, kw_only=True)

