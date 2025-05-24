use std::io::{self, Write};

// Ejercicio 1: MTF con lista fija de [0,1,2,3,4] y secuencia repetida
fn ejercicio1() {
    let mut list = vec![0, 1, 2, 3, 4];
    let requests = vec![
        0,1,2,3,4,
        0,1,2,3,4,
        0,1,2,3,4,
        0,1,2,3,4,
    ];
    let mut total_cost = 0;
    println!("\n=== Ejercicio 1: MTF sobre [0,1,2,3,4] y 20 solicitudes ===");
    for req in requests {
        println!("Lista antes:    {:?}", list);
        let cost = access_cost(&mut list, req);
        println!("Solicitud: {:>2} | Costo: {:>2} | Lista después: {:?}\n", req, cost, list);
        total_cost += cost;
    }
    println!("Costo total de todas las accesos: {}\n", total_cost);
    println!("=========================\n");

}

fn ejercicio2() {
    println!("\n=== Ejercicio 2: MTF sobre [0,1,2,3,4] y secuencia dada ===");
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
        println!("Lista antes:    {:?}", list);
        // Calcula costo y aplica MTF
        let cost = access_cost(&mut list, req);
        // Estado después
        println!(
            "Solicitud: {:>2} | Costo: {:>2} | Lista después: {:?}\n",
            req, cost, list
        );
        total_cost += cost;
    }

    println!("Costo total de las accesos: {}\n", total_cost);
}

fn ejercicio3() {
    println!("\n=== Ejercicio 3: secuencia de 20 solicitudes de costo mínimo ===");

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
    println!("Secuencia elegida (20 veces el mismo elemento al frente): {:?}", requests);
    println!("Costo total mínimo de acceso: {}\n", total_cost);
}

fn ejercicio4() {
    println!("Ejercicio 4 aún no implementado.\n");
}
fn ejercicio5() {
    println!("Ejercicio 5 aún no implementado.\n");
}
fn ejercicio6() {
    println!("Ejercicio 6 aún no implementado.\n");
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
    loop {
        println!("Seleccione una opción:");
        println!("1) Ejercicio 1");
        println!("2) Ejercicio 2");
        println!("3) Ejercicio 3");
        println!("4) Ejercicio 4");
        println!("5) Ejercicio 5");
        println!("6) Ejercicio 6");
        println!("7) Salir\n");
        print!("Opción: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => ejercicio1(),
            2 => ejercicio2(),
            3 => ejercicio3(),
            4 => ejercicio4(),
            5 => ejercicio5(),
            6 => ejercicio6(),
            7 => {
                println!("Saliendo...");
                break;
            }
            _ => println!("Opción inválida, intente de nuevo.\n"),
        }
    }
}
