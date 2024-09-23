//cargo run -- ruta/a/tablas "DELETE FROM ordenes  WHERE producto = 'Laptop';"
use std::{fs, io::BufWriter};
use crate::parciar::{regex_casero, evaluar_condiciones_logicas, parciar_condiciones_logicas};//parse_operadores
use crate::sql_predicate::SqlCondicionesLogicas;
use crate::errores::SQLError;
use std::io;
use std::io::{BufRead, BufReader, Write};
use crate::sql_conditions::SqlSelect;
use crate::manejo_archivos::archivo;



#[derive(Debug)]
pub struct DeleteSQL {
    tabla: String,
    where_conditions: SqlCondicionesLogicas,
}



///Funcion principal que recibe la consulta delete de SQL y el path de los archivos CSV.
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

///Recibe el nombre de la tabla y las condiciones logicas.
/// Se encarga de abrir el archivo CSV y filtrarlo por las condiciones logicas.
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


///Recibe las lineas del archivo, la tabla, los indices de las columnas a seleccionar, los indices de las condiciones y las condiciones logicas.
///Se ocupa de reescribir el archivo CSV sin las filas que cumplen con las condiciones.
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


//Obtiene la consulta SQL parciada como vector y el path de los archivos csv
//Retorna un Result con el struct DeleteSQL relleno con la consulta
fn crear_consulta_delete(condiciones_separadas: Vec<String>, direccion_archivo: String) -> Result<DeleteSQL, SQLError> {
    //let mut tabla = String::new(); 
    let tabla = archivo(&condiciones_separadas[0].to_string(), &direccion_archivo.to_string())?;

    let where_conditions = parciar_condiciones_logicas(&condiciones_separadas[1].replace(";",""));
    Ok(DeleteSQL { tabla, where_conditions })
}
