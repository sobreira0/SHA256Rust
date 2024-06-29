use sha2::{Sha256, Digest};
use std::time::Instant;
use base16ct;
use std::thread;
use std::{fs, io::{Write, BufWriter}};

pub fn write_data_to_file(path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::OpenOptions::new() 
        .create(true) 
        .append(true) 
        .open(&path)?;

    let mut file = BufWriter::new(file);

    file.write_all(&data.as_bytes())?;

    file.flush()?;

    Ok(())
}

pub fn main_func(input: String, zeros: usize) -> (String, String, u128) {
    let mut buf: [u8; 512] = [0u8; 512];
    let mut attempts: u128 = 1;
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

    let formatted_data = input+&attempts.to_string();
    match write_data_to_file("./Hash.txt", &format!("\nHash: {hash}\nPre Imagem final: {}\n", formatted_data)) {
        Ok(_) => println!("Arquivo escrito com sucesso!"),
        Err(_) => println!("Erro ao escrever oa rquivo"),
    }
    println!("Hash: {}", hash);
    println!("Numero de tentativas: {}", attempts);
    println!("Tempo de execucao: {}", time.as_secs());
    println!("Pre Imagem final: {}", formatted_data);
}

fn main() {
    let mut handles = vec![];

    for i in 0..11 {
        let handle = thread::spawn( move || {
                println!("Thread {} rodando", i);
                match i {
                    10 => rodar(String::from("")),
                    _ => rodar(String::from(i.to_string())),
                }
            }
        );
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
