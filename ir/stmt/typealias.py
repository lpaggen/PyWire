from __future__ import annotations

from dataclasses import dataclass, field

from ir.stmt.stmt import stmt


@dataclass
class TypeAlias(stmt):
    name: expr
    type_params: list[type_param]
    value: expr
    lineno: int | None = field(default=None, kw_only=True)
    col_offset: int | None = field(default=None, kw_only=True)
    end_lineno: int | None = field(default=None, kw_only=True)
    end_col_offset: int | None = field(default=None, kw_only=True)

