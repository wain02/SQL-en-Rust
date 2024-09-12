
use crate::sql_conditions::SqlSelect;

#[derive(Debug)]
pub enum SqlOperador {//OperadoresLogicos
    And, //AND
    Or, //OR
}
#[derive(Debug)]
pub struct SqlCondicionesLogicas{//SqlPredicate
    pub conditions: Vec<SqlSelect>,
    pub logic_ops: Vec<SqlOperador>, //operadores
}



