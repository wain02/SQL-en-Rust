

pub struct SqlConditions{
    columna: String,
    operador: String,
    valor: String,
}

pub fn newSqlCondition(c: String, o: String, v:String) -> SqlConditions{
    let s1 = SqlConditions{columna: c, operador: o, valor: v};
    return s1
}