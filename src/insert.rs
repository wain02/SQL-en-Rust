
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use std::io;
use crate::errores::SQLError;
use std::path::Path;

//INSERT INTO ordenes (id, id_cliente, producto, cantidad) 
//VALUES (111, 6, 'Laptop', 3),
        //(101, 7, 'momito', 3),
        //(13, 6, 'laso', 8),
        //(001, 8, 'juan ', 0),
        //(123, 2, 'Laso', 4);


#[derive(Debug)]
pub struct InsertSql {
    tabla: String,
    columnas: Vec<String>,
    values: Vec<String>,
}

        
pub fn comando_insert(consulta_inst_terminal: String, direccion_archivo: String)  -> Result<(),SQLError>{
    validar_insert(&consulta_inst_terminal)?;
    let consulta_insertar = procesar_consulta(&consulta_inst_terminal, direccion_archivo)?;
    insert_csv(consulta_insertar);
    Ok(())

} 

pub fn insert_csv(consulta_insertar: InsertSql) -> io::Result<()>{
    
    let file = OpenOptions::new()
        .append(true)
        .open(&consulta_insertar.tabla)?;

    let mut writer = BufWriter::new(file);

    for valores in consulta_insertar.values {
        let linea = valores.trim().replace("(", "").replace(")", "").replace("'", "").replace(" ","");
        writeln!(writer, "{}", linea)?;
    }
    println!("Se insertaron los valores con exito");
    Ok(())
    
} 

pub fn validar_insert(consulta: &str) -> Result<(), SQLError> {
    if !consulta.contains("INSERT INTO") || !consulta.contains("VALUES") {
        return Err(SQLError::new("INVALID_SYNTAX"));
    }
    Ok(())
}


pub fn procesar_consulta(consulta: &str, direccion_archivo: String) -> Result<InsertSql, SQLError> {
    let consulta_limpia = consulta.replace("\n", " ").replace(";", "");
    
    let division_values: Vec<&str> = consulta_limpia.trim().split("VALUES").collect();
    if division_values.len() != 2 {
        return Err(SQLError::new("INVALID_SYNTAX"));
    }

    let previo_values = division_values[0];
    let posterior_values = division_values[1]; 
    let valores_insert: Vec<&str> = posterior_values.split("),").collect();
    let mut columas_insert: Vec<&str> = previo_values.trim().split_whitespace().collect();

    if columas_insert.len() < 3 {
        return Err(SQLError::new("INVALID_SYNTAX"));
    }
    columas_insert.remove(0);
    columas_insert.remove(0); 

    let mut tabla: String = direccion_archivo.to_string();
    tabla.push_str("/");
    tabla.push_str(&columas_insert[0].replace(";", ""));
    tabla.push_str(".csv");
    if !Path::new(&tabla).exists() {
        println!("No existe la tabla");
        return Err(SQLError::new("INVALID_TABLE"));
    }

    /*let tabla_insert = columas_insert[0];
    let tabla = match tabla_insert {
        "ordenes"=> "ordenes.csv".to_string(),
        "clientes" => "clientes.csv".to_string(),
        _ => return Err(SQLError::new("INVALID_TABLE")),
    };
*/
    columas_insert.remove(0);

    let columas_insert: Vec<String> = columas_insert.into_iter().map(|s| s.to_string()).collect();
    let valores_insert: Vec<String> = valores_insert.into_iter().map(|s| s.to_string()).collect();

    Ok(InsertSql{
        tabla: tabla,
        columnas: columas_insert,
        values: valores_insert,
    })
}

