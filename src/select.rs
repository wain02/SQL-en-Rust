
//use crate::parciar::{regex_casero, parse_operadores};
use crate::parciar::{regex_casero, parse_operadores, unica_condition, evaluar_condiciones_logicas, parciar_condiciones_logicas};
use crate::sql_conditions::SqlSelect;
use crate::sql_predicate::{SqlOperador, SqlCondicionesLogicas};
use crate::errores::SQLError;


use std::{fs, io::BufWriter};
//use csv::{Reader, Writer};
//use std::error::Error;

//use std::fs::OpenOptions;
use std::io;

//use std::fs::File;
use std::io::{BufRead, BufReader, Write};
//use csv::ReaderBuilder;


//#[derive(Debug)]
/* 
struct SqlSelect{
    columna: String,
    operador: String,
    valor: String,
}
 
enum SqlOperador {
    And,
    Or,
}

pub struct SqlCondicionesLogicas {
    conditions: Vec<SqlSelect>,
    logic_ops: Vec<SqlOperador>, // Operadores entre las condiciones
}
*/
pub fn comando_select(consulta_del_terminal: String) -> Result<(), SQLError> {
    //SELECT id, producto, id_cliente FROM ordenes WHERE cantidad > 1;
    if !consulta_del_terminal.contains("SELECT") || !consulta_del_terminal.contains("FROM") {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    }
    
    let claves: Vec<&str> = vec!["WHERE", "SELECT", "FROM"];
    consulta_del_terminal.replace(",", ""); //si rompe es por esto

    let condiciones_separadas = regex_casero(&consulta_del_terminal, claves);

    if condiciones_separadas.len() < 3 {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    }

    let mut tabla_de_consulta = String::new();
    
    let tabla_leer = &condiciones_separadas[1]; 
    let condicion = &condiciones_separadas[2];

    //let tablas_existentes = vec!["ordenes", "clientes"];
    
    match tabla_leer.as_str(){
        "ordenes" => tabla_de_consulta = "ordenes.csv".to_string(),
        "clientes" =>tabla_de_consulta = "clientes.csv".to_string(),
        _ => {
            let error = SQLError::new("INVALID_TABLE");
            println!("Error: {}", error);
            return Err(error);
        }
    
    }
    let mut vector_consulta_string: Vec<String> = Vec::new();
    //let mut vec_filtro_string: Vec<String> = Vec::new();

    
    let Some((columna_filtro, operador_valor , valor_filtro)) = parse_operadores(&condiciones_separadas[2])else {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    };
        // let struct_filtro = SqlSelect {
        //     columna: columna_filtro,
        //     operador: operador_valor,
        //     valor: valor_filtro,
        // };

    let condiciones_logicas = parciar_condiciones_logicas(condicion);
    
    let header_columnas: Vec<&str> = condiciones_separadas[0].trim().split_whitespace().collect();
    //let mut vector_consulta: Vec<&str> = Vec::new();
    if !(header_columnas.contains(&"*")){
        for i in header_columnas {
            vector_consulta_string.push(i.to_string());
        }
    }

    let _ = select_csv(tabla_de_consulta, vector_consulta_string, condiciones_logicas);
    Ok(())

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
*/

pub fn select_csv(tabla: String , vector_consulta: Vec<String>, condiciones_logicas: SqlCondicionesLogicas)-> io::Result<()>{
    let mut index_vector_consulta = Vec::new();
    let mut index_condiciones = Vec::new();
    //let mut index = 0;

    let input = BufReader::new(fs::File::open(&tabla)?);
    let mut lines = input.lines();
    let mut archivo_output = BufWriter::new(fs::File::create("output.csv")?);

    
    //let mut contador_columnas = 0;
    if let Some(Ok(header)) = &lines.next() {
        let columnas_archivo: Vec<&str> = header.split(',').collect(); 
              
        for (contador_columnas, columna) in columnas_archivo.iter().enumerate() {
            let columna = columna.trim();
            if vector_consulta.is_empty() {
                index_vector_consulta.push(contador_columnas);
            }
            else if vector_consulta.contains(&columna.to_string()) {
                index_vector_consulta.push(contador_columnas);
                
            }
            for cond in &condiciones_logicas.conditions {
                if cond.columna.trim() == columna.trim() {
                    index_condiciones.push((contador_columnas, cond));
                }
            } 
        }
    }
    
    if index_condiciones.is_empty() {
        eprintln!("Error: No se encontraron coincidencias para las condiciones.");
        return Ok(());  // Salir sin realizar ninguna acción si no hay coincidencias
    }

    for line in lines {
        let line = line?;
        let columnas: Vec<&str> = line.split(',').collect();
        if evaluar_condiciones_logicas(&columnas, &index_condiciones, &condiciones_logicas){
            //writeln!(archivo_output, "{}", line)?;
            let columnas_seleccionadas: Vec<&str> = index_vector_consulta.iter().map(|&i| columnas[i]).collect();

            writeln!(archivo_output, "{}", columnas_seleccionadas.join(","))?;
        }
    }
    Ok(())

}

/* 
fn evaluar_condiciones_logicas(
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


fn unica_condition(columnas: &Vec<&str>, index: usize, condition: &SqlSelect) -> bool {
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
}*/

/*  
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
*/








