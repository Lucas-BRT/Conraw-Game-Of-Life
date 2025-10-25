# Conway's Game of Life em Rust

Este repositório é uma exploração e implementação do clássico Jogo da Vida de Conway, escrito em Rust. O objetivo é criar uma série de implementações, começando pela mais simples e didática, e progredindo para versões mais complexas e otimizadas.

## Sobre o Jogo da Vida

O Jogo da Vida é um autômato celular desenvolvido pelo matemático britânico John Horton Conway em 1970. É um jogo de "zero jogadores", o que significa que sua evolução é determinada por seu estado inicial, não exigindo nenhuma entrada adicional. Interagimos com o Jogo da Vida criando uma configuração inicial e observando como ela evolui.

## Implementações

O projeto está organizado em etapas, cada uma em sua própria branch, representando uma versão diferente da implementação.

### 1. Versão Inicial (main)

A implementação atual na branch `main` é a mais simples de todas:

- **Renderização no Terminal:** O universo do Jogo da Vida é exibido diretamente no terminal.
- **Single-Thread:** Toda a lógica de atualização e renderização roda em uma única thread.
- **Estrutura de Dados Simples:** O universo é representado por uma matriz 2D de células.

Esta versão é um ponto de partida, focada em ter uma implementação funcional e fácil de entender.

### Próximas Versões (Planejadas)

As futuras implementações explorarão diferentes técnicas de otimização e renderização:

- **Multi-threading:** Utilizar múltiplas threads para paralelizar o cálculo do próximo estado do universo.
- **Otimização de Algoritmos:** Implementar algoritmos mais eficientes, como "HashLife".
- **Renderização Gráfica:** Criar uma interface gráfica usando bibliotecas como `bevy` ou `macroquad`.

## Como Executar

Para executar a implementação atual, você precisa ter o Rust instalado.

1.  **Clone o repositório:**

    ```bash
    git clone https://github.com/seu-usuario/Conraw-Game-Of-Life.git
    cd Conraw-Game-Of-Life
    ```

2.  **Execute o projeto:**
    ```bash
    cargo run --release
    ```

O programa iniciará e exibirá a simulação no seu terminal.

## Dependências

Esta implementação utiliza as seguintes crates:

- `rand`: Para a geração aleatória da população inicial de células.
- `term_size`: Para detectar o tamanho do terminal e ajustar o tamanho do universo.
