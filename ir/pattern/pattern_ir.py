from common.span import SourceSpan
from .expr_ir import ExprIR
from .ir_node import IRNode
from generated import _pb2


class PatternIR(IRNode):
    def __init__(
        self,
        span: SourceSpan,
    ):
        self.span = span


class ValuePatternIR(PatternIR):
    def __init__(
        self,
        value: ExprIR,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.value = value

    def to_proto(self):
        proto = _pb2.ValuePatternIR()

        proto.value.CopyFrom(self.value.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        pattern = _pb2.PatternIR()
        pattern.value_pattern.CopyFrom(proto)
        return pattern


class SingletonPatternIR(PatternIR):
    def __init__(
        self,
        value: None | bool,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.value = value

    def to_proto(self):
        proto = _pb2.SingletonPatternIR()

        if self.value is None:
            proto.value = _pb2.SINGLETON_NONE
        elif self.value is True:
            proto.value = _pb2.SINGLETON_TRUE
        else:
            proto.value = _pb2.SINGLETON_FALSE

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        pattern = _pb2.PatternIR()
        pattern.singleton_pattern.CopyFrom(proto)
        return pattern


class SequencePatternIR(PatternIR):
    def __init__(
        self,
        patterns: list[PatternIR],
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.patterns = patterns

    def to_proto(self):
        proto = _pb2.SequencePatternIR()

        proto.patterns.extend([
            pattern.to_proto()
            for pattern in self.patterns
        ])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        pattern = _pb2.PatternIR()
        pattern.sequence_pattern.CopyFrom(proto)
        return pattern


class MappingPatternIR(PatternIR):
    def __init__(
        self,
        keys: list[ExprIR],
        patterns: list[PatternIR],
        rest: str | None,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.keys = keys
        self.patterns = patterns
        self.rest = rest

    def to_proto(self):
        proto = _pb2.MappingPatternIR()

        proto.keys.extend([
            key.to_proto()
            for key in self.keys
        ])

        proto.patterns.extend([
            pattern.to_proto()
            for pattern in self.patterns
        ])

        if self.rest is not None:
            proto.rest = self.rest

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        pattern = _pb2.PatternIR()
        pattern.mapping_pattern.CopyFrom(proto)
        return pattern


class ClassPatternIR(PatternIR):
    def __init__(
        self,
        cls: ExprIR,
        patterns: list[PatternIR],
        kwd_attrs: list[str],
        kwd_patterns: list[PatternIR],
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.cls = cls
        self.patterns = patterns
        self.kwd_attrs = kwd_attrs
        self.kwd_patterns = kwd_patterns

    def to_proto(self):
        proto = _pb2.ClassPatternIR()

        proto.cls.CopyFrom(self.cls.to_proto())

        proto.patterns.extend([
            pattern.to_proto()
            for pattern in self.patterns
        ])

        proto.kwd_attrs.extend(self.kwd_attrs)

        proto.kwd_patterns.extend([
            pattern.to_proto()
            for pattern in self.kwd_patterns
        ])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        pattern = _pb2.PatternIR()
        pattern.class_pattern.CopyFrom(proto)
        return pattern


class StarPatternIR(PatternIR):
    def __init__(
        self,
        name: str | None,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.name = name

    def to_proto(self):
        proto = _pb2.StarPatternIR()

        if self.name is not None:
            proto.name = self.name

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        pattern = _pb2.PatternIR()
        pattern.star_pattern.CopyFrom(proto)
        return pattern


class CapturePatternIR(PatternIR):
    def __init__(
        self,
        name: str,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.name = name

    def to_proto(self):
        proto = _pb2.CapturePatternIR()

        proto.name = self.name

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        pattern = _pb2.PatternIR()
        pattern.capture_pattern.CopyFrom(proto)
        return pattern


class WildcardPatternIR(PatternIR):
    def __init__(
        self,
        span: SourceSpan,
    ):
        super().__init__(span=span)

    def to_proto(self):
        proto = _pb2.WildcardPatternIR()

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        pattern = _pb2.PatternIR()
        pattern.wildcard_pattern.CopyFrom(proto)
        return pattern


class AsPatternIR(PatternIR):
    def __init__(
        self,
        pattern: PatternIR,
        name: str,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.pattern = pattern
        self.name = name

    def to_proto(self):
        proto = _pb2.AsPatternIR()

        proto.pattern.CopyFrom(self.pattern.to_proto())
        proto.name = self.name

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        pattern = _pb2.PatternIR()
        pattern.as_pattern.CopyFrom(proto)
        return pattern


class OrPatternIR(PatternIR):
    def __init__(
        self,
        patterns: list[PatternIR],
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.patterns = patterns

    def to_proto(self):
        proto = _pb2.OrPatternIR()

        proto.patterns.extend([
            pattern.to_proto()
            for pattern in self.patterns
        ])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        pattern = _pb2.PatternIR()
        pattern.or_pattern.CopyFrom(proto)
        return pattern
