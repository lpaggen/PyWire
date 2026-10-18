from __future__ import annotations

from dataclasses import dataclass

from ir.helper.type_ignore import type_ignore


@dataclass
class TypeIgnore(type_ignore):
    lineno: int
    tag: str
