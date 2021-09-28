use crate::prelude::*;
pub(crate) struct Report {
    pub timestamp: u128,
    pub name: String,
}

impl fmt::Debug for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"[{}, {:?}]"#, self.timestamp, self.name,)
    }
}
