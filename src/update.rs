//cargo run -- ruta/a/tablas "UPDATE clientes SET email = 'mrodriguez@hotmail.com', nombre = 'Sol' WHERE id = 4;"
use std::{fs, io::BufWriter};
use crate::parciar::{regex_casero, parse_operadores, unica_condition, evaluar_condiciones_logicas, parciar_condiciones_logicas};
use crate::sql_predicate::{SqlOperador, SqlCondicionesLogicas};
use crate::sql_conditions::SqlSelect;
use std::io;
use std::io::{BufRead, BufReader, Write};
use crate::errores::SQLError;

pub fn comando_update(consulta_del_terminal: String) -> Result<(), SQLError> {
    //UPDATE clientes SET email = 'mrodriguez@hotmail.com' WHERE id = 4;

    if !consulta_del_terminal.contains("UPDATE") || !consulta_del_terminal.contains("SET") || !consulta_del_terminal.contains("WHERE") {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    }

    let claves: Vec<&str> = vec!["UPDATE", "SET", "WHERE"];
    let mut condiciones_separadas = regex_casero(&consulta_del_terminal, claves);

    if condiciones_separadas.len() < 3 {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    }

    let tabla_leer = &condiciones_separadas[0];
    let condicion = &condiciones_separadas[2];
    let editar = condiciones_separadas[1].replace(","," AND ");

    let mut tabla_de_consulta = String::new();

    match tabla_leer.as_str(){
        "ordenes" => tabla_de_consulta = "ordenes.csv".to_string(),
        "clientes" =>tabla_de_consulta = "clientes.csv".to_string(),
        _ => {
            let error = SQLError::new("INVALID_TABLE");
            println!("Error: {}", error);
            return Err(error);
        }
    
    }

    let Some((columna_filtro, operador_valor , valor_filtro)) = parse_operadores(&condiciones_separadas[2])else {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    };
    let condiciones_logicas = parciar_condiciones_logicas(condicion);

    let Some((columna_filtro, operador_valor , valor_filtro)) = parse_operadores(&condiciones_separadas[1].replace(","," AND "))else {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    };

    let condiciones_editar = parciar_condiciones_logicas(&editar);
    
    let _ =  update_csv(tabla_de_consulta, condiciones_editar, condiciones_logicas);
    Ok(())

}


pub fn update_csv(tabla: String , condiciones_editar: SqlCondicionesLogicas, condiciones_logicas: SqlCondicionesLogicas)-> io::Result<()>{
    
    let mut index_condiciones_editar = Vec::new();
    let mut index_condiciones = Vec::new();

    let input = BufReader::new(fs::File::open(&tabla)?);
    let mut lines = input.lines();
    let mut archivo_output = BufWriter::new(fs::File::create("output.csv")?);
   
    if let Some(Ok(header)) = &lines.next() {
        let columnas_archivo: Vec<&str> = header.split(',').collect(); 
        
        writeln!(archivo_output, "{}", header)?; //escribo el header en el archivo output

        for (contador_columnas, columna) in columnas_archivo.iter().enumerate() {
            let columna = columna.trim();
            for cond_editar in &condiciones_editar.conditions{
                //for para guardar los datos que hay que editar
                if cond_editar.columna.trim() == columna.trim() {
                    index_condiciones_editar.push((contador_columnas, cond_editar));
                }
            }
            
            for cond in &condiciones_logicas.conditions {
                //for para guardar los datos que hay que filtrar
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
    if index_condiciones_editar.is_empty() {
        eprintln!("Error: No se encontraron coincidencias para las condiciones a editar.");
        return Ok(());  // Salir sin realizar ninguna acción si no hay coincidencias
    }

    for line in lines {
        let line = line?;
        let mut columnas: Vec<&str> = line.split(',').collect();
        if evaluar_condiciones_logicas(&columnas, &index_condiciones, &condiciones_logicas){
            println!("se actualizo esta linea del csv");

            for index in &index_condiciones_editar {
                let valor_modificar = index.1;
                columnas[index.0] = &valor_modificar.valor; 
                
            }
            let nueva_linea = columnas.join(",").replace("'","");
            writeln!(archivo_output, "{nueva_linea}")?;
            
        }else{
            writeln!(archivo_output, "{}", line)?;
        }
    }
    fs::rename("output.csv", tabla)?;
    Ok(())

}
