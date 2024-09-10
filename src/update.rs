use std::{fs, io::BufWriter};
//use csv::{Reader, Writer};
//use std::error::Error;

use crate::parciar::regex_casero;
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

    let mut tabla_de_consulta = String::new(); 
    
    //let tablas_existentes = vec!["ordenes", "clientes"];
    let tabla_ingresada = &condiciones_separadas[0] ;
    match tabla_ingresada.as_str(){
        "ordenes" => tabla_de_consulta = "ordenes.csv".to_string(),
        "clientes" =>tabla_de_consulta = "clientes.csv".to_string(),
        _ => println!("la tabla ingresada no existe")
    
    }
    condiciones_separadas.remove(0);
    
    let vector_condiciones: Vec<String> = condiciones_separadas[0].trim().split("=").map(|s| s.to_string()).collect(); //[email, mrodriguez@hotmail.com]
    let vector_filtros: Vec<String> = condiciones_separadas[1].trim().split("=").map(|s| s.to_string()).collect(); //[id, 4]
    
    let _ =  update_csv(vector_condiciones, vector_filtros, tabla_de_consulta);
    
}

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
}