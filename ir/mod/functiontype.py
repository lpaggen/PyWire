from __future__ import annotations

from dataclasses import dataclass

from ir.mod.mod import mod


@dataclass
class FunctionType(mod):
    argtypes: list[expr]
    returns: expr

