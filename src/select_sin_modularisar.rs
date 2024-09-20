//use crate::parciar::{regex_casero, parse_operadores};
use crate::parciar::{regex_casero, parse_operadores, evaluar_condiciones_logicas, parciar_condiciones_logicas};
use crate::sql_predicate::{SqlCondicionesLogicas};
use crate::errores::SQLError;


use std::{fs, io::BufWriter};
//use csv::{Reader, Writer};
//use std::error::Error;

//use std::fs::OpenOptions;
use std::io;

//use std::fs::File;
use std::io::{BufRead, BufReader, Write};
//use csv::ReaderBuilder;

#[derive(Debug)]
pub struct OrderBy {
    columna: String,
    orden: String,
}
   
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

    let mut consulta_principal = consulta_del_terminal.trim().to_string();
    let mut order_by_clau = None;

    // Separar la consulta principal de la cláusula ORDER BY (si existe)
    
    if let Some((principal, order_by)) = &consulta_principal.split_once("ORDER BY") {
        let consulta_tempo = principal.trim().to_string();
        let order_by_tempo = order_by.trim().to_string();

        consulta_principal = consulta_tempo;
        let parts: Vec<&str> = order_by_tempo.split_whitespace().collect();
        if parts.len() == 2 {
            order_by_clau = Some(OrderBy {
                columna: parts[0].to_string(),
                orden: parts[1].to_string(),
            });
        }
    }
    
    let claves: Vec<&str> = vec!["WHERE", "SELECT", "FROM"];
    //consulta_principal.replace(",", ""); //si rompe es por esto

    let condiciones_separadas = regex_casero(&consulta_principal, claves);
    println!("{:?}", condiciones_separadas);

    if condiciones_separadas.len() < 2 {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    }

    let mut tabla_de_consulta = String::new();
    
    let tabla_leer = &condiciones_separadas[1].replace(";", "");
    let condicion = if condiciones_separadas.len() > 2 {
        &condiciones_separadas[2]
    } else {
        "" // No hay condiciones
    };

    //let tablas_existentes = vec!["ordenes", "clientes"];
    println!("{:?}", tabla_leer);
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

    if !condicion.is_empty() {
        let Some((columna_filtro, operador_valor , valor_filtro)) = parse_operadores(&condiciones_separadas[2])else {
            let error = SQLError::new("INVALID_SYNTAX");
            println!("Error: {}", error);
            return Err(error);
        };
    }    

    let condiciones_logicas = parciar_condiciones_logicas(condicion);
    
    let header_columnas: Vec<&str> = condiciones_separadas[0].trim().split_whitespace().collect();
    //let mut vector_consulta: Vec<&str> = Vec::new();
    if !(header_columnas.contains(&"*")){
        for i in header_columnas {
            vector_consulta_string.push(i.to_string());
        }
    }

    //let _ = select_csv(tabla_de_consulta, vector_consulta_string, condiciones_logicas);
    println!("{:?}", order_by_clau);
    let _ = select_csv(tabla_de_consulta, vector_consulta_string, condiciones_logicas, order_by_clau);
    Ok(())

}






pub fn select_csv(tabla: String , vector_consulta: Vec<String>, condiciones_logicas: SqlCondicionesLogicas, order_by_clau:  Option<OrderBy>)-> io::Result<()>{
    let mut index_vector_consulta = Vec::new();
    let mut index_condiciones = Vec::new();
    //let mut index = 0;
    let mut rows = Vec::new();

    let input = BufReader::new(fs::File::open(&tabla)?);
    let mut lines = input.lines();
    let mut archivo_output = BufWriter::new(fs::File::create("output.csv")?);

    let mut order_by_index = None;
    let mut order_by_asc = true;
    
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
            if let Some(order_by) = &order_by_clau {
                if order_by.columna == columna {
                    order_by_index = Some(contador_columnas);
                    order_by_asc = order_by.orden.eq_ignore_ascii_case("ASC");
                }
            }

        }
    }
    
    if !condiciones_logicas.conditions.is_empty() && index_condiciones.is_empty() {
        eprintln!("Error: No se encontraron coincidencias para las condiciones.");
        return Ok(());  // Salir sin realizar ninguna acción si no hay coincidencias
    }

    for linea_actual in lines {
        let line = linea_actual?;
        let columnas: Vec<&str> = line.split(',').collect();
        if condiciones_logicas.conditions.is_empty() || evaluar_condiciones_logicas(&columnas, &index_condiciones, &condiciones_logicas){
            let columnas_seleccionadas: Vec<&str> = index_vector_consulta.iter().map(|&i| columnas[i]).collect();
            //writeln!(archivo_output, "{}", columnas_seleccionadas.join(","))?;
            
            //let columnas_seleccionadas: Vec<&str> = index_vector_consulta.iter().map(|&i| columnas[i]).collect();
            rows.push(line);
            //writeln!(archivo_output, "{}", columnas_seleccionadas.join(","))?;

        }
        
        
    }
    //println!("{:?}", rows);
    if let Some(index) = order_by_index {
        rows.sort_by(|a, b| {
            let a_columnas: Vec<&str> = a.split(',').collect();
            let b_columnas: Vec<&str> = b.split(',').collect();
            if order_by_asc {
                a_columnas[index].cmp(&b_columnas[index])
            } else {
                b_columnas[index].cmp(&a_columnas[index])
            }
        });
    }
    
    for line in rows {
        let columnas: Vec<&str> = line.split(',').collect();
        let columnas_seleccionadas: Vec<&str> = index_vector_consulta.iter().map(|&i| columnas[i]).collect();
        writeln!(archivo_output, "{}", columnas_seleccionadas.join(","))?;
    }
 
    Ok(())

}
