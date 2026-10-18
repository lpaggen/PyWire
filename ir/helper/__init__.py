from importlib import import_module


__all__ = ["comprehension","arguments","arg","keyword","alias","withitem","match_case","type_ignore","TypeIgnore"]

for _name in __all__:
    _module = import_module(f"{__name__}.{_name.lower()}")
    globals()[_name] = getattr(_module, _name)

del _module
del _name
