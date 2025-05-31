# Keyword in Context 

## Descrição do Trabalho
Este projeto implementa um sistema de Keyword in Context (KWIC) utilizando o método de programação `Actors` e a linguagem de programação `Rust`. O KWIC é um método no qual uma palavra-chave é apresentada junto ao seu contexto, ou seja, acompanhada pelas palavras que aparecem antes e depois dela em um determinado texto.

## Integrantes do Grupo
- **Gustavo Henrique Gomes Barbosa** - 232002771  
- **Henrique Morcelles Salum** - 232003008  
- **João Magno Lourenço Soares** - 232038077  
- **José Antônio Alcântara da Silva de Andrade** - 232013031  
- **Leandro Lucas da Silva Santos** - 222025852  

## Estilo e Linguagem de Programação
- **Estilo:** Actors
- **Linguagem:** Rust

## Links Importantes
- **Repositório no GitHub:** [https://github.com/dede6giu/rust-wic](https://github.com/dede6giu/rust-wic)
- **Vídeo Aula:** [Link para o vídeo](https://drive.google.com/file/d/1Om0i6f9AgnQdve5dGIqEQRxnUhC-e7vi/view?usp=drive_link)
- **Slides da Vídeo Aula:** [Link para os slides](https://docs.google.com/presentation/d/1qXVpl_pG4pOzO94t1AupAwZhCjEY-AgP/edit?usp=sharing&ouid=115946345136164169040&rtpof=true&sd=true)

## Build e Execução dos Testes

### Dependências
- [Rust](https://www.rust-lang.org/)   
- cargo: "1.87.0"
    - actix: "0.13.5"
    - tokio: "1.45.1"
- input padrão:
    - No Meio do Caminho (1928) — Carlos Drummond de Andrade
    - Canção do Exílio (1843) — Gonçalvez Dias
- stopwords padrão: (fergiemcdowall_stopwords_pt)[https://github.com/stopwords-iso/stopwords-pt/blob/master/raw/fergiemcdowall_stopwords_pt.txt] (adaptado)

### Como compilar e executar o projeto
```bash
# Clone o repositório
git clone https://github.com/dede6giu/rust-wic.git
cd rust-wic

# Execute o projeto
cargo run
```

### Como excutar os testes
```bash
cargo test
```

## Instruções

Navegue ao diretório "data/" e atualize os arquivos "input.txt" e "stopwords.txt" como desejado.

- `"input.txt"`: O texto original em que o algoritmo percorrerá. O algoritmo separa esse texto em "frases", as quais sempre começam em uma letra maiúscula e terminam na próxima letra maiúscula (ou o término do arquivo).
- `"stopwords.txt"`: Uma lista de stopwords que serão ignoradas no processamento. Cada palavra é separada por whitespace, e todas serão lidas como minúsculas.

Feito isso, use `cargo run` no diretório principal. O resultado irá aparecer no console. 


> **OBS**
><br>
>O resultado estará ordenado em ordem alfabética de keywords, e em seguida ordem alfabética de frases. O contexto recolhe duas palavras antes e depois da keyword, no formato key depois ... antes. O uso de reticências ... indica que palavras foram omitidas na mudança de contexto posterior à palavra para contexto anterior a ela.

> **AVISO**
><br>
>Ambos os arquivos não aceitam diacríticos nem caracteres especiais. Evite utilizar pontuação.
