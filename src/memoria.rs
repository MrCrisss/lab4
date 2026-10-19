// Administrador de memoria simple.
// RAM simulada de 1 KiB dividida en 64 bloques de 16 bytes.

use crate::println;

pub const TAM_BLOQUE: usize = 16;
pub const NUM_BLOQUES: usize = 64;

pub struct Memoria {
    datos: [u8; TAM_BLOQUE * NUM_BLOQUES],
    dueno: [u8; NUM_BLOQUES],
    largo: [usize; NUM_BLOQUES],
}

impl Memoria {
    pub fn new() -> Self {
        Memoria {
            datos: [0; TAM_BLOQUE * NUM_BLOQUES],
            dueno: [b'.'; NUM_BLOQUES],
            largo: [0; NUM_BLOQUES],
        }
    }

    // Reserva memoria usando primer ajuste.
    pub fn reservar(&mut self, nombre: u8, bytes: usize) -> Option<usize> {
        let n = (bytes + TAM_BLOQUE - 1) / TAM_BLOQUE;
        let mut inicio = 0;

        while inicio + n <= NUM_BLOQUES {
            if self.dueno[inicio..inicio + n]
                .iter()
                .all(|&d| d == b'.')
            {
                for i in inicio..inicio + n {
                    self.dueno[i] = nombre;
                }

                self.largo[inicio] = n;

                println!(
                    "{} pide {} bytes -> bloques {}..{} (desperdicio interno: {} bytes)",
                    nombre as char,
                    bytes,
                    inicio,
                    inicio + n - 1,
                    n * TAM_BLOQUE - bytes
                );

                return Some(inicio);
            }

            inicio += 1;
        }

        println!(
            "{} pide {} bytes -> NO HAY ESPACIO CONTIGUO",
            nombre as char,
            bytes
        );

        None
    }

pub fn reservar_mejor_ajuste(
    &mut self,
    nombre: u8,
    bytes: usize,
) -> Option<usize> {
    if bytes == 0 {
        return None;
    }

    let n = (bytes + TAM_BLOQUE - 1) / TAM_BLOQUE;

    if n > NUM_BLOQUES {
        println!("{} pide {} bytes -> NO HAY ESPACIO", nombre as char, bytes);
        return None;
    }

    let mut mejor_inicio = None;
    let mut mejor_tamano = NUM_BLOQUES + 1;
    let mut inicio = 0;

    while inicio < NUM_BLOQUES {
        if self.dueno[inicio] == b'.' {
            let mut fin = inicio;

            while fin < NUM_BLOQUES && self.dueno[fin] == b'.' {
                fin += 1;
            }

            let tamano_hueco = fin - inicio;

            if tamano_hueco >= n && tamano_hueco < mejor_tamano {
                mejor_inicio = Some(inicio);
                mejor_tamano = tamano_hueco;
            }

            inicio = fin;
        } else {
            inicio += 1;
        }
    }

    match mejor_inicio {
        Some(pos) => {
            for i in pos..pos + n {
                self.dueno[i] = nombre;
            }

            self.largo[pos] = n;

            println!(
                "{} pide {} bytes -> bloques {}..{} (desperdicio interno: {} bytes)",
                nombre as char,
                bytes,
                pos,
                pos + n - 1,
                n * TAM_BLOQUE - bytes
            );

            Some(pos)
        }
        None => {
            println!(
                "{} pide {} bytes -> NO HAY ESPACIO CONTIGUO",
                nombre as char,
                bytes
            );

            None
        }
    }
}	

    // Libera los bloques de una reserva.
    pub fn liberar(&mut self, inicio: usize) {
        let n = self.largo[inicio];
        let nombre = self.dueno[inicio];

        for i in inicio..inicio + n {
            self.dueno[i] = b'.';
        }

        self.largo[inicio] = 0;

        println!("{} libera {} bloques", nombre as char, n);
    }

    // Escribe datos sin permitir que excedan el limite.
    pub fn escribir(
        &mut self,
        inicio: usize,
        desp: usize,
        datos: &[u8],
    ) -> Result<(), &'static str> {
        let limite = self.largo[inicio] * TAM_BLOQUE;

        if desp + datos.len() > limite {
            return Err("fuera de limites");
        }

        let base = inicio * TAM_BLOQUE + desp;

        self.datos[base..base + datos.len()].copy_from_slice(datos);

        Ok(())
    }

    // Lee una cantidad de bytes.
    pub fn leer(
    &self,
    inicio: usize,
    desp: usize,
    len: usize,
) -> Result<&[u8], &'static str> {
    if inicio >= NUM_BLOQUES {
        return Err("inicio fuera de limites");
    }

    let limite = self.largo[inicio] * TAM_BLOQUE;

    if limite == 0 {
        return Err("region no asignada");
    }

    if desp > limite || len > limite - desp {
        return Err("lectura fuera de limites");
    }

    let base = inicio * TAM_BLOQUE + desp;

    Ok(&self.datos[base..base + len])
}

    // Muestra el mapa de memoria.
    pub fn mostrar(&self) {
        let mut libres = 0;
        let mut hueco = 0;
        let mut mayor = 0;

        for &d in self.dueno.iter() {
            if d == b'.' {
                libres += 1;
                hueco += 1;

                if hueco > mayor {
                    mayor = hueco;
                }
            } else {
                hueco = 0;
            }
        }

        println!("[{}]", core::str::from_utf8(&self.dueno).unwrap());

        println!(
            "  libres: {} bloques ({} bytes), hueco mayor: {} bloques",
            libres,
            libres * TAM_BLOQUE,
            mayor
        );
    }
}
