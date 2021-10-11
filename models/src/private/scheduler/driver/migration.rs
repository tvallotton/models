use super::{
    actions::{action::Action, Actions},
    schema::Schema,
    Report,
};
use crate::prelude::*;
use fs::File;
use std::io::Write;
#[derive(Debug)]
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
    pub fn new(name: ObjectName) -> Self {
        Self {
            up: vec![],
            down: vec![],
            name,
        }
    }

    pub fn create_down(&mut self, old: Schema, new: &Schema, table: &ObjectName) -> Result {
        if let Some(target) = old.get_table(table) {
            let actions = Actions::new(&new, &target)?;

            self.down = actions
                .as_migrations()? //
                .into_iter()
                .map(|mig| mig.up)
                .fold(vec![], |mut x, mut y| {
                    x.append(&mut y);
                    x
                });
        } else {
            let drop_stmt = Statement::Drop(Drop {
                object_type: ObjectType::Table,
                if_exists: false,
                names: vec![table.clone()],
                cascade: DIALECT.supports_cascade(),
                purge: false,
            });
            self.down.push(drop_stmt);
        }
        Ok(())
    }

    pub fn up(&self) -> &[Statement] {
        &self.up[..]
    }

    pub fn is_empty(&self) -> bool {
        self.up.is_empty()
    }
    pub fn push_up(&mut self, action: Action) -> Result {
        let stmts = action.to_statements()?;
        self.up.extend(stmts);
        Ok(())
    }

    fn write_to_file(file_name: &str, stmts: &[Statement]) -> Result<()> {
        let mut file = File::create(file_name)?;
        for stmt in stmts {
            #[cfg(feature = "sqlformat")]
            let stmt = Self::formatted_stmt(stmt);
            write!(file, "{};\n\n", stmt)?;
        }
        Ok(())
    }

    pub fn commit(self) -> Result<Option<Report>> {
        if self.is_empty() {
            return Ok(None);
        }
        let timestamp = timestamp();
        let file_name = format!("{}/{}_{}", *MIGRATIONS_DIR, timestamp, self.name);

        let name = self.name.to_string().to_lowercase();
        if !*MODELS_GENERATE_DOWN {
            let up = format!("{}.sql", file_name);
            Self::write_to_file(&up, &self.up)?;
            return Ok(Some(Report { timestamp, name }));
        } else {
            let up = format!("{}.up.sql", file_name);
            let down = format!("{}.down.sql", file_name);
            Self::write_to_file(&up, &self.up)?;
            Self::write_to_file(&down, &self.down)?;
            return Ok(Some(Report { timestamp, name }));
        };
    }

    #[cfg(feature = "sqlformat")]
    fn formatted_stmt(stmt: &Statement) -> String {
        use sqlformat::QueryParams;
        let stmt = format!("{}", stmt);
        sqlformat::format(&stmt, &QueryParams::None, FORMAT_OPTIONS)
    }
}
