use crate::parciar::{regex_casero, evaluar_condiciones_logicas, parciar_condiciones_logicas};
use crate::sql_predicate::SqlCondicionesLogicas;
use crate::errores::SQLError;
use std::{fs, io::BufWriter};
use std::io::{self, BufRead, BufReader, Write};
use crate::sql_conditions::SqlSelect;
use crate::manejo_archivos::archivo;



#[derive(Debug)]
pub struct UpdateSql {
    tabla: String,
    set: Vec<(String, String)>,
    where_conditions: Option<SqlCondicionesLogicas>,
}

pub fn comando_update(consulta_del_terminal: String, direccion_archivo: String) -> Result<(), SQLError> {

    if !consulta_del_terminal.contains("UPDATE") || !consulta_del_terminal.contains("SET") {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    }
    let mut consulta_principal = consulta_del_terminal.trim().to_string();
    let condiciones_separadas = regex_casero(&consulta_principal, vec!["WHERE", "UPDATE", "SET"]);
    if condiciones_separadas.len() < 2 {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    }
    let consulta_sql = crear_consulta_update(condiciones_separadas, direccion_archivo)?;
    let _ = update_csv(consulta_sql);
    Ok(())
}

pub fn update_csv(consulta: UpdateSql) -> io::Result<()> {
    
    let input = BufReader::new(fs::File::open(&consulta.tabla)?);
    let mut lineas = input.lines();
    let mut archivo_output = BufWriter::new(fs::File::create("output.csv")?);

    let mut rows = Vec::new();
    let mut header = String::new();
    let mut index_condiciones = Vec::new();
    let mut index_editar = Vec::new();

    if let Some(Ok(line)) = lineas.next() {
        writeln!(archivo_output, "{}", line)?;
        header = line;
        rows.push(&header);
        let columnas: Vec<&str> = header.split(',').collect();
        for (i, columna) in columnas.iter().enumerate() {
            let columna = columna.trim().to_string();
            if let Some(where_conditions) = &consulta.where_conditions {
                for cond in &where_conditions.conditions {
                    if cond.columna.trim() == columna {
                        index_condiciones.push((i, cond));
                    }
                }
            }
            for edit in &consulta.set {
                let col = &edit.0;
                if *col == columna {
                    let valor_edir = &edit.1;
                    index_editar.push((i, &edit.1));
                }
            }
        }
    }
    escribir_resultado(&mut archivo_output, &mut lineas, &consulta, &index_condiciones, &index_editar)?;
    Ok(())
}


pub fn escribir_resultado(archivo_output:&mut BufWriter<fs::File>, 
    lineas: &mut std::io::Lines<BufReader<fs::File>>, 
    consulta: &UpdateSql, 
    index_condiciones:&Vec<(usize, &SqlSelect)>, 
    index_editar:&Vec<(usize, &String)>
) -> io::Result<()> {
    for linea_actual in lineas {
        let line = linea_actual?;
        let columnas: Vec<&str> = line.split(',').collect();
        let where_conditions = &consulta.where_conditions;
        
        if let Some(where_conditions) = where_conditions {
            
            if where_conditions.conditions.is_empty()
                || evaluar_condiciones_logicas(&columnas, &index_condiciones, &consulta.where_conditions.as_ref().unwrap())
            {
                let new_line = &actualizar_fila(&line, &consulta.set, &index_editar);
                writeln!(archivo_output, "{}", new_line)?;
            } else {
                writeln!(archivo_output, "{}", line)?;
            }
        } else {
            writeln!(archivo_output, "{}", line)?;
        }
    }
    let tabla = &consulta.tabla;
    println!("Se ha actualizado la tabla {}", tabla);
    fs::rename("output.csv", tabla)?;
    Ok(())
}

fn actualizar_fila(line: &str, set: &Vec<(String, String)>, index_editar: &Vec<(usize, &String)>) -> String {
    let mut columnas: Vec<&str> = line.split(',').collect();
    for (columna, valor) in index_editar {
        columnas[*columna] = valor;
    }
    columnas.join(",")

}

fn crear_consulta_update(condiciones_separadas: Vec<String>, direccion_archivo: String) -> Result<UpdateSql, SQLError> {
    
    let tabla_de_consulta = archivo(&condiciones_separadas[0], &direccion_archivo)?;
    /*let mut tabla_de_consulta: String = direccion_archivo.to_string();
    tabla_de_consulta.push_str("/");
    tabla_de_consulta.push_str(&condiciones_separadas[0].replace(";", ""));
    tabla_de_consulta.push_str(".csv");
    if !Path::new(&tabla_de_consulta).exists() {
        return Err(SQLError::new("INVALID_TABLE"));
    } */
    
    let set_clause = extraer_set(&condiciones_separadas[1]);

    let condicion = if condiciones_separadas.len() > 2 {
        &condiciones_separadas[2].replace(";", "").replace(",", " AND ")
    } else {
        "" // No hay condiciones
    };
    let condiciones_logicas = parciar_condiciones_logicas(condicion);
    Ok(UpdateSql {
        tabla: tabla_de_consulta,
        set: set_clause,
        where_conditions: Some(condiciones_logicas),
    })
}

fn extraer_set(consulta_set: &str) -> Vec<(String, String)> {
    let mut set_clause = vec![];
    let sets: Vec<&str> = consulta_set.trim().split(',').collect();
    for set in sets {
        let partes: Vec<&str> = set.split('=').collect();
        if partes.len() == 2 {
            set_clause.push((partes[0].trim().to_string(), partes[1].trim().to_string()));
        }
    }
    set_clause
}