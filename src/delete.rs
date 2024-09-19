//cargo run -- ruta/a/tablas "DELETE FROM ordenes  WHERE producto = 'Laptop';"
use std::{fs, io::BufWriter};
use crate::parciar::{regex_casero, parse_operadores, evaluar_condiciones_logicas, parciar_condiciones_logicas};
use crate::sql_predicate::SqlCondicionesLogicas;
use crate::errores::SQLError;
use std::io;
use std::io::{BufRead, BufReader, Write};

/*pub struct DeleteSQL{
    tabla: String,
    condiciones: SqlCondicionesLogicas,
}
 */
pub fn comando_delete(consulta_del_terminal: String) -> Result<(), SQLError>{
    //DELETE FROM ordenes WHERE producto = 'Laptop';
    //println!("consulta_del_terminal: {:?}", consulta_del_terminal);
    if !consulta_del_terminal.contains("DELETE FROM") || !consulta_del_terminal.contains("WHERE") {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    }

    //let where_delete: Vec<String> =  Vec::new();
    //consulta_del_terminal.replace(",", ""); 
    let claves: Vec<&str> = vec!["DELETE", "FROM", "WHERE"];
    let condiciones_separadas = regex_casero(&consulta_del_terminal, claves);
    
    if condiciones_separadas.len() < 2 {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    }

    let mut tabla_de_consulta = String::new(); 
    
    let tabla_leer = &condiciones_separadas[0];
    let condicion = &condiciones_separadas[1];

    match tabla_leer.as_str(){
        "ordenes" => tabla_de_consulta = "ordenes.csv".to_string(),
        "clientes" =>tabla_de_consulta = "clientes.csv".to_string(),
        _ => {
            let error = SQLError::new("INVALID_TABLE");
            println!("Error: {}", error);
            return Err(error);
        }
    
    }
    let Some((columna_filtro, operador_valor , valor_filtro)) = parse_operadores(&condiciones_separadas[1])else {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    };
    
    let condiciones_logicas = parciar_condiciones_logicas(condicion);

    let _ = delete_csv(tabla_de_consulta, condiciones_logicas);
    Ok(())
}


pub fn delete_csv(tabla: String, condiciones_logicas: SqlCondicionesLogicas)-> io::Result<()>{
    let vector_consulta = Vec::new();
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


/*#[test]
fn test_delete_basico() {
    let builder = Self::new().consulta_del_terminal("DELETE FROM ordenes WHERE producto = 'Laptop';");
    println!("Helado con cucharita: {:?}", builder.build());
} */

/* 
pub fn delete_csv(vec_delete: Vec<String>, tabla_delelete: String) -> io::Result<()> { //del_csv: Vec<&str>
    
    let mut target =  String::new();
    let mut columna_csv =  String::new();

    for i in vec_delete{
               
        let consulta_vec: Vec<&str> = i.trim().split("=").collect();
        
        columna_csv = consulta_vec[0].to_string();
        target = consulta_vec[1].replace("'", "").replace(";","").replace(" ", "");

    }

    let input = BufReader::new(fs::File::open(&tabla_delelete)?);
    let mut lines = input.lines();
    let mut archivo_copia = BufWriter::new(fs::File::create("archivo_copia.csv")?);
    let mut contador_columnas = 0;
    let mut index = 0;
    //let mut new_lines: Vec<String> = Vec::new();
    
    if let Some(Ok(header)) = &lines.next() {
        let columns: Vec<&str>  = header.split(',').collect();
        for i in columns {
            if i.trim() == columna_csv.trim() {
                index = contador_columnas;
            }
            contador_columnas += 1   
        }
    }
    
    for line in lines{
        let line = line?;
        let columnas: Vec<&str> = line.split(',').collect();
        if !(columnas[index] == &target.to_string()){
            
            writeln!(archivo_copia, "{line}")?;
        }else{
            println!("se borro esta linea del csv");
        }
        
    }
    fs::rename("archivo_copia.csv", tabla_delelete)?;
    Ok(())
}*/
