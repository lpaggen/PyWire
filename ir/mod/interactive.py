from __future__ import annotations

from dataclasses import dataclass

from ir.mod.mod import mod


@dataclass
class Interactive(mod):
    body: list[stmt]

