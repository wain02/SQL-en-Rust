use crate::parciar::{regex_casero, evaluar_condiciones_logicas, parciar_condiciones_logicas};//parse_operadores
use crate::sql_predicate::SqlCondicionesLogicas;
use crate::errores::SQLError;
use crate::sql_conditions::SqlSelect;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader};
use crate::manejo_archivos::archivo;



#[derive(Debug)]
pub struct SelectSql{
    select: Vec<String>,
    tabla: String,
    where_conditions: Option<SqlCondicionesLogicas>,
    order_by: Option<OrderBy>,
}


#[derive(Debug)]
pub struct OrderBy {
    columna: String,
    orden: Orden,
}

#[derive(Debug)]
pub enum Orden {
    Asc,
    Desc,
}





///Recibe la consulta select de SQL y el path de los archivos CSV.
pub fn comando_select(consulta_del_terminal: String, direccion_archivo: String) -> Result<(), SQLError> {
    //SELECT id, producto, id_cliente FROM ordenes WHERE cantidad > 1;
    if !consulta_del_terminal.contains("SELECT") || !consulta_del_terminal.contains("FROM") {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    }

    let mut consulta_principal = consulta_del_terminal.trim().to_string();
    let mut order_by_clau = None;

    if let Some((principal, order_by)) = consulta_principal.split_once("ORDER BY"){
        let mut order_by_string = order_by.trim().to_string();
        order_by_clau = Some(extraer_order_by(&mut order_by_string)?);
        consulta_principal = principal.trim().to_string();
    }
     
 

    let condiciones_separadas = regex_casero(&consulta_principal, vec!["WHERE", "SELECT", "FROM"]);

    if condiciones_separadas.len() < 2 {
        let error = SQLError::new("INVALID_SYNTAX");
        println!("Error: {}", error);
        return Err(error);
    }

    let consulta_sql = crear_consulta_select(condiciones_separadas, order_by_clau, direccion_archivo)?;

    let _ = select_csv(consulta_sql);
    Ok(())

}

///Recibe la consulta SQL parciada en el struct SelectSql
///Se encarga de selccionar las columnas en el archivo CSV
pub fn select_csv(consulta: SelectSql) -> io::Result<()> {
    let mut rows = Vec::new();
    let input = BufReader::new(fs::File::open(&consulta.tabla)?);
    let mut lineas = input.lines();
    let mut index_vector_consulta = Vec::new();
    let mut index_condiciones = Vec::new();
    let mut order_by_index = None;


    if let Some(Ok(header)) = &lineas.next() {
        let columnas_archivo: Vec<&str> = header.split(',').collect();
        for (i, columna) in columnas_archivo.iter().enumerate() {
            let columna = columna.trim().to_string();
            if consulta.select.is_empty() || consulta.select.contains(&columna) {
                index_vector_consulta.push(i);
            }
            if let Some(where_conditions) = &consulta.where_conditions {
                for cond in &where_conditions.conditions {
                    if cond.columna.trim() == columna {
                        index_condiciones.push((i, cond));
                    }
                }
            }

            if let Some(order_by) = &consulta.order_by {
                if order_by.columna == columna {
                    order_by_index = Some(i);
                }
            }
        }
    }


    filtrar_filas(&mut lineas, &mut rows, &consulta, &index_vector_consulta, &index_condiciones)?;
    ordenar_filas(&mut rows, order_by_index, &consulta.order_by);
    escribir_resultado(&mut rows, &index_vector_consulta)?;


    Ok(())
}

///Recibe las lineas del archivo, las filtra y las guarda en un vector.
fn filtrar_filas(
    lineas: &mut std::io::Lines<BufReader<fs::File>>,
    rows: &mut Vec<String>,
    consulta: &SelectSql,
    index_vector_consulta: &Vec<usize>,
    index_condiciones: &Vec<(usize, &SqlSelect)>,
) -> io::Result<()> {
    for linea_actual in lineas {
        let line = linea_actual?;
        let columnas: Vec<&str> = line.split(',').collect();
        

        match &consulta.where_conditions {
            Some(where_conditions) if !where_conditions.conditions.is_empty() => {
                if evaluar_condiciones_logicas(&columnas, index_condiciones, where_conditions) {
                    rows.push(line);
                }
            }
            _ => {
                // Si no hay condiciones WHERE, o están vacías, se añade la línea
                rows.push(line);
            }
        }    
    }
    Ok(())
}
///Se encarga orden el vector rows
fn ordenar_filas(rows: &mut Vec<String>, order_by_index: Option<usize>, order_by: &Option<OrderBy>) {
    
    if let (Some(index), Some(order_by_value)) = (order_by_index, order_by.as_ref()) {
        rows.sort_by(|a, b| {
            let a_columnas: Vec<&str> = a.split(',').collect();
            let b_columnas: Vec<&str> = b.split(',').collect();
            match order_by_value.orden {
                Orden::Asc => a_columnas[index].cmp(&b_columnas[index]),
                Orden::Desc => b_columnas[index].cmp(&a_columnas[index]),
            }
        });
    }

}
///Recibe las filas filtradas y las columnas seleccionadas.
fn escribir_resultado(
    rows: &mut Vec<String>,
    index_vector_consulta: &Vec<usize>,
) -> io::Result<()> {
    
    for line in rows {    
        let columnas: Vec<&str> = line.split(',').collect();
        let columnas_seleccionadas: Vec<&str> = index_vector_consulta.iter().map(|&i| columnas[i]).collect(); 
        println!("{}", columnas_seleccionadas.join(","));
        //writeln!(archivo_output, "{}", columnas_seleccionadas.join(","))?;
    }
    Ok(())
}

///Recibe las condiciones separadas, la clausula ORDER BY y el path de los archivos CSV.
/// Parcea y guarda la informacion en el struct SelectSql
fn crear_consulta_select(condiciones_separadas: Vec<String>, order_by_clause: Option<OrderBy>, direccion_archivo: String) -> Result<SelectSql, SQLError> {
    
    let tabla_de_consulta = archivo(&condiciones_separadas[1].to_string(), &direccion_archivo.to_string())?;

    let selected_columns = extraer_columnas(&condiciones_separadas[0]);

    let condicion = if condiciones_separadas.len() > 2 {
        &condiciones_separadas[2]
    } else {
        "" // No hay condiciones
    };
    let condiciones_logicas = parciar_condiciones_logicas(condicion);

    Ok(SelectSql {
        select: selected_columns,
        tabla: tabla_de_consulta,
        where_conditions: Some(condiciones_logicas),
        order_by: order_by_clause,
    })
}
///Recibe la clausula SELECT y la divide en columnas.
fn extraer_columnas(consulta_seleccionada: &str) -> Vec<String> {
    let mut columnas_seleccionadas = vec![];
    let columnas: Vec<&str> = consulta_seleccionada.trim().split_whitespace().collect();
    if !columnas.contains(&"*") {
        for col in columnas {
            columnas_seleccionadas.push(col.to_string().replace(",", ""));
        }
    }
    columnas_seleccionadas
}

///Recibe la clausula ORDER BY y la divide en columna y orden.
fn extraer_order_by(order_by: &mut String) -> Result<OrderBy, SQLError> {
    let partes: Vec<&str> = order_by.trim().split_whitespace().collect();
    if partes.len() == 2 {
        let orden = match partes[1].replace(";","").to_uppercase().as_str() {
            "ASC" => Orden::Asc,
            "DESC" => Orden::Desc,
            _ => return Err(SQLError::new("INVALID_ORDER")),
        };
        return Ok(OrderBy {
            columna: partes[0].to_string(),
            orden,
        });
    }else {
        return Err(SQLError::new("INVALID_ORDER_FORMAT"));
    }
    
    
}







