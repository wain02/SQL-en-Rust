mod insert;
mod delete;
mod update;
mod parciar;
mod select;
mod sql_predicate;
//use csv::{Reader, Writer};
//use std::error::Error;
//use std::fs::OpenOptions;
//use std::io;
//use std::fs::File;
//use std::io::{BufRead, BufReader, Write};
//use std::collections::HashMap;
//use std::{fs, io::BufWriter};


fn main() {
    let terminal: Vec<String> = std::env::args().collect();
    let consulta_terminal = &terminal[2]; //consulta completa de SQL
    let consulta = consulta_terminal.replace(",", "").replace("\n", " "); 
    

    let mut partes_consulta: Vec<&str> = consulta.trim().split_whitespace().collect(); // Divide la cadena en partes
    let instruccion = partes_consulta[0];
    partes_consulta.remove(0);

    if instruccion.to_uppercase().as_str() == "INSERT" {
        //INSERT INTO ordenes (id, id_cliente, producto, cantidad) VALUES (111, 6, 'Laptop', 3);
        let consulta_ref = &consulta;
        insert::comando_insert(consulta_ref.to_string());  
    }

    if instruccion.to_uppercase().as_str() == "UPDATE" {
        let consulta_ref = &consulta;
        update::comando_update(consulta_ref.to_string());

    }
    if instruccion.to_uppercase().as_str() == "DELETE" {
        //DELETE FROM contact WHERE person_id IN (SELECT id FROM person WHERE  place_of_birth = 'San Francisco');
        //DELETE FROM person WHERE lastname = 'Burton';
        let consulta_ref = &consulta;
        delete::comando_delete(consulta_ref.to_string());

        
    }
    if instruccion.to_uppercase().as_str() == "SELECT" {
        let consulta_ref = &consulta;
        select::comando_select(consulta_ref.to_string());
    }

}
