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
    let mut result = unica_condition(columnas, index_condiciones[0].0, index_condiciones[0].1);

    for (i, logic_op) in condiciones_logicas.logic_ops.iter().enumerate() {
        let proxima_condicion = unica_condition(columnas, index_condiciones[i + 1].0, index_condiciones[i + 1].1);
        result = match logic_op {
            SqlOperador::And => result && proxima_condicion,
            SqlOperador::Or => result || proxima_condicion,
        };
    }

    result
}


 pub fn unica_condition(columnas: &Vec<&str>, index: usize, condition: &SqlSelect) -> bool {
    let columna_valor = columnas[index].replace(" ", "").parse::<i32>();
    let valor_filtro_num = condition.valor.replace(";", "").replace(" ", "").parse::<i32>();

    if let (Ok(col_val), Ok(filtro_val)) = (columna_valor, valor_filtro_num) {
        match condition.operador.as_str() {
            "mayor" => col_val > filtro_val,
            "menor" => col_val < filtro_val,
            "igual" => col_val == filtro_val,
            _ => false,
        }
    } else {
        false
    }
}



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
                println!("Error al parsear la condición: {}", parte);  // Depuración
            }
        }

        i += 1;
        
    }

    SqlCondicionesLogicas {
        conditions,
        logic_ops,
    }

}
