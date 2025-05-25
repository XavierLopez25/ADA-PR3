use std::fs::OpenOptions;
use std::io::{self, Write};


// Función helper que imprime por pantalla y graba en el archivo
fn log_line<W: Write>(writer: &mut W, s: &str) {
    println!("{}", s);
    writeln!(writer, "{}", s).unwrap();
}

// Ejercicio 1: MTF con lista fija de [0,1,2,3,4] y secuencia repetida
fn ejercicio1<W: Write>(writer: &mut W) {
    let mut list = vec![0, 1, 2, 3, 4];
    let requests = vec![
        0,1,2,3,4,
        0,1,2,3,4,
        0,1,2,3,4,
        0,1,2,3,4,
    ];
    let mut total_cost = 0;
    log_line(writer,"\n=== Ejercicio 1: MTF sobre [0,1,2,3,4] y 20 solicitudes ===");
    for req in requests {
        let before = format!("Lista antes:    {:?}", list);
        log_line(writer, &before);
        let cost = access_cost(&mut list, req);
        let after = format!("Solicitud: {:>2} | Costo: {:>2} | Lista después: {:?}\n",
                             req, cost, list);
        log_line(writer, &after);
        total_cost += cost;
    }
    log_line(writer, &format!("Costo total de todas las accesos: {}\n", total_cost));
    log_line(writer, "=========================\n");

}

fn ejercicio2<W: Write>(writer: &mut W) {
    log_line(writer,"\n=== Ejercicio 2: MTF sobre [0,1,2,3,4] y secuencia dada ===");
    // Configuración inicial
    let mut list = vec![0, 1, 2, 3, 4];
    // Secuencia de 17 solicitudes según el enunciado
    let requests = vec![
        4, 3, 2, 1, 0,
        1, 2, 3, 4,
        3, 2, 1, 0,
        1, 2, 3, 4,
    ];

    let mut total_cost = 0;
    for req in requests {
        // Estado antes del acceso
        let before = format!("Lista antes:    {:?}", list);
        log_line(writer, &before);        // Calcula costo y aplica MTF
        let cost = access_cost(&mut list, req);
        // Estado después
        let after = format!("Solicitud: {:>2} | Costo: {:>2} | Lista después: {:?}\n",
                             req, cost, list);
        log_line(writer, &after);
        total_cost += cost;
    }

    log_line(writer,&format!("Costo total de las accesos: {}\n", total_cost));
}

fn ejercicio3<W: Write>(writer: &mut W) {
    log_line(writer,"\n=== Ejercicio 3: secuencia de 20 solicitudes de costo mínimo ===");

    // Configuración inicial
    let mut list = vec![0, 1, 2, 3, 4];

    // La secuencia que minimiza el costo es siempre pedir el elemento al frente (inicialmente 0)
    let requests = vec![0; 20];

    // Calculamos el costo total (y aplicamos MTF para verificar)
    let mut total_cost = 0;
    for &req in &requests {
        total_cost += access_cost(&mut list, req);
    }

    // Mostramos resultados
    log_line(writer,&format!("Secuencia elegida (20 veces el mismo elemento al frente): {:?}", requests));
    log_line(writer,&format!("Costo total mínimo de acceso: {}\n", total_cost));
}

fn ejercicio4<W: Write>(writer: &mut W) {
    log_line(writer,"\n=== Ejercicio 4: secuencia de peor caso de 20 solicitudes ===");
    // Configuración inicial
    let mut list = vec![0, 1, 2, 3, 4];
    // La peor secuencia pide siempre el elemento en la última posición:
    // 4,3,2,1,0 repetido 4 veces → cada acceso cuesta 5 → total = 20*5 = 100
    let base_seq = vec![4, 3, 2, 1, 0];
    let mut requests = Vec::with_capacity(20);
    for _ in 0..4 {
        requests.extend(&base_seq);
    }

    // Calculamos el costo total
    let mut total_cost = 0;
    for &req in &requests {
        total_cost += access_cost(&mut list, req);
    }

    // Mostramos la secuencia y el costo máximo
    log_line(writer,&format!("Secuencia elegida (peor caso): {:?}", requests));
    log_line(writer,&format!("Costo total de acceso (peor caso): {}", total_cost));
}

fn ejercicio5<W: Write>(writer: &mut W) {
    log_line(writer,"\n=== Ejercicio 5: repetición de un mismo elemento 20 veces ===");

    // Parte a) 20 solicitudes de '2'
    log_line(writer,"\n-- Secuencia: veinte veces el elemento 2 --");
    let mut list = vec![0, 1, 2, 3, 4];
    let requests2 = vec![2; 20];
    let mut total2 = 0;

    for &req in &requests2 {
        let before = format!("Lista antes:    {:?}", list);
        log_line(writer, &before);
        let cost = access_cost(&mut list, req);
        let after = format!("Solicitud: {:>2} | Costo: {:>2} | Lista después: {:?}\n",
                             req, cost, list);
        log_line(writer, &after);
        total2 += cost;
    }
    log_line(writer,&format!("Costo total con 20 repeticiones de 2: {}\n", total2));

    // Parte b) ¿y si fueran veinte '3'?
    let mut list = vec![0, 1, 2, 3, 4];
    let requests3 = vec![3; 20];
    let mut total3 = 0;
    for &req in &requests3 {
        total3 += access_cost(&mut list, req);
    }
    log_line(writer,&format!("Costo total con 20 repeticiones de 3: {}\n", total3));

    // Observación del patrón
    log_line(writer,"Observación:");
    log_line(writer,
        "  Cuando repites 20 veces el mismo elemento x, el primer acceso cuesta \
         su posición inicial (p) y los 19 restantes cuestan 1 (por estar ya en front).\n\
         Así, C_total = p + 19·1. Por ejemplo:\n\
         - Para x=2, p=3 ⇒ C=3+19=22\n\
         - Para x=3, p=4 ⇒ C=4+19=23\n"
    );
}


/// Aplica una sola petición con IMTF:
/// - `list`: lista actual mutable
/// - `req`: elemento solicitado
/// - `seq`: secuencia completa de peticiones
/// - `idx`: índice (0-based) de esta petición en `seq`
/// Devuelve el costo de acceso (posición 1-based).
fn imtf_access(list: &mut Vec<usize>, req: usize, seq: &[usize], idx: usize) -> usize {
    // posición 0-based en la lista
    let pos = list.iter()
                  .position(|&x| x == req)
                  .expect("¡Elemento no existe en la lista!");
    let cost = pos + 1;           // 1-based

    // lookahead de i-1 = pos elementos
    let mut lookahead = seq.iter()
                       .skip(idx + 1)
                       .take(pos);
    // si aparece dentro de esos pos elementos, movemos a front
    if lookahead.any(|&x| x == req) {
        list.remove(pos);
        list.insert(0, req);
    }
    cost
}

fn ejercicio6<W: Write>(writer: &mut W) {
    log_line(writer,"\n=== Ejercicio 6: IMTF sobre secuencias de mejor/peor caso MTF ===");

    // 1) mejor caso MTF: 20 veces el mismo elemento al frente (0)
    let seq_best = vec![0; 20];

    // 2) peor caso MTF: [4,3,2,1,0] repetido 4 veces
    let mut seq_worst = Vec::with_capacity(20);
    for _ in 0..4 {
        seq_worst.extend(&[4, 3, 2, 1, 0]);
    }

    for (label, seq) in &[("Mejor caso MTF", &seq_best), ("Peor caso MTF", &seq_worst)] {
        log_line(writer,&format!("\n-- {} ({} solicitudes) --", label, seq.len()));
        let mut list = vec![0, 1, 2, 3, 4];
        let mut total = 0;

        for (i, &req) in seq.iter().enumerate() {
            let before = format!("Lista antes:    {:?}", list);
            log_line(writer, &before);            
            let cost = imtf_access(&mut list, req, seq, i);
            let after = format!("Solicitud: {:>2} | Costo: {:>2} | Lista después: {:?}\n",
                             req, cost, list);
            log_line(writer, &after);
            total += cost;
        }

        log_line(writer,&format!("Costo total IMTF en {}: {}\n", label, total));
    }
}

// Función de costo y MTF reusada
fn access_cost(list: &mut Vec<usize>, request: usize) -> usize {
    if let Some(pos) = list.iter().position(|&x| x == request) {
        let cost = pos + 1;
        list.remove(pos);
        list.insert(0, request);
        cost
    } else {
        panic!("Solicitud {} no encontrada en la lista", request);
    }
}

fn main() {


    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("salida.txt")
        .expect("No pude abrir salida.txt");


    loop {
        log_line(&mut log_file,"Seleccione una opción:");
        log_line(&mut log_file,"1) Ejercicio 1");
        log_line(&mut log_file,"2) Ejercicio 2");
        log_line(&mut log_file,"3) Ejercicio 3");
        log_line(&mut log_file,"4) Ejercicio 4");
        log_line(&mut log_file,"5) Ejercicio 5");
        log_line(&mut log_file,"6) Ejercicio 6");
        log_line(&mut log_file,"7) Salir\n");
        print!("Opción: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => ejercicio1(&mut log_file),
            2 => ejercicio2(&mut log_file),
            3 => ejercicio3(&mut log_file),
            4 => ejercicio4(&mut log_file),
            5 => ejercicio5(&mut log_file),
            6 => ejercicio6(&mut log_file),
            7 => {
                log_line(&mut log_file,"Saliendo...");
                break;
            }
            _ => log_line(&mut log_file,"Opción inválida, intente de nuevo.\n"),
        }
    }
}
