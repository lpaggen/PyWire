pub mod excepthandler;

pub use excepthandler::ExceptHandler;

#[derive(Debug, Clone, PartialEq)]
pub enum ExceptHandlerNode {
    ExceptHandler(Box<ExceptHandler>),
}
