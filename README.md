# One-Time-Pad

Esse projeto é uma implementação de criptografia pelo algoritmo de Vigenère e da criptografia matematicamente inquebrável chamada de One-Time-Pad (OTP), onde a criptografia é feita por operações XOR da mensagem com a senha, mas, ao contrário do algoritmo de Vigenère, que repete a senha até que o tamanho seja o mesmo da mensagem, o OTP gera uma senha aleatória do mesmo tamanho da mensagem, usando valores confiavelmente aleatórios para criptografia, usando a crate **"getrandom"**.

## Para rodar 🦀

- Se você tem o Rust instalado, você pode clonar o repositório e rodar com o **cargo**.

```bash
git clone https://github.com/neozmmv/one-time-pad
```

```bash
cargo run
```

Ou você pode baixar o executável pelo [Release](https://github.com/neozmmv/one-time-pad/releases/) mais recente.
