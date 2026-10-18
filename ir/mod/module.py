from __future__ import annotations

from dataclasses import dataclass

from ir.mod.mod import mod


@dataclass
class Module(mod):
    body: list[stmt]
    type_ignores: list[TypeIgnore]

