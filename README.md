# rust-wic

Implementação de "Words in Context" do livro "Exercises in programming style" por Crista Lopes.

## Instruções

Navegue ao diretório "data/" e atualize os arquivos "input.txt" e "stopwords.txt" como desejado.

- "input.txt": O texto original em que o algoritmo percorrerá. O algoritmo separa esse texto em "frases", as quais sempre começam em uma letra maiúscula e terminam na próxima letra maiúscula (ou o término do arquivo).
- "stopwords.txt": Uma lista de stopwords que serão ignoradas no processamento. Cada palavra é separada por whitespace, e todas serão lidas como minúsculas.

Feito isso, use `cargo run` no diretório principal. O resultado irá aparecer no console. 

O resultado estará ordenado em ordem alfabética de keywords, e em seguida ordem alfabética de frases. O uso de reticências `...` indica que palavras foram omitidas na mudança de contexto posterior à palavra para contexto anterior a ela. 

> [!warning] Observação
> Ambos os arquivos não aceitam diacríticos nem caracteres especiais. Evite utilizar pontuação.

# Dependências:

- cargo: "1.87.0"
    - actix: "0.13.5"
    - tokio: "1.45.1"
- input:
    - No Meio do Caminho (1928) — Carlos Drummond de Andrade
    - Canção do Exílio (1843) — Gonçalvez Dias
- stopwords: (fergiemcdowall_stopwords_pt)[https://github.com/stopwords-iso/stopwords-pt/blob/master/raw/fergiemcdowall_stopwords_pt.txt] (adaptado)