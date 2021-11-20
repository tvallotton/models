use crate::prelude::*; 



pub struct GetterVariant<'a, T> {
    pub model_name: &'a Ident, 
    pub table_name: &'a String, 
    pub variant: &'a T, 
}