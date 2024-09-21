//cargo run -- ruta/a/tablas "DELETE FROM ordenes  WHERE producto = 'Laptop';"
use std::{fs, io::BufWriter};
use crate::parciar::{regex_casero, parse_operadores, evaluar_condiciones_logicas, parciar_condiciones_logicas};
use crate::sql_predicate::SqlCondicionesLogicas;
use crate::errores::SQLError;
use std::io;
use std::io::{BufRead, BufReader, Write};
use crate::sql_conditions::SqlSelect;
use std::path::Path;

#[derive(Debug)]
pub struct DeleteSQL {
    tabla: String,
    where_conditions: SqlCondicionesLogicas,
}


pub fn comando_delete(consulta_del_terminal: String, direccion_archivo: String) -> Result<(), SQLError>{
    //DELETE FROM ordenes WHERE producto = 'Laptop';
    if !consulta_del_terminal.contains("DELETE FROM") || !consulta_del_terminal.contains("WHERE") {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    }
    let claves: Vec<&str> = vec!["DELETE", "FROM", "WHERE"];
    let condiciones_separadas = regex_casero(&consulta_del_terminal, claves);
    
    if condiciones_separadas.len() < 2 {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    }

    let consulta_sql = crear_consulta_delete(condiciones_separadas, direccion_archivo)?;
    let _ = delete_csv(consulta_sql.tabla, consulta_sql.where_conditions);
    Ok(())
}


pub fn delete_csv(tabla: String, condiciones_logicas: SqlCondicionesLogicas)-> io::Result<()>{
    let vector_consulta = Vec::new();
    let mut index_vector_consulta = Vec::new();
    let mut index_condiciones = Vec::new();

    let input = BufReader::new(fs::File::open(&tabla)?);
    let mut lines = input.lines();
    let mut archivo_output = BufWriter::new(fs::File::create("output.csv")?);

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
    escribir_delete(&mut archivo_output, &mut lines, &tabla, &index_vector_consulta, &index_condiciones, &condiciones_logicas)?;
    Ok(())
}



fn escribir_delete(
    archivo_output: &mut BufWriter<fs::File>,
    lines: &mut std::io::Lines<BufReader<fs::File>>,
    tabla: &String,
    index_vector_consulta: &Vec<usize>,
    index_condiciones: &Vec<(usize, &SqlSelect)>,
    condiciones_logicas: &SqlCondicionesLogicas,
) -> io::Result<()>{
    if index_condiciones.is_empty() {
        eprintln!("Error: No se encontraron coincidencias para las condiciones.");
        return Ok(());  // Salir sin realizar ninguna acción si no hay coincidencias
    }

    for line in lines {
        let line = line?;
        let columnas: Vec<&str> = line.split(',').collect();
        if evaluar_condiciones_logicas(&columnas, &index_condiciones, &condiciones_logicas){
            println!("Se eliminaron los datos");
        }else{
            //writeln!(archivo_output, "{}", line)?;
            let columnas_seleccionadas: Vec<&str> = index_vector_consulta.iter().map(|&i| columnas[i]).collect();
            writeln!(archivo_output, "{}", columnas_seleccionadas.join(","))?;
        }
    }
    fs::rename("output.csv", tabla)?;
    Ok(())
}

fn crear_consulta_delete(condiciones_separadas: Vec<String>, direccion_archivo: String) -> Result<DeleteSQL, SQLError> {
    //let mut tabla = String::new(); 
    
    let mut tabla: String = direccion_archivo.to_string();
    tabla.push_str("/");
    tabla.push_str(&condiciones_separadas[0].replace(";", ""));
    tabla.push_str(".csv");
    if !Path::new(&tabla).exists() {
        println!("No existe la tabla");
        return Err(SQLError::new("INVALID_TABLE"));
    }

    /*let tabla_de_consulta = condiciones_separadas[0].trim().to_string();
    match tabla_de_consulta.as_str(){
        "ordenes" => tabla = "ordenes.csv".to_string(),
        "clientes" =>tabla = "clientes.csv".to_string(),
        _ => {
            let error = SQLError::new("INVALID_TABLE");
            println!("Error: {}", error);
            return Err(error);
        }
    }*/
    let where_conditions = parciar_condiciones_logicas(&condiciones_separadas[1].replace(";",""));
    Ok(DeleteSQL { tabla, where_conditions })
}
