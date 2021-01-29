#[macro_use]
extern crate serde_derive;

use std::io;
use std::io::Write;
use std::process;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Ingresa el nombre del minero: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);
    print!("Dificultad: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("Debe ser un entero");
    print!("generando el bloque Genesis! ");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) Nueva Transacción");
        println!("2) Minar bloque");
        println!("3) Cambiar dificultad");
        println!("4) Cambiar recompensa");
        println!("0) Salir");
        print!("Ingresa un numero: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!();

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Saliendo...");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut reciever = String::new();
                let mut amount = String::new();

                print!("Ingresa la dirección remitente: ");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);
                print!("Ingresa la dirección del receptor: ");
                io::stdout().flush();
                io::stdin().read_line(&mut reciever);
                print!("Ingresa la cantidad: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    reciever.trim().to_string(),
                    amount.trim().parse().unwrap());
                match res {
                    true => println!("transacción añadida "),
                    false => println!("transacción fallida "),
                }
            }
            2 => {
                println!("Generando bloque...");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Bloque generado"),
                    false => println!("Error al generar el Bloque"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                print!("Ingresa una nueva dificultad: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Dificultad actualizada"),
                    false => println!("Error al actualizar la dificultad"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("Ingresa una nueva recompensa: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Recompensa actualizada"),
                    false => println!("Error al actualizar la recompensa"),
                }
            }
            _ => println!("Opción invalida, Reintentar"),
        }
    }
}
