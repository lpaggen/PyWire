from dataclasses import dataclass


@dataclass
class AST:
    def to_proto(self):
        from ir.serialization import to_proto

        return to_proto(self)
