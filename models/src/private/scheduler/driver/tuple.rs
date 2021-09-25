use crate::prelude::*;
pub(crate) struct Tuple(pub u128, pub String);

impl fmt::Debug for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{:?}]", self.0, self.1)
    }
}
