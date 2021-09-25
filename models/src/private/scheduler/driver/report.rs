use crate::prelude::*;
pub(crate) struct Report {
    pub timestamp: u128,
    pub name: String,
    pub reversible: bool,
}

impl fmt::Debug for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{
            "timestamp": {}, 
            "name": {}, 
            "reversible": {}
        }}"#,
            self.timestamp, self.name, self.reversible
        )
    }
}
