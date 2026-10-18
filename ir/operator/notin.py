from dataclasses import dataclass

from ir.operator.cmpop import cmpop


@dataclass
class NotIn(cmpop):
    pass

