use super::driver::Tuple;
use crate::prelude::*;
use fs::{create_dir, File};
pub(crate) struct Migration {
    up: Vec<Statement>,
    down: Vec<Statement>,
    name: ObjectName,
}

fn timestamp() -> u128 {
    time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_micros()
}

impl Migration {
    fn new(name: ObjectName) -> Self {
        Self {
            up: vec![],
            down: vec![],
            name,
        }
    }

    pub fn up(&self) -> &[Statement] {
        &self.up[..]
    }

    pub fn is_empty(&self) -> bool {
        self.up.is_empty() && self.down.is_empty()
    }

    pub fn commit(self) -> Result<Option<Tuple>> {
        if self.is_empty() {
            return Ok(None);
        }
        let time = timestamp();

        let dir_name = format!("{}/{}_{}", *MIGRATIONS_DIR, time, self.name);
        create_dir(dir_name)?;

        let up_name = format!("{}/up.sql", dir_name);
        let down_name = format!("{}/down.sql", dir_name);
        let mut up_file = File::create(up_name)?;
        let mut down_file = File::create(down_name)?;

        for stmt in self.up {
            use std::io::Write;
            #[cfg(feature = "sqlformat")]
            let stmt = Self::formatted_stmt(stmt);
            write!(up_file, "{};\n\n", stmt)?;
        }

        for stmt in self.down {
            use std::io::Write;
            #[cfg(feature = "sqlformat")]
            let stmt = Self::formatted_stmt(stmt);
            write!(down_file, "{};\n\n", stmt)?;
        }
        let name = self.name.to_string().to_lowercase();
        Ok(Some(Tuple(time, name)))
    }

    #[cfg(feature = "sqlformat")]
    fn formatted_stmt(stmt: Statement) -> String {
        use sqlformat::QueryParams;
        let stmt = format!("{}", stmt);
        sqlformat::format(&stmt, &QueryParams::None, FORMAT_OPTIONS)
    }
}
