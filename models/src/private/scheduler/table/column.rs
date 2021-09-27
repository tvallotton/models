use crate::prelude::*;
use models_parser::{dialect::*, parser::*};
#[derive(Clone, Debug, PartialEq)]
pub struct Column {
    pub name: Ident,
    pub r#type: DataType,
    pub options: Vec<ColumnOptionDef>,
}

impl Column {
    pub fn new(name: &str, r#type: DataType, is_nullable: bool) -> Self {
        let options;
        if !is_nullable {
            options = vec![ColumnOptionDef {
                name: None,
                option: ColumnOption::NotNull,
            }];
        } else {
            options = vec![]
        }

        Column {
            name: Ident::new(name.to_lowercase()),
            r#type,
            options,
        }
    }

    pub fn new_with_default(name: &str, r#type: DataType, is_nullable: bool, def: &str) -> Self {
        let dialect = GenericDialect {};
        let mut tokens = tokenizer::Tokenizer::new(&dialect, def);
        let mut parser = Parser::new(tokens.tokenize().unwrap(), &dialect);
        let expr = parser.parse_expr().unwrap();

        let mut col = Column {
            name: Ident::new(name.to_lowercase()),
            r#type,
            options: vec![ast::ColumnOptionDef {
                name: None,
                option: ast::ColumnOption::Default(expr),
            }],
        };
        if !is_nullable {
            col.options.push(ColumnOptionDef {
                name: None,
                option: ColumnOption::NotNull,
            });
        };
        col
    }

    pub fn has_default(&self) -> bool {
        for option in &self.options {
            if matches!(option.option, ColumnOption::Default(_)) {
                return true;
            }
        }
        false
    }

    pub fn is_nullable(&self) -> bool {
        for option in &self.options {
            if matches!(option.option, ColumnOption::NotNull) {
                return false;
            }
        }
        true
    }
}

impl From<ColumnDef> for Column {
    fn from(col: ColumnDef) -> Self {
        Column {
            name: col.name,
            options: col.options,
            r#type: col.data_type,
        }
    }
}

impl From<Column> for ColumnDef {
    fn from(col: Column) -> Self {
        ColumnDef {
            name: col.name,
            options: col.options,
            data_type: col.r#type,
            collation: None,
        }
    }
}

impl TryFrom<Statement> for Table {
    type Error = String;
    fn try_from(value: Statement) -> Result<Self, Self::Error> {
        match value {
            Statement::CreateTable(table) => Ok(Table {
                name: table.name,
                if_not_exists: false,
                or_replace: false,
                columns: table.columns.into_iter().map(Into::into).collect(),
                constraints: table.constraints,
            }),
            value => Err(format!(
                "Expected a \"CREATE TABLE\" statement, found {}",
                value
            )),
        }
    }
}
impl From<Table> for Statement {
    fn from(table: Table) -> Self {
        Statement::CreateTable(Box::new(ast::CreateTable {
            or_replace: false,
            temporary: false,
            external: false,
            if_not_exists: false,
            name: table.name,
            columns: table.columns.into_iter().map(Into::into).collect(),
            constraints: table.constraints,
            hive_distribution: HiveDistributionStyle::NONE,
            hive_formats: None,
            table_properties: vec![],
            with_options: vec![],
            file_format: None,
            location: None,
            query: None,
            without_rowid: false,
            like: None,
        }))
    }
}
