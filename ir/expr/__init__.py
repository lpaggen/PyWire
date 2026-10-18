from importlib import import_module


__all__ = ["expr","BoolOp","NamedExpr","BinOp","UnaryOp","Lambda","IfExp","Dict","Set","ListComp","SetComp","DictComp","GeneratorExp","Await","Yield","YieldFrom","Compare","Call","FormattedValue","Interpolation","JoinedStr","TemplateStr","Constant","Attribute","Subscript","Starred","Name","List","Tuple","Slice"]

for _name in __all__:
    _module = import_module(f"{__name__}.{_name.lower()}")
    globals()[_name] = getattr(_module, _name)

del _module
del _name

