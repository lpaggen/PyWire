from .expr_ir import ExprIR
from common.span import SourceSpan
from .ir_node import IRNode


class PatternIR(IRNode):
    def __init__(
        self,
        span: SourceSpan,
    ):
        self.span = span


class ValuePatternIR(PatternIR):
    """
    case 1
    case "foo"
    case Color.RED
    """
    def __init__(
        self,
        value: ExprIR,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.value = value


class SingletonPatternIR(PatternIR):
    """
    case None
    case True
    case False
    """
    def __init__(
        self,
        value: None | bool,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.value = value


class SequencePatternIR(PatternIR):
    """
    case [a, b]
    case (a, b)
    case [head, *rest]
    """
    def __init__(
        self,
        patterns: list[PatternIR],
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.patterns = patterns


class MappingPatternIR(PatternIR):
    """
    case {"x": x}
    case {"x": x, "y": y}
    case {"x": x, **rest}
    """
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


class ClassPatternIR(PatternIR):
    """
    case Point(x, y)
    case Point(x=x, y=y)
    """
    def __init__(
        self,
        cls: ExprIR,
        positional_patterns: list[PatternIR],
        keyword_names: list[str],
        keyword_patterns: list[PatternIR],
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.cls = cls
        self.positional_patterns = positional_patterns
        self.keyword_names = keyword_names
        self.keyword_patterns = keyword_patterns


class StarPatternIR(PatternIR):
    """
    *rest
    *_

    Only appears inside sequence patterns.
    """
    def __init__(
        self,
        name: str | None,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.name = name


class CapturePatternIR(PatternIR):
    """
    case x
    """
    def __init__(
        self,
        name: str,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.name = name


class WildcardPatternIR(PatternIR):
    """
    case _
    """
    def __init__(
        self,
        span: SourceSpan,
    ):
        super().__init__(span=span)


class AsPatternIR(PatternIR):
    """
    case [x, y] as point
    case (1 | 2) as value
    """
    def __init__(
        self,
        pattern: PatternIR,
        name: str,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.pattern = pattern
        self.name = name


class OrPatternIR(PatternIR):
    """
    case 1 | 2
    case "yes" | "y"
    """
    def __init__(
        self,
        patterns: list[PatternIR],
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.patterns = patterns
