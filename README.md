# Proyecto MTF / IMTF en Rust

Este proyecto implementa los algoritmos **Move-To-Front (MTF)** e **Improved Move-To-Front (IMTF)** en Rust, y responde a seis ejercicios de análisis de costos de acceso en una lista auto-organizada. El programa ofrece un menú interactivo por consola y registra toda la salida en un archivo `salida.txt`.

---

## Requisitos

- Rust (1.60 o superior) y Cargo instalados  
  Instálalo con [rustup](https://rustup.rs/):  
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

* (Opcional) MSYS2 / MinGW en Windows para enlaces GNU

---

## Compilar

En la raíz del proyecto, ejecuta:

```bash
# Debug (rápido, con símbolos de depuración)
cargo build

# Release (más optimizado)
cargo build --release
```

Los binarios se generarán en `target/debug/` o `target/release/`.

---

## Ejecutar

```bash
cargo run
```

O bien, si compilaste en modo release:

```bash
cargo run --release
```

Cuando inicie, verás un menú como éste:

```
Seleccione una opción:
1) Ejercicio 1
2) Ejercicio 2
3) Ejercicio 3
4) Ejercicio 4
5) Ejercicio 5
6) Ejercicio 6
7) Salir

Opción:
```

* Teclea el número del ejercicio (1–6) y presiona **Enter** para verlo en pantalla.
* Teclea **7** para salir del programa.

---

## Registro de salida

Todas las líneas que aparecen en consola también se append-an a un archivo llamado `salida.txt` en la carpeta de ejecución.
Cada vez que corras el programa, los resultados se añadirán al final de dicho archivo.

---

## Estructura de carpetas

```
.
├── Cargo.toml          # Metadatos de Cargo
├── src
│   └── main.rs         # Código fuente principal
└── salida.txt          # Archivo de log (se crea al ejecutar)
```

* `main.rs` define:

  * `access_cost()` para MTF
  * `imtf_access()` para IMTF
  * Seis funciones `ejercicio1()`…`ejercicio6()` que cubren cada enunciado.
  * Menú interactivo en `main()` y helper `log_line()` para consola + archivo.

---
