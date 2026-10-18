from importlib import import_module


__all__ = ["excepthandler","ExceptHandler"]

for _name in __all__:
    _filename = "_excepthandler" if _name == "excepthandler" else _name.lower()
    _module = import_module(f"{__name__}.{_filename}")
    globals()[_name] = getattr(_module, _name)

globals()["excepthandler"] = getattr(
    import_module(f"{__name__}._excepthandler"), "excepthandler"
)

del _module
del _name
del _filename
