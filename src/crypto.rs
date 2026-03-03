/*
    crypto.rs
    * Esse módulo contém as funções de criptografia e descriptografia
    * para o método de Vigenere (XOR) e para OTP (matematicamente inquebrável)
*/

pub fn encrypt(text: &String, key: &String) -> String {
    // xor com vigenere
    let mut msg_completa: String = String::new();
    let msg = text.trim().to_string();

    let txtlen = msg.len();

    for tamanho_chave in 0..txtlen {
        let valor = msg.as_bytes()[tamanho_chave] ^ key.as_bytes()[tamanho_chave % key.len()];
        let hex = format!("{:02x}", valor);
        /* println!(
            "{}: {} ^ {} = {}",
            tamanho_chave,
            msg.as_bytes()[tamanho_chave],
            key.as_bytes()[tamanho_chave % key.len()],
            valor
        ); */
        msg_completa.push_str(&hex);
    }
    return msg_completa;
}

pub fn decrypt(cypher: String, key: &String) -> String {
    let mut msg_completa: String = String::new();
    let cypher = cypher.trim().to_string();

    let txtlen = cypher.len() / 2;

    for tamanho_chave in 0..txtlen {
        // estranho para ler, mas é o intervalo entre x..y, para pegar os slices com 2 digitos
        let hex_str = &cypher[(2 * tamanho_chave)..(2 * tamanho_chave + 2)];
        let valor =
            u8::from_str_radix(hex_str, 16).unwrap() ^ key.as_bytes()[tamanho_chave % key.len()];
        msg_completa.push(valor as char);
    }
    return msg_completa;
}

pub fn otp_encrypt(message: String) -> Result<(String, String), getrandom::Error> {
    // usa a crate getrandom para gerar uma chave aleatória do mesmo tamanho da mensagem
    // a sintaxe não é muito amigável, mas é isso aí, funciona.
    let mut key = vec![0u8; message.as_bytes().len()];
    getrandom::fill(&mut key)?;

    // isso é o processo de XOR entre a mensagem e a chave
    let cipher = message
        .as_bytes()
        .iter()
        .zip(key.iter())
        .map(|(m, k)| m ^ k)
        .collect::<Vec<u8>>();

    // formatação para leitura em hex
    let cipher_hex: String = cipher.iter().map(|b| format!("{:02x}", b)).collect();
    let key_hex: String = key.iter().map(|b| format!("{:02x}", b)).collect();

    Ok((cipher_hex, key_hex))
}

pub fn otp_decrypt(cipher_hex: String, key_hex: String) -> Result<String, String> {
    // converte os hex de volta para bytes
    let cipher_bytes = (0..cipher_hex.len())
        .step_by(2)
        .map(|tamanho_chave| {
            u8::from_str_radix(&cipher_hex[tamanho_chave..tamanho_chave + 2], 16).unwrap()
        })
        .collect::<Vec<u8>>();

    let key_bytes = (0..key_hex.len())
        .step_by(2)
        .map(|tamanho_chave| {
            u8::from_str_radix(&key_hex[tamanho_chave..tamanho_chave + 2], 16).unwrap()
        })
        .collect::<Vec<u8>>();

    if cipher_bytes.len() != key_bytes.len() {
        return Err("Tamanhos diferentes, inválido!".to_string());
    }

    // processo de XOR para descriptografar
    let message_bytes = cipher_bytes
        .iter()
        .zip(key_bytes.iter())
        .map(|(c, k)| c ^ k)
        .collect::<Vec<u8>>();

    Ok(String::from_utf8(message_bytes).unwrap())
}

pub fn break_vigenere(cypher: &String) {
    for tamanho_chave in 1..20 {
        // tenta chaves de 1 a 20 caracteres
        println!("============================");
        println!("Tamanho da chave: {}", tamanho_chave);

        for num_conjunto in 0..tamanho_chave {
            let mut array_ocorencias = [0u32; 256]; //array de 256 posicoes (todas em 0 do tipo u32)
            let mut maior_quantidade = 0;
            let mut maior_letra = 0;

            for i in (0..cypher.len()).step_by(2) {
                //lendo de 2 em 2 (hex)
                if ((i / 2) % tamanho_chave) == num_conjunto {
                    let letra = u8::from_str_radix(&cypher[(i)..(i + 2)], 16).unwrap();
                    array_ocorencias[letra as usize] += 1; //incrementa a quantidade de ocorrencias da letra

                    #[allow(unused_parens)]
                    if (array_ocorencias[letra as usize] > maior_quantidade) {
                        maior_quantidade = array_ocorencias[letra as usize];
                        maior_letra = letra;
                    }
                }
            }
            print!("{}", (char::from_u32((maior_letra ^ 32) as u32).unwrap()));
        }
        println!();
    }
}
