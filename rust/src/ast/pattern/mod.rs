pub mod matchas;
pub mod matchclass;
pub mod matchmapping;
pub mod matchor;
pub mod matchsequence;
pub mod matchsingleton;
pub mod matchstar;
pub mod matchvalue;

pub use matchas::MatchAs;
pub use matchclass::MatchClass;
pub use matchmapping::MatchMapping;
pub use matchor::MatchOr;
pub use matchsequence::MatchSequence;
pub use matchsingleton::MatchSingleton;
pub use matchstar::MatchStar;
pub use matchvalue::MatchValue;

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    MatchValue(Box<MatchValue>),
    MatchSingleton(Box<MatchSingleton>),
    MatchSequence(Box<MatchSequence>),
    MatchMapping(Box<MatchMapping>),
    MatchClass(Box<MatchClass>),
    MatchStar(Box<MatchStar>),
    MatchAs(Box<MatchAs>),
    MatchOr(Box<MatchOr>),
}
