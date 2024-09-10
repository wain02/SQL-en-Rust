//cargo run -- ruta/a/tablas "DELETE FROM ordenes  WHERE producto = 'Laptop';"
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

pub fn comando_delete(consulta_del_terminal: String){
    
    let mut where_delete: Vec<String> =  Vec::new();
    let claves: Vec<&str> = vec!["DELETE", "FROM", "WHERE", "AND", "OR"];
    let condiciones_separadas = regex_casero(&consulta_del_terminal, claves);
    let mut tabla = String::new(); 
    let tablas: [&str; 2] = ["ordenes", "clientes"];
    for condicion in condiciones_separadas {
        
        if tablas.contains(&condicion.as_str()){
            tabla = format!("{}.csv", condicion);
        }else{
            where_delete.push(condicion);
        }
    }
   let _ = delete_csv(where_delete, tabla);
}


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
}
