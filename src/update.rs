use std::{fs, io::BufWriter};
//use csv::{Reader, Writer};
//use std::error::Error;
use crate::parciar::{regex_casero, parse_operadores, unica_condition, evaluar_condiciones_logicas, parciar_condiciones_logicas};
use crate::sql_predicate::{SqlOperador, SqlCondicionesLogicas};
use crate::sql_conditions::SqlSelect;
//use crate::parciar::regex_casero;
//use csv::Writer;
//use std::fs::OpenOptions;
use std::io;

//use std::fs::File;
use std::io::{BufRead, BufReader, Write};
//use csv::ReaderBuilder;

pub fn comando_update(consulta_del_terminal: String){
    //UPDATE clientes SET email = 'mrodriguez@hotmail.com' WHERE id = 4;

    let claves: Vec<&str> = vec!["UPDATE", "SET", "WHERE"];
    let mut condiciones_separadas = regex_casero(&consulta_del_terminal, claves);

    let tabla_leer = &condiciones_separadas[0];
    let condicion = &condiciones_separadas[2];
    let editar = &condiciones_separadas[1];


    let mut tabla_de_consulta = String::new(); 
    
    //let tablas_existentes = vec!["ordenes", "clientes"];
    //let tabla_ingresada = &condiciones_separadas[0] ;
    match tabla_leer.as_str(){
        "ordenes" => tabla_de_consulta = "ordenes.csv".to_string(),
        "clientes" =>tabla_de_consulta = "clientes.csv".to_string(),
        _ => println!("la tabla ingresada no existe")
    
    }



    let Some((columna_filtro, operador_valor , valor_filtro)) = parse_operadores(&condiciones_separadas[2])else { todo!() };
    let condiciones_logicas = parciar_condiciones_logicas(condicion);

    let Some((columna_filtro, operador_valor , valor_filtro)) = parse_operadores(&condiciones_separadas[1])else { todo!() };
    let condiciones_editar = parciar_condiciones_logicas(editar);
    // println!("condiciones_logicas: {:?} ", condiciones_logicas);
    // println!("----------------------- ----------------------");
    // println!("condiciones_editar: {:?} ", condiciones_editar);
    //condiciones_separadas.remove(0);
    
    //let vector_condiciones: Vec<String> = condiciones_separadas[0].trim().split("=").map(|s| s.to_string()).collect(); //[email, mrodriguez@hotmail.com]
    //let vector_filtros: Vec<String> = condiciones_separadas[1].trim().split("=").map(|s| s.to_string()).collect(); //[id, 4]
    
    //let _ =  update_csv(vector_condiciones, vector_filtros, tabla_de_consulta);
    let _ =  update_csv(tabla_de_consulta, condiciones_editar, condiciones_logicas);

    
}




pub fn update_csv(tabla: String , condiciones_editar:SqlCondicionesLogicas, condiciones_logicas: SqlCondicionesLogicas)-> io::Result<()>{
    let mut index_condiciones_editar = Vec::new();
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
            for cond_editar in &condiciones_editar.conditions{
                if cond_editar.columna.trim() == columna.trim() {
                    index_condiciones_editar.push((contador_columnas, cond_editar));
                }
            }
            // if vector_consulta.contains(&columna.to_string()) {
            //     index_vector_consulta.push(contador_columnas);
                
            // }
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
    if index_condiciones_editar.is_empty() {
        eprintln!("Error: No se encontraron coincidencias para las condiciones a editar.");
        return Ok(());  // Salir sin realizar ninguna acción si no hay coincidencias
    }

    for line in lines {
        let line = line?;
        let mut columnas: Vec<&str> = line.split(',').collect();
        if evaluar_condiciones_logicas(&columnas, &index_condiciones, &condiciones_logicas){
            println!("se actualizo esta linea del csv");
            
            columnas[index_condiciones_editar[0].0] = &index_condiciones_editar[0].1.valor;
            let nueva_linea = columnas.join(",");
            writeln!(archivo_output, "{nueva_linea}")?;
        }else{
            writeln!(archivo_output, "{line}")?;
        }
    }
    fs::rename("output.csv", tabla)?;
    Ok(())

}




/* 
pub fn update_csv(vec_update: Vec<String>, vec_filtro: Vec<String> ,tabla_update: String)-> io::Result<()>{
    let columna_update =  &vec_update[0]; //email
    let valor_update =  &vec_update[1]; //mrodriguez@hotmail.com
    let columna_filtro =  &vec_filtro[0]; //id
    let valor_filtro =  &vec_filtro[1]; //4

    let mut index_filtro = 0; //0
    let mut index_update = 0; //3


    let  input = BufReader::new(fs::File::open(&tabla_update)?);
    let mut lines = input.lines();
    let mut archivo_copia = BufWriter::new(fs::File::create("archivo_copia.csv")?);

    let mut contador_columnas = 0;
    
    //let mut new_lines: Vec<String> = Vec::new();
    
    if let Some(Ok(header)) = &lines.next() {
        let columns: Vec<&str>  = header.split(',').collect();
        for i in columns {
            if i.trim() == columna_filtro.trim() {
                index_filtro = contador_columnas; //0
            }
            if i.trim() == columna_update.trim() {
                index_update = contador_columnas; //3
            }
            
            contador_columnas += 1   
        }
    }
    println!("indice update: {}", index_update);

    for line in lines {
        let line = line?;
        let mut columnas: Vec<&str> = line.split(',').collect();
        
        if !(columnas[index_filtro].replace(" ", "") == valor_filtro.to_string().replace(";", "").replace(" ", "")){ //fijarme si puedo cambiar  esto
            writeln!(archivo_copia, "{line}")?;
        }else{
            println!("se actualizo esta linea del csv");
            columnas[index_update] = valor_update;
            //line[index_update] = valor_update;
            let nueva_linea = columnas.join(",");
            writeln!(archivo_copia, "{nueva_linea}")?;
        }

    }
    
    fs::rename("archivo_copia.csv", tabla_update)?;
    Ok(())
}*/