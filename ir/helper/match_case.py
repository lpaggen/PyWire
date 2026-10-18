from __future__ import annotations

from dataclasses import dataclass

from ir.ast import AST


@dataclass
class match_case(AST):
    pattern: pattern
    guard: expr | None
    body: list[stmt]

