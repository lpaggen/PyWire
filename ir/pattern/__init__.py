from importlib import import_module


__all__ = ["pattern","MatchValue","MatchSingleton","MatchSequence","MatchMapping","MatchClass","MatchStar","MatchAs","MatchOr"]

for _name in __all__:
    _module = import_module(f"{__name__}.{_name.lower()}")
    globals()[_name] = getattr(_module, _name)

del _module
del _name

