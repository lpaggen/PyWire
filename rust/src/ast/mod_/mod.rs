pub mod expression;
pub mod functiontype;
pub mod interactive;
pub mod module;
pub mod suite;
pub mod typeignore;

pub use expression::Expression;
pub use functiontype::FunctionType;
pub use interactive::Interactive;
pub use module::Module;
pub use suite::Suite;
pub use typeignore::TypeIgnore;

#[derive(Debug, Clone, PartialEq)]
pub enum Mod {
    Module(Module),
    Interactive(Interactive),
    Expression(Expression),
    FunctionType(FunctionType),
    Suite(Suite),
}
