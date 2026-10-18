pub mod annassign;
pub mod assert;
pub mod assign;
pub mod asyncfor;
pub mod asyncfunctiondef;
pub mod asyncwith;
pub mod augassign;
pub mod r#break;
pub mod classdef;
pub mod r#continue;
pub mod delete;
pub mod expr;
pub mod r#for;
pub mod functiondef;
pub mod global;
pub mod r#if;
pub mod import;
pub mod importfrom;
pub mod r#match;
pub mod nonlocal;
pub mod pass;
pub mod raise;
pub mod r#return;
pub mod r#try;
pub mod trystar;
pub mod typealias;
pub mod r#while;
pub mod with;

pub use annassign::AnnAssign;
pub use assert::Assert;
pub use assign::Assign;
pub use asyncfor::AsyncFor;
pub use asyncfunctiondef::AsyncFunctionDef;
pub use asyncwith::AsyncWith;
pub use augassign::AugAssign;
pub use r#break::Break;
pub use classdef::ClassDef;
pub use r#continue::Continue;
pub use delete::Delete;
pub use expr::Expr;
pub use r#for::For;
pub use functiondef::FunctionDef;
pub use global::Global;
pub use r#if::If;
pub use import::Import;
pub use importfrom::ImportFrom;
pub use r#match::Match;
pub use nonlocal::Nonlocal;
pub use pass::Pass;
pub use raise::Raise;
pub use r#return::Return;
pub use r#try::Try;
pub use trystar::TryStar;
pub use typealias::TypeAlias;
pub use r#while::While;
pub use with::With;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    FunctionDef(Box<FunctionDef>),
    AsyncFunctionDef(Box<AsyncFunctionDef>),
    ClassDef(Box<ClassDef>),
    Return(Box<Return>),
    Delete(Box<Delete>),
    Assign(Box<Assign>),
    TypeAlias(Box<TypeAlias>),
    AugAssign(Box<AugAssign>),
    AnnAssign(Box<AnnAssign>),
    For(Box<For>),
    AsyncFor(Box<AsyncFor>),
    While(Box<While>),
    If(Box<If>),
    With(Box<With>),
    AsyncWith(Box<AsyncWith>),
    Match(Box<Match>),
    Raise(Box<Raise>),
    Try(Box<Try>),
    TryStar(Box<TryStar>),
    Assert(Box<Assert>),
    Import(Box<Import>),
    ImportFrom(Box<ImportFrom>),
    Global(Box<Global>),
    Nonlocal(Box<Nonlocal>),
    Expr(Box<Expr>),
    Pass(Box<Pass>),
    Break(Box<Break>),
    Continue(Box<Continue>),
}
