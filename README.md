# 🚀 Solana Zero to Hero #1 & #2

![Solana](https://img.shields.io/badge/Solana-362D59?style=for-the-badge&logo=solana&logoColor=white)
![Anchor](https://img.shields.io/badge/Anchor-000000?style=for-the-badge)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

Este repositório contém a entrega oficial do bounty **"Web3Experts - Solana Zero to Hero"**, ministrado por Kaue. O objetivo foi criar, testar, e fazer deploy do meu primeiro programa (Counter) na blockchain da Solana, puramente em Rust.

---

## 🎯 Resultados da Execução na Devnet

### 📜 Program ID Deployado
O contrato foi publicado com sucesso na Devnet da Solana:
> **`3NPBA3zn4Dc8FsXkqoGKXE1uERdch63TkGCcMTTDkVVT`**
🔗 **[Ver Contrato no Solana Explorer](https://explorer.solana.com/address/3NPBA3zn4Dc8FsXkqoGKXE1uERdch63TkGCcMTTDkVVT?cluster=devnet)**

### ⚡ Transações de Interação via CLI

A interação com o contrato também foi feita através de chamadas RPC e testes em Rust direto no CLI, garantindo maior integração e controle nativo sem necessitar de JavaScript.

1. **Initialize (Criando a conta PDA do Counter e setando 0):**
   - Assinatura: `3jGS7UsHwTnvPxUyAjZUaKVbk7B9bsU6KJm2c2ZoL5fcsaZFB6vdYjh51cvbGwHd5GAeJJ2DkWeSXXmKFJdt2etg`
   - 🔗 **[Ver Initialize no Solana Explorer](https://explorer.solana.com/tx/3jGS7UsHwTnvPxUyAjZUaKVbk7B9bsU6KJm2c2ZoL5fcsaZFB6vdYjh51cvbGwHd5GAeJJ2DkWeSXXmKFJdt2etg?cluster=devnet)**

2. **Increment (Adicionando +1 no valor do Counter):**
   - Assinatura: `53FykEpwK7Qz48ja8GZGqswmpDBNGasPyDG8EX7d7Ctu2LM5ZeDdnjmpYCkTsNBdGfkXddgCSKKtoz8LwxZBXose`
   - 🔗 **[Ver Increment no Solana Explorer](https://explorer.solana.com/tx/53FykEpwK7Qz48ja8GZGqswmpDBNGasPyDG8EX7d7Ctu2LM5ZeDdnjmpYCkTsNBdGfkXddgCSKKtoz8LwxZBXose?cluster=devnet)**

---

## 🛠️ Desafios Completados

- [x] Instalar o Solana CLI e Anchor no ambiente local
- [x] Gerar um novo keypair via CLI
- [x] Fazer Airdrop de SOL na devnet 
- [x] Executar a transferência de fundos via CLI para o vault do Bounty (`2Q6gMetRuJ2aqyU8neWbyq5p2gU5h3Q6WKAwWH7UzM1f`)
- [x] Criar um programa Anchor Counter com instruções: `initialize` e `increment`
- [x] Escrever testes validando o fluxo
- [x] Fazer o deploy do programa na Devnet
- [x] Interagir com o programa deployado

---

## 💻 Como Rodar e Testar

Para testar localmente, o repositório usa as ferramentas nativas de simulação da Solana (`litesvm`). Tudo está autocontido em Rust.

1. Instale dependências e compile o programa Anchor:
```bash
anchor build
```

2. Rode a suite de testes (incluindo local validator e testes na devnet):
```bash
cargo test -- --nocapture
```

Feito com ☕ e Rust.
