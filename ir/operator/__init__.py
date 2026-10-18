from importlib import import_module


__all__ = ["boolop","operator","unaryop","cmpop","expr_context","And","Or","Add","Sub","Mult","MatMult","Div","Mod","Pow","LShift","RShift","BitOr","BitXor","BitAnd","FloorDiv","Invert","Not","UAdd","USub","Eq","NotEq","Lt","LtE","Gt","GtE","Is","IsNot","In","NotIn","Load","Store","Del"]

for _name in __all__:
    _module = import_module(f"{__name__}.{_name.lower()}")
    globals()[_name] = getattr(_module, _name)

del _module
del _name

