from __future__ import annotations

from dataclasses import dataclass

from ir.ast import AST


@dataclass
class comprehension(AST):
    target: expr
    iter: expr
    ifs: list[expr]
    is_async: int

