from __future__ import annotations

from dataclasses import dataclass, field

from ir.excepthandler._excepthandler import excepthandler


@dataclass
class ExceptHandler(excepthandler):
    type: expr | None
    name: str | None
    body: list[stmt]
    lineno: int | None = field(default=None, kw_only=True)
    col_offset: int | None = field(default=None, kw_only=True)
    end_lineno: int | None = field(default=None, kw_only=True)
    end_col_offset: int | None = field(default=None, kw_only=True)
