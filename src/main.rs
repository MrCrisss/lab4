#![no_std]
#![no_main]

mod memoria;
mod vga_buffer;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("so1_kernel UMG - Asignacion de memoria simple");

    let mut m = memoria::Memoria::new();

    m.mostrar();

    // 1. Tres procesos solicitan memoria.
    let a = m.reservar(b'A', 40).unwrap();
    let b = m.reservar(b'B', 100).unwrap();
    let c = m.reservar(b'C', 20).unwrap();

    m.mostrar();

    // 2. Escribir y leer datos.
    m.escribir(a, 0, b"Hola UMG").unwrap();

    println!(
    "A contiene: {}",
    core::str::from_utf8(m.leer(a, 0, 8).unwrap()).unwrap()
);

match m.leer(a, 0, 49) {
    Ok(datos) => println!("Lectura aceptada: {:?}", datos),
    Err(e) => println!("Lectura de A rechazada: {}", e),
}

    // 3. C intenta escribir fuera de su region.
    match m.escribir(c, 30, b"desborde!") {
        Ok(_) => println!("Escritura en C aceptada"),
        Err(e) => println!("Escritura en C rechazada: {}", e),
    }

    // 4. Liberar B y reutilizar su espacio.
    m.liberar(b);

    m.mostrar();

    let _d = m.reservar(b'D', 50);
    let _e = m.reservar(b'E', 700);

    m.mostrar();

    // 5. Intentar reservar memoria para F.
    let _f = m.reservar(b'F', 150);

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

    // 5. Intentar reservar memoria para F sin compactar.
    println!("--- INTENTO DE RESERVA DE F ---");

    let _f = m.reservar(b'F', 150);

    m.mostrar();

    // 6. Compactar la memoria.
    println!("--- COMPACTACION ---");

    m.compactar();

    m.mostrar();

    // 7. Intentar reservar F nuevamente.
    println!("--- RESERVA DE F DESPUES DE COMPACTAR ---");

    let _f = m.reservar(b'F', 150);

    m.mostrar();

    // 8. Probar la estrategia de mejor ajuste.
    println!("--- PRUEBA DE MEJOR AJUSTE ---");

    let mut m2 = memoria::Memoria::new();

    let _a2 = m2.reservar(b'A', 40).unwrap();
    let b2 = m2.reservar(b'B', 100).unwrap();
    let _c2 = m2.reservar(b'C', 20).unwrap();

    m2.mostrar();

    // Liberar B.
    m2.liberar(b2);

    m2.mostrar();

    // Reservar D y E usando mejor ajuste.
    let _d2 = m2.reservar_mejor_ajuste(b'D', 50);
    let _e2 = m2.reservar_mejor_ajuste(b'E', 700);

    m2.mostrar();

    // Intentar reservar F usando mejor ajuste.
    let _f2 = m2.reservar_mejor_ajuste(b'F', 150);

    m2.mostrar();

    loop {}
