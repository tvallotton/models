use super::*;
use crate::prelude::*;
use fs::*;
use itertools::Itertools;
impl Schema {
    /// constructs a new Schema from the "migrations/" directory.
    #[throws(Error)]
    pub fn new(dialect: Dialect) -> Self {
        let mut out = Self {
            dialect,
            tables: HashMap::new(),
        };
        out.init()?;
        out
    }

    /// Computes the current state of the schema
    /// from the "migrations/" directory.
    #[throws(Error)]
    fn init(&mut self) {
        let stmts = self.get_statements()?;
        for stmt in stmts {
            self.update_schema(stmt);
        }
    }
    /// It retrieves a vec of all statements in the "migrations/" directory
    /// In the order they were written.
    
    fn get_statements(&mut self) -> Result<Vec<Statement>, Error> {
        self.read_dir()
            .into_iter()
            .filter(|file| file.is_file())
            .map(read_to_string)
            .into_iter()
            .map_ok(|x| x.to_lowercase())
            .map_ok(|sql| parse_sql(&self.dialect, &sql))
            .map(|result| Ok(result?))
            .map(|result| {
                match result {
                    Ok(result) => Ok(result?),
                    Err(err) => Err(err)
                }
            })
            .fold_ok(vec![], |mut a, mut b| {
                a.append(&mut b);
                a
            })
    }
    /// returns a list of all the files in the migrations directory.
    fn read_dir(&self) -> Vec<PathBuf> {
        let directory = &*MIGRATIONS_DIR;
        let mut dir: Vec<_> = read_dir(directory)
            .or_else(|_| {
                create_dir(directory) //
                    .and_then(|_| read_dir(directory))
            })
            .expect(&format!("Could not read the \"{}\" directiory.", directory))
            .map(|x| x.unwrap().path())
            .collect();
        dir.sort();
        dir
    }
}
