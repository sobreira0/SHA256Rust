use sha2::{Sha256, Digest};
use std::time::Instant;
use base16ct;
use std::thread;

pub fn main_func(input: String, zeros: usize) -> (String, String, u32) {
    let mut buf = [0u8; 64];
    let mut attempts: u32 = 1;
    let prefix: String = "0".repeat(zeros);
    let mut hasher = Sha256::new();
    let mut data: String = input.clone() + &attempts.to_string();
    hasher.update(data);
    let mut hash = hasher.clone().finalize();
    let mut hex_hash = base16ct::lower::encode_str(&hash, &mut buf).unwrap();

    while !hex_hash.starts_with(&prefix) {
        attempts = attempts + 1;
        data = input.clone() + &attempts.to_string();
        hasher.update(data);
        hash = hasher.clone().finalize();
        hex_hash = base16ct::lower::encode_str(&hash, &mut buf).unwrap();
    }

    return (input, hex_hash.to_string(), attempts);
}

pub fn rodar(entrada:String) {
    let start = Instant::now();
    let (input, hash, attempts) = main_func(entrada, 11);
    let end = Instant::now();
    let time = end - start;

    println!("Hash: {}", hash);
    println!("Numero de tentativas: {}", attempts);
    println!("Tempo de execucao: {}", time.as_secs());
    println!("Pre Imagem final: {}", input+&attempts.to_string());
}

fn main() {
    let mut handles = vec![];

    for i in 0..10 {
        let handle = thread::spawn( move || {
                println!("Thread {} rodando", i);
                rodar(String::from(i.to_string()));
            }
        );
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
