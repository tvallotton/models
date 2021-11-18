use crate::prelude::*; 



pub struct GetterVariant<'a, T> {
    pub model_name: &'a Ident, 
    pub variant: &'a T, 
}