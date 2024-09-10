

pub enum OperadoresLogicos {
    AND, 
    OR,
}

pub struct SqlPredicate{
    condition: Vec<OperadoresLogicos>,
    operador: Vec<OperadoresLogicos>,
}



