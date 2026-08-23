# Python 3.14 AST Mapping

This document maps Python 3.14 AST nodes to the portable IR, Protocol Buffer representation, and Rust IR types used by this project.

The general pipeline is:

```text
CPython AST
    ↓
Python IR
    ↓
Protocol Buffers
    ↓
Rust IR
```

## Expressions

| Python AST | Python IR | Protobuf | Rust IR |
|---|---|---|---|
| `ast.Constant(bool)` | `BooleanIR` | `ExprIR.constant.bool_lit` | `ExprIR::Constant(ConstantIR::BooleanLit(...))` |
| `ast.Constant(int)` | `IntegerIR` | `ExprIR.constant.integer_lit` | `ExprIR::Constant(ConstantIR::IntegerLit(...))` |
| `ast.Constant(float)` | `FloatIR` | `ExprIR.constant.float_lit` | `ExprIR::Constant(ConstantIR::FloatLit(...))` |
| `ast.Constant(str)` | `StringIR` | `ExprIR.constant.string_lit` | `ExprIR::Constant(ConstantIR::StringLit(...))` |
| `ast.Constant(bytes)` | `BytesIR` | `ExprIR.constant.bytes_lit` | `ExprIR::Constant(ConstantIR::BytesLit(...))` |
| `ast.Constant(complex)` | `ComplexIR` | `ExprIR.constant.complex_lit` | `ExprIR::Constant(ConstantIR::ComplexLit(...))` |
| `ast.Constant(None)` | `NoneIR` | `ExprIR.constant.none_lit` | `ExprIR::Constant(ConstantIR::NoneLit(...))` |
| `ast.Constant(Ellipsis)` | `EllipsisIR` | `ExprIR.constant.ellipsis_lit` | `ExprIR::Constant(ConstantIR::EllipsisLit(...))` |
| `ast.Name` | `IdentifierIR` | `ExprIR.identifier` | `ExprIR::IdentifierExpr(...)` |
| `ast.List` | `ListIR` | `ExprIR.list` | `ExprIR::ListExpr(...)` |
| `ast.Tuple` | `TupleIR` | `ExprIR.tuple` | `ExprIR::TupleExpr(...)` |
| `ast.Set` | `SetIR` | `ExprIR.set` | `ExprIR::SetExpr(...)` |
| `ast.Dict` | `DictIR` | `ExprIR.dict` | `ExprIR::DictExpr(...)` |
| `ast.Attribute` | `AttributeExprIR` | `ExprIR.attribute` | `ExprIR::AttributeExpr(...)` |
| `ast.Subscript` | `SubscriptIR` | `ExprIR.subscript` | `ExprIR::SubscriptExpr(...)` |
| `ast.Slice` | `SliceIR` | `ExprIR.slice` | `ExprIR::SliceExpr(...)` |
| `ast.Call` | `CallExprIR` | `ExprIR.call` | `ExprIR::CallExpr(...)` |
| `ast.BinOp` | `BinOpIR` | `ExprIR.binop` | `ExprIR::BinOpExpr(...)` |
| `ast.UnaryOp` | `UnaryOpIR` | `ExprIR.unaryop` | `ExprIR::UnaryOpExpr(...)` |
| `ast.BoolOp` | `BoolOpIR` | `ExprIR.boolop` | `ExprIR::BoolOpExpr(...)` |
| `ast.Compare` | `CompareIR` | `ExprIR.compare` | `ExprIR::CompareExpr(...)` |
| `ast.IfExp` | `IfExprIR` | `ExprIR.if_expr` | `ExprIR::IfExpr(...)` |
| `ast.NamedExpr` | `NamedExprIR` | `ExprIR.named_expr` | `ExprIR::NamedExpr(...)` |
| `ast.Starred` | `StarredIR` | `ExprIR.starred` | `ExprIR::StarredExpr(...)` |
| `ast.Lambda` | `LambdaIR` | `ExprIR.lambda` | `ExprIR::Lambda(...)` |
| `ast.ListComp` | `ListCompIR` | `ExprIR.list_comp` | `ExprIR::ListComp(...)` |
| `ast.SetComp` | `SetCompIR` | `ExprIR.set_comp` | `ExprIR::SetComp(...)` |
| `ast.DictComp` | `DictCompIR` | `ExprIR.dict_comp` | `ExprIR::DictComp(...)` |
| `ast.GeneratorExp` | `GeneratorExprIR` | `ExprIR.generator_expr` | `ExprIR::GeneratorExpr(...)` |
| `ast.Await` | `AwaitIR` | `ExprIR.await_expr` | `ExprIR::AwaitExpr(...)` |
| `ast.Yield` | `YieldIR` | `ExprIR.yield_expr` | `ExprIR::YieldExpr(...)` |
| `ast.YieldFrom` | `YieldFromIR` | `ExprIR.yield_from` | `ExprIR::YieldFromExpr(...)` |
| `ast.JoinedStr` | `JoinedStrIR` | `ExprIR.joined_str` | `ExprIR::JoinedStr(...)` |
| `ast.FormattedValue` | `FormattedValueIR` | `ExprIR.formatted_value` | `ExprIR::FormattedValue(...)` |
| `ast.TemplateStr` | `TemplateStrIR` | `ExprIR.template_str` | `ExprIR::TemplateStr(...)` |
| `ast.Interpolation` | `InterpolationIR` | `ExprIR.interpolation` | `ExprIR::Interpolation(...)` |

## Statements

| Python AST | Python IR | Protobuf | Rust IR |
|---|---|---|---|
| `ast.FunctionDef` | `FunctionDefIR` | `StmtIR.function_def` | `StmtIR::FunctionDef(...)` |
| `ast.AsyncFunctionDef` | `AsyncFunctionDefIR` | `StmtIR.async_function_def` | `StmtIR::AsyncFunctionDef(...)` |
| `ast.ClassDef` | `ClassDefIR` | `StmtIR.class_def` | `StmtIR::ClassDef(...)` |
| `ast.Return` | `ReturnIR` | `StmtIR.return_stmt` | `StmtIR::Return(...)` |
| `ast.Delete` | `DeleteIR` | `StmtIR.delete_stmt` | `StmtIR::Delete(...)` |
| `ast.Assign` | `AssignIR` | `StmtIR.assign` | `StmtIR::Assign(...)` |
| `ast.TypeAlias` | `TypeAliasIR` | `StmtIR.type_alias` | `StmtIR::TypeAlias(...)` |
| `ast.AugAssign` | `AugAssignIR` | `StmtIR.aug_assign` | `StmtIR::AugAssign(...)` |
| `ast.AnnAssign` | `AnnAssignIR` | `StmtIR.ann_assign` | `StmtIR::AnnAssign(...)` |
| `ast.For` | `ForIR` | `StmtIR.for_stmt` | `StmtIR::For(...)` |
| `ast.AsyncFor` | `AsyncForIR` | `StmtIR.async_for` | `StmtIR::AsyncFor(...)` |
| `ast.While` | `WhileIR` | `StmtIR.while_stmt` | `StmtIR::While(...)` |
| `ast.If` | `IfIR` | `StmtIR.if_stmt` | `StmtIR::If(...)` |
| `ast.With` | `WithIR` | `StmtIR.with_stmt` | `StmtIR::With(...)` |
| `ast.AsyncWith` | `AsyncWithIR` | `StmtIR.async_with` | `StmtIR::AsyncWith(...)` |
| `ast.Match` | `MatchIR` | `StmtIR.match` | `StmtIR::Match(...)` |
| `ast.Raise` | `RaiseIR` | `StmtIR.raise_stmt` | `StmtIR::Raise(...)` |
| `ast.Try` | `TryIR` | `StmtIR.try_stmt` | `StmtIR::Try(...)` |
| `ast.TryStar` | `TryStarIR` | `StmtIR.try_star_stmt` | `StmtIR::TryStar(...)` |
| `ast.Assert` | `AssertIR` | `StmtIR.assert_stmt` | `StmtIR::Assert(...)` |
| `ast.Import` | `ImportIR` | `StmtIR.import_stmt` | `StmtIR::Import(...)` |
| `ast.ImportFrom` | `ImportFromIR` | `StmtIR.import_from` | `StmtIR::ImportFrom(...)` |
| `ast.Global` | `GlobalIR` | `StmtIR.global_stmt` | `StmtIR::Global(...)` |
| `ast.Nonlocal` | `NonlocalIR` | `StmtIR.nonlocal_stmt` | `StmtIR::Nonlocal(...)` |
| `ast.Expr` | `ExprStmtIR` | `StmtIR.expr_stmt` | `StmtIR::Expr(...)` |
| `ast.Pass` | `PassIR` | `StmtIR.pass_stmt` | `StmtIR::Pass(...)` |
| `ast.Break` | `BreakIR` | `StmtIR.break_stmt` | `StmtIR::Break(...)` |
| `ast.Continue` | `ContinueIR` | `StmtIR.continue_stmt` | `StmtIR::Continue(...)` |

## Pattern Matching

| Python AST | Python IR | Protobuf | Rust IR |
|---|---|---|---|
| `ast.MatchValue` | `ValuePatternIR` | `PatternIR.value_pattern` | `PatternIR::Value(...)` |
| `ast.MatchSingleton` | `SingletonPatternIR` | `PatternIR.singleton_pattern` | `PatternIR::Singleton(...)` |
| `ast.MatchSequence` | `SequencePatternIR` | `PatternIR.sequence_pattern` | `PatternIR::Sequence(...)` |
| `ast.MatchMapping` | `MappingPatternIR` | `PatternIR.mapping_pattern` | `PatternIR::Mapping(...)` |
| `ast.MatchClass` | `ClassPatternIR` | `PatternIR.class_pattern` | `PatternIR::Class(...)` |
| `ast.MatchStar` | `StarPatternIR` | `PatternIR.star_pattern` | `PatternIR::Star(...)` |
| `ast.MatchAs(pattern=None, name=<name>)` | `CapturePatternIR` | `PatternIR.capture_pattern` | `PatternIR::Capture(...)` |
| `ast.MatchAs(pattern=None, name=None)` | `WildcardPatternIR` | `PatternIR.wildcard_pattern` | `PatternIR::Wildcard(...)` |
| `ast.MatchAs(pattern=<pattern>, name=<name>)` | `AsPatternIR` | `PatternIR.as_pattern` | `PatternIR::As(...)` |
| `ast.MatchOr` | `OrPatternIR` | `PatternIR.or_pattern` | `PatternIR::Or(...)` |

## Helper AST Structures

These CPython AST structures are not top-level expressions or statements, but are embedded inside other nodes.

| Python AST | Python IR | Protobuf | Rust IR |
|---|---|---|---|
| `ast.comprehension` | `CompIR` | `CompIR` | `CompIR` |
| `ast.withitem` | `WithItemIR` | `WithItemIR` | `WithItemIR` |
| `ast.ExceptHandler` | `ExceptHandlerIR` | `ExceptHandlerIR` | `ExceptHandlerIR` |
| `ast.keyword` | `KeywordArgIR` | `KeywordArgIR` | `KeywordArgIR` |
| `ast.arguments` / `ast.arg` | `ParamIR` and parameter lowering | `ParamIR` | `ParamIR` |
| `ast.match_case` | `MatchCaseIR` | `MatchCaseIR` | `MatchCaseIR` |

## Type Parameters

Python 3.14 generic syntax uses a separate AST family for type parameters.

| Python AST | Python IR | Protobuf | Rust IR |
|---|---|---|---|
| `ast.TypeVar` | `TypeVarIR` | `TypeParamIR.type_var` | `TypeParamIR::TypeVar(...)` |
| `ast.ParamSpec` | `ParamSpecIR` | `TypeParamIR.param_spec` | `TypeParamIR::ParamSpec(...)` |
| `ast.TypeVarTuple` | `TypeVarTupleIR` | `TypeParamIR.type_var_tuple` | `TypeParamIR::TypeVarTuple(...)` |

## Root Node

For normal source-file parsing with `ast.parse(..., mode="exec")`:

| Python AST | Python IR | Protobuf | Rust IR |
|---|---|---|---|
| `ast.Module` | `ProgramIR` | `ProgramIR` | `ProgramIR` |

`ProgramIR` is intentionally richer than `ast.Module`. In addition to the lowered syntax tree, it may contain portable metadata such as file identity, declarations, bindings, and lexical scope information.

## Normalizations

The IR is not intended to preserve every CPython implementation detail literally. Some structures are normalized into representations that are easier to consume across languages.

### Constants

CPython represents literals through `ast.Constant`. The portable IR refines these into strongly typed constant variants:

```text
ast.Constant
    ↓
ConstantIR
    ├── IntegerIR
    ├── FloatIR
    ├── StringIR
    ├── BooleanIR
    ├── BytesIR
    ├── ComplexIR
    ├── NoneIR
    └── EllipsisIR
```

For example, Python's native `complex` object is serialized as explicit real and imaginary components:

```text
complex(3.0, 4.0)
    ↓
ComplexIR {
    real: 3.0,
    imag: 4.0
}
```

### MatchAs

CPython overloads `ast.MatchAs` to represent several distinct language concepts. The portable IR separates them:

```text
MatchAs(pattern=None, name="x")
    → CapturePatternIR

MatchAs(pattern=None, name=None)
    → WildcardPatternIR

MatchAs(pattern=<pattern>, name="x")
    → AsPatternIR
```

### Formatted and Template Strings

F-string components remain expressions:

```text
JoinedStrIR.values: list[ExprIR]
```

The valid values produced by the Python frontend are normally:

```text
ConstantIR::String
FormattedValueIR
```

Python 3.14 template strings use the same general abstraction:

```text
TemplateStrIR.values: list[ExprIR]
```

with interpolated portions represented through `InterpolationIR`.

## Semantic Metadata

Some project IR types do not correspond directly to CPython AST constructors.

Examples include:

```text
ProgramIR
DeclIR
BindingIR
ScopeIR
scope_id
use_scope_id
```

These are semantic enrichments derived from the AST for downstream compiler and static-analysis work.

For example, CPython has separate nodes such as:

```text
ast.FunctionDef
ast.AsyncFunctionDef
ast.ClassDef
ast.Assign
ast.AnnAssign
```

while the semantic layer may additionally classify declaration-producing constructs through `DeclIR`.

The original statement IR is still preserved; `DeclIR` is additional metadata rather than a replacement for the syntax tree.
