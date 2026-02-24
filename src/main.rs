mod crypto;
use std::io::Write;
// src/crypto.rs para o código das funções com "crypto::"

fn main() {
    loop {
        let mut text = String::new();
        let mut key = String::new();
        let mut option_str: String = String::new();
        println!("Escolha uma opção:");
        println!("1. Criptografar com sua própria chave (Vigenère)");
        println!("2. Descriptografar com sua própria chave (Vigenère)");
        println!("3. Criptografar com OTP (One-Time Pad)");
        println!("4. Descriptografar com OTP (One-Time Pad)");
        println!("5. Sair");
        print!("Opção: ");
        std::io::stdout().flush().unwrap(); // garante que o prompt aparece

        std::io::stdin()
            .read_line(&mut option_str)
            .expect("Failed to read line");

        let option: u8 = option_str.trim().parse().expect("Failed to parse option");

        if option == 1 {
            println!("Digite a mensagem a ser criptografada: ");
            std::io::stdin() // lê o input
                .read_line(&mut text)
                .expect("Failed to read line");

            text = text.trim().to_string();

            println!("Digite a chave de criptografia: ");
            std::io::stdin() // lê o input
                .read_line(&mut key)
                .expect("Failed to read line");

            key = key.trim().to_string();

            let encrypted = crypto::encrypt(&text, &key);
            println!("Mensagem Criptografada: {}", encrypted);
            return;
        } else if option == 2 {
            println!("Digite a mensagem a ser descriptografada: ");
            std::io::stdin() // lê o input
                .read_line(&mut text)
                .expect("Failed to read line");

            text = text.trim().to_string();

            println!("Digite a chave de descriptografia: ");
            std::io::stdin() // lê o input
                .read_line(&mut key)
                .expect("Failed to read line");

            key = key.trim().to_string();

            let decrypted = crypto::decrypt(text, &key);
            println!("Mensagem Descriptografada: {}", decrypted);
            return;
        } else if option == 3 {
            println!("Digite a mensagem a ser criptografada: ");
            std::io::stdin() // lê o input
                .read_line(&mut text)
                .expect("Failed to read line");
            text = text.trim().to_string();
            let result = crypto::otp_encrypt(text);
            match result {
                Ok((cipher, key)) => {
                    println!("Mensagem Criptografada: {}", cipher);
                    println!("Chave de Criptografia (OTP): {}", key);
                }
                Err(e) => {
                    eprintln!("Erro ao gerar chave OTP: {}", e);
                }
            }
        } else if option == 4 {
            println!("Digite a mensagem criptografada em hexadecimal: ");
            std::io::stdin() // lê o input
                .read_line(&mut text)
                .expect("Failed to read line");
            text = text.trim().to_string();
            println!("Digite a chave OTP em hexadecimal: ");
            std::io::stdin() // lê o input
                .read_line(&mut key)
                .expect("Failed to read line");
            key = key.trim().to_string();
            let result = crypto::decrypt_otp(text, key);
            match result {
                Ok(message) => {
                    println!("Mensagem Descriptografada: {}", message);
                }
                Err(e) => {
                    eprintln!("Erro ao descriptografar: {}", e);
                }
            }
        } else if option == 5 {
            println!("Saindo...");
            break;
        } else {
            println!("Opção inválida!");
            continue;
        }
    }
}
