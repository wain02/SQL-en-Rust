use std::fs::OpenOptions;
pub fn comando_insert(consulta_inst_terminal: String) {
    let consulta_inst = consulta_inst_terminal
        .replace(",", "")
        .replace("\n", " ")
        .replace(";", "")
        .replace("(", "")
        .replace(")", "")
        .replace("'", "");
        
    let division_values: Vec<&str> = consulta_inst.trim().split("VALUES").collect(); // Divide la cadena en partes
    let previo_values = division_values[0];
    let posterior_values = division_values[1];
    let valores_insert: Vec<&str> = posterior_values.trim().split_whitespace().collect(); //[111, 6, 'Laptop', 3]
    let mut columas_insert: Vec<&str> = previo_values.trim().split_whitespace().collect(); //[id, id_cliente, producto, cantidad]

    columas_insert.remove(0);
    columas_insert.remove(0);
    let tabla_insert = columas_insert[0];
    columas_insert.remove(0);

    write_csv(valores_insert, tabla_insert.to_string());
} 

pub fn write_csv(insert: Vec<&str>, tabla_insert: String) {
    let mut tabla_csv = String::from(tabla_insert);
    tabla_csv.push_str(".csv");
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(tabla_csv)
        .unwrap();

    let mut wtr = csv::Writer::from_writer(file);
    let _ = wtr.write_record(&insert); //let _ = que no me importa el resultado.
    
} 

