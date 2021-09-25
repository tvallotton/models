use crate::prelude::*; 
pub enum ActionConstraint {
    ForeignKey {
        referred_table: ObjectName,
        reffered_column: ObjectName,
        local_column: ObjectName,
    },
    Unique {
        columns: Vec<ObjectName>,
        is_primary: bool,
    },
}


impl ActionConstraint {
    fn name(&self, table_name: ObjectName) {
        
    }
}
