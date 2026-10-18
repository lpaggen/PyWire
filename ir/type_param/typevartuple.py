from __future__ import annotations

from dataclasses import dataclass, field

from ir.type_param.type_param import type_param


@dataclass
class TypeVarTuple(type_param):
    name: str
    default_value: expr | None
    lineno: int | None = field(default=None, kw_only=True)
    col_offset: int | None = field(default=None, kw_only=True)
    end_lineno: int | None = field(default=None, kw_only=True)
    end_col_offset: int | None = field(default=None, kw_only=True)

