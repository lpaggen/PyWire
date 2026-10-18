from dataclasses import dataclass

from ir.ast import AST


@dataclass
class type_ignore(AST):
    pass
