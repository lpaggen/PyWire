from __future__ import annotations

from dataclasses import dataclass

from ir.ast import AST


@dataclass
class arguments(AST):
    posonlyargs: list[arg]
    args: list[arg]
    vararg: arg | None
    kwonlyargs: list[arg]
    kw_defaults: list[expr | None]
    kwarg: arg | None
    defaults: list[expr]

