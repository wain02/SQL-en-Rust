
use std::fs::OpenOptions;
use std::{fs, io::BufWriter};
use std::io::{BufRead, BufReader, Write};
use csv::Writer;
use crate::parciar::{regex_casero, parse_operadores, unica_condition, evaluar_condiciones_logicas, parciar_condiciones_logicas};
use crate::sql_predicate::{SqlOperador, SqlCondicionesLogicas};
use crate::sql_conditions::SqlSelect;
use std::io;
use crate::errores::SQLError;

//INSERT INTO ordenes (id, id_cliente, producto, cantidad) 
//VALUES (111, 6, 'Laptop', 3),
        //(101, 7, 'momito', 3),
        //(13, 6, 'laso', 8),
        //(001, 8, 'juan ', 0),
        //(123, 2, 'Laso', 4);



pub fn comando_insert(consulta_inst_terminal: String)  -> Result<(),SQLError>{
    
    if !consulta_inst_terminal.contains("INSERT INTO") || !consulta_inst_terminal.contains("VALUES") {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    }
    
    
    let consulta_inst = consulta_inst_terminal
        .replace("\n", " ")
        .replace(";", "")
        ;
    
    let division_values: Vec<&str> = consulta_inst.trim().split("VALUES").collect(); // Divide la cadena en partes
    if division_values.len() != 2 {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    }

    let previo_values = division_values[0];
    let posterior_values = division_values[1]; 
    let valores_insert: Vec<&str> = posterior_values.split("),").collect(); // Divide en filas por '),'

    //let valores_insert: Vec<&str> = posterior_values.trim().split_whitespace().collect(); //[111, 6, 'Laptop', 3]
    let mut columas_insert: Vec<&str> = previo_values.trim().split_whitespace().collect(); //[id, id_cliente, producto, cantidad]
    if columas_insert.len() < 3 {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    }
    columas_insert.remove(0);
    columas_insert.remove(0); 

    let mut tabla_de_consulta = String::new(); 

    let mut tabla_insert = columas_insert[0];
    match tabla_insert{
        "ordenes" => tabla_de_consulta = "ordenes.csv".to_string(),
        "clientes" =>tabla_de_consulta = "clientes.csv".to_string(),
        _ => {
            let error = SQLError::new("INVALID_TABLE");
            println!("Error: {}", error);
            return Err(error);
        }
    
    }
    columas_insert.remove(0);

    // if let Err(e) = write_csv(valores_insert, tabla_de_consulta) {
    //     eprintln!("Error writing to CSV: {}", e);
    // }
    
    write_csv(valores_insert, tabla_de_consulta);
    Ok(())

    //write_csv(valores_insert, tabla_de_consulta);
} 

pub fn write_csv(insert: Vec<&str>, tabla: String) -> io::Result<()>{

    // Abre el archivo en modo append
    let file = OpenOptions::new()
        .append(true)
        .open(&tabla)?;

    let mut writer = BufWriter::new(file);

    for i in insert {
        let linea = i.trim().replace("(", "").replace(")", "").replace("'", "").replace(" ","");
        writeln!(writer, "{}", linea)?;

    }
    
    writer.flush()?;
    println!("Datos escritos correctamente en {}", tabla);

    Ok(())
    
} 

