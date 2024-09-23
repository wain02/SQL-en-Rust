use crate::sql_conditions::SqlSelect;
use crate::sql_predicate::{SqlOperador, SqlCondicionesLogicas};

pub fn regex_casero(sql: &str, claves: Vec<&str>) -> Vec<String> {
    //let claves = ["DELETE", "FROM", "WHERE", "AND", "OR"];
    let mut partes = Vec::new();
    let mut partes_actuales = String::new();

    for palabra in sql.split_whitespace() {
        if claves.contains(&palabra.to_uppercase().as_str()) {
            if !partes_actuales.is_empty() {
                partes.push(partes_actuales.trim().to_string());
                partes_actuales.clear();
            }
        } else {
            partes_actuales.push_str(palabra);
            partes_actuales.push(' ');
        }
    }

    if !partes_actuales.is_empty() {
        partes.push(partes_actuales.trim().to_string());
    }

    partes
}


pub fn parse_operadores(condicion: &str) -> Option<(String, String, String)> {
    let operadores = ['>', '<', '='];

    for operador in &operadores {
        if let Some(pos) = condicion.find(*operador) {
            //divide de un lado la columna y del otro el valor con su operador respectivo.
            let columna_filtro = condicion[..pos].trim().to_string();
            let valor_filtro = condicion[pos + 1..].trim().to_string();
           
            let operador = match operador {
                '>' => "mayor".to_string(),
                '<' => "menor".to_string(),
                '=' => "igual".to_string(),
                _ => return None, // Error
            };
            
            return Some((columna_filtro, operador, valor_filtro));
        }
    }

    // si el operador seleccionado no esta.
    
    None
}



pub fn evaluar_condiciones_logicas(
    columnas: &Vec<&str>, 
    index_condiciones: &Vec<(usize, &SqlSelect)>, 
    condiciones_logicas: &SqlCondicionesLogicas
) -> bool {
    if index_condiciones.is_empty() {
        eprintln!("Error: No se encontraron condiciones para evaluar.");
        return false;
    }
    
    let mut result = unica_condition(columnas, index_condiciones[0].0, index_condiciones[0].1);
    
    println!("cond log: {:?}", condiciones_logicas);
    println!("index condiciones: {:?}", index_condiciones); 
    let mut proxima_condicion = false;
    for (i, logic_op) in condiciones_logicas.logic_ops.iter().enumerate() {
        println!("i: {}", i);
        if index_condiciones.len() == 1 {
            proxima_condicion = unica_condition(columnas, index_condiciones[i].0, index_condiciones[i].1);
        }else {
            proxima_condicion = unica_condition(columnas, index_condiciones[i+1].0, index_condiciones[i+1].1);
        }
        result = match logic_op {
            SqlOperador::And => result && proxima_condicion,
            SqlOperador::Or => result || proxima_condicion,
        };
    }

    println!("--------------------------");
    result
}





/*
pub fn evaluar_condiciones_logicas(
    columnas: &Vec<&str>, 
    index_condiciones: &Vec<(usize, &SqlSelect)>, //&SqlSelect
    condiciones_logicas: &SqlCondicionesLogicas
) -> bool {
    if index_condiciones.is_empty() {
        eprintln!("Error: No se encontraron condiciones para evaluar.");
        return false;
    }
    let mut result = unica_condition(columnas, index_condiciones[0].0, index_condiciones[0].1);
    println!("cond log: {:?}", condiciones_logicas);
    for (i, logic_op) in condiciones_logicas.logic_ops.iter().enumerate() {
        let proxima_condicion = unica_condition(columnas, index_condiciones[i + 1].0, index_condiciones[i + 1].1);
        result = match logic_op {
            SqlOperador::And => result && proxima_condicion,
            SqlOperador::Or => result || proxima_condicion,
        };
    }
    println!("--------------------------");
    result
}
*/

pub fn unica_condition(
    columnas: &Vec<&str>,
    index: usize,
    condition: &SqlSelect,
) -> bool {
    // Verificar si la condición es un grupo (subcondición lógica)
    if condition.operador == "grupo" {

        if let sub_condiciones = parciar_condiciones_logicas(&condition.valor) {
            return evaluar_condiciones_logicas(columnas, &vec![], &sub_condiciones);
        } else {
            eprintln!("Error al parsear subcondiciones: {}", condition.valor);
            return false;
        }
    }

    // Condición normal (sin paréntesis)
    let columna_valor = columnas[index].replace(" ", "");
    let valor_filtro = condition.valor.replace(";", "").replace(" ", "").replace("'", "");

    // Intentamos convertir los valores a números para comparaciones numéricas
    let columna_num = columna_valor.parse::<i32>();
    let filtro_num = valor_filtro.parse::<i32>();

    // Si ambos valores son números, realizamos comparaciones numéricas
    if let (Ok(col_val), Ok(filtro_val)) = (columna_num, filtro_num) {
        match condition.operador.as_str() {
            "mayor" => col_val > filtro_val,
            "menor" => col_val < filtro_val,
            "igual" => col_val == filtro_val,
            _ => {
                eprintln!("Operador no válido: {}", condition.operador);
                false
            }
        }
    } else {
        // Si no son números, realizamos comparaciones de cadenas
        match condition.operador.as_str() {
            "igual" => columna_valor == valor_filtro,
            _ => {
                eprintln!("Operador no válido para valores no numéricos: {}", condition.operador);
                false
            }
        }
    }
}




/*
 pub fn unica_condition(columnas: &Vec<&str>, index: usize, condition: &SqlSelect) -> bool {
    let columna_valor = columnas[index].replace(" ", "");
    let valor_filtro = condition.valor.replace(";", "").replace(" ", "").replace("'", "");

    // convertir los dos valores a numros
    let columna_num = columna_valor.parse::<i32>();
    let filtro_num = valor_filtro.parse::<i32>();

    if let (Ok(col_val), Ok(filtro_val)) = (columna_num, filtro_num) {
        // Si son números, realizo la comparación
        match condition.operador.as_str() {
            "mayor" => col_val > filtro_val,
            "menor" => col_val < filtro_val,
            "igual" => col_val == filtro_val,
            _ => false,
        }
    } else {
        // Si no son números, realizo la comparación
        //println!("columna_valor: {} valor_filtro: {}", columna_valor, valor_filtro);
        match condition.operador.as_str() {
            "igual" => columna_valor == valor_filtro,
            _ => false,
        }
    }
}
*/


pub fn parciar_condiciones_logicas(condicion_raw: &str) -> SqlCondicionesLogicas {
    let mut conditions = Vec::new();
    let mut logic_ops = Vec::new();
    let mut stack: Vec<(Vec<SqlSelect>, Vec<SqlOperador>)> = Vec::new(); // Para manejar los paréntesis
    
    let binding = condicion_raw
        .replace(" AND ", "|AND|")
        .replace(" OR ", "|OR|")
        .replace("(", "|(|")
        .replace(")", "|)|");
    
    let partes_and_or: Vec<&str> = binding.split('|').collect();
    let mut i = 0;

    while i < partes_and_or.len() {
        let parte = partes_and_or[i].trim();

        match parte {
            "(" => {
                stack.push((conditions, logic_ops));
                conditions = Vec::new();
                logic_ops = Vec::new();
            }
            ")" => {
                if let Some((mut prev_conditions, mut prev_logic_ops)) = stack.pop() {
                    
                    let logical_condition = SqlCondicionesLogicas {
                        conditions,
                        logic_ops,
                    };

                    
                    let group_condition = SqlSelect {
                        columna: String::new(),
                        operador: "grupo".to_string(),
                        valor: format!("{:?}", logical_condition),
                    };

                    prev_conditions.push(group_condition);
                    conditions = prev_conditions;
                    logic_ops = prev_logic_ops;
                }
            }
            "AND" => logic_ops.push(SqlOperador::And),
            "OR" => logic_ops.push(SqlOperador::Or),
            _ => {
                
                if let Some((columna, operador, valor)) = parse_operadores(parte) {
                    conditions.push(SqlSelect {
                        columna,
                        operador,
                        valor,
                    });
                } else {
                    eprintln!("No hay condición válida en: {}", parte);  
                }
            }
        }
        i += 1;
    }

    SqlCondicionesLogicas {
        conditions,
        logic_ops,
    }
}



/*  
pub fn parciar_condiciones_logicas(condicion_raw: &str) ->SqlCondicionesLogicas {
    
    let mut conditions = Vec::new();
    let mut logic_ops = Vec::new();

    let binding = condicion_raw
        .replace(" AND ", "|AND|")
        .replace(" OR ", "|OR|");
    let partes_and_or: Vec<&str> = binding.split('|').collect();

    //let partes_and_or: Vec<&str> = condicion_raw.split_whitespace().collect();
    let mut i = 0;

    while i < partes_and_or.len() {
        
        let parte = partes_and_or[i].trim(); // Limpiar espacios

        // Verificar si la parte es un operador lógico
        if parte == "AND" {
            logic_ops.push(SqlOperador::And);
        } else if parte == "OR" {
            logic_ops.push(SqlOperador::Or);
        } else {
            // Si no es un operador lógico, parsear como una condición
            if let Some((columna, operador, valor)) = parse_operadores(parte) {
                conditions.push(SqlSelect {
                    columna,
                    operador,
                    valor,
                });
            } else {
                eprintln!("No hay condiciónes: {}", parte);  // Depuración
            }
        }

        i += 1;
        
    }

    SqlCondicionesLogicas {
        conditions,
        logic_ops,
    }

}
*/