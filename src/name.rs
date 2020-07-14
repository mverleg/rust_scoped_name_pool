use ::std::rc::Rc;

use crate::scope::Scope;
use std::hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    scope: Scope,
    data: InputName,
}

#[derive(Debug, Eq)]
enum InputName {
    /// A given identifier that should not collide.
    Given {
        // Index in the scope's string 'arena'.
        index: usize,
    },
    /// An anonymous identifier, but with a prefix.
    Prefixed {
        // Index in the scope's string 'arena'.
        index: usize,
    },
    /// A totally anonymous identifier.
    Anonymous {
    },
}

/// Only given identifiers can be equal; anonymous ones have no identifying information, so are assumed non-equal.
impl PartialEq for InputName {
    fn eq(&self, other: &Self) -> bool {
        if let InputName::Given { index: first } = self {
            if let InputName::Given { index: second } = other {
                return first == second
            }
        }
        return false;
    }
}

impl hash::Hash for InputName {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        match self {
            InputName::Given { index } => index.hash(state),
            InputName::Prefixed { index } => index.hash(state),
            InputName::Anonymous => 0.hash(state),
        }
    }
}
