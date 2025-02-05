# Informações

Este projeto contém o algoritmo de leitura e armazenamento de uma estrutura de dados face-vertice a partir de um arquivo `.off`. O algoritmo foi desenvolvido em Rust e não utiliza nenhuma biblioteca externa para a leitura do arquivo.

# Como executar o projeto

Para executar o projeto é necessário ter a versão mais recente do `cargo` instalada. Para isso siga as instruções do site oficial do [Rust clicando aqui](https://www.rust-lang.org/pt-BR/tools/install).

Após ter instalado, basta baixar o projeto e executar os seguintes comandos em um terminal Windows, Linux ou MacOS:

```shell
$ git clone https://github.com/caiogmrocha/projeto-face-vertice.git
$ cd projeto-face-vertice
$ cargo run -- --read assets/triangles.off
$ cargo run -- --write assets/triangles.off
$ cargo run -- --read assets/hand-hybrid.off
$ cargo run -- --write assets/hand-hybrid.off
```

# Como funcionam os argumentos

O projeto possui dois argumentos que podem ser passados para o programa. São eles:

- `--read`: Lê um arquivo `.off` e imprime a estrutura de dados face-vertice na tela.
- `--write`: Lê um arquivo `.off` e escreve a estrutura de dados face-vertice em `assets/output.off`.