# Rocket Simples

Este é um projeto onde criei um site todo com Rust usando o Rocket e templates HTML.

## Dependências

- `rocket`: Versão 0.5.1 com as features `secrets` e `json`
- `rocket_dyn_templates`: Versão 0.2.0 com a feature `tera`
- `figment`: Versão 0.10.18
- `serde`: Versão 1.0.204 com a feature `derive`
- `serde_json`: Versão 1.0.120

## Estrutura do Projeto

O projeto possui a seguinte estrutura:

rocket-simples/
├── Cargo.toml
├── src/
│ ├── main.rs
├── templates/
│ ├── layouts/
│ │ ├── application.html
│ ├── clientes/
│ │ ├── index.html
│ │ ├── new.html
└── README.md


## Como Iniciar o Aplicativo

### Instalação das Dependências

Antes de iniciar o aplicativo, certifique-se de que você tenha o Rust instalado. Se não tiver, você pode instalar seguindo as instruções em [rust-lang.org](https://www.rust-lang.org/).

### Clonar o Repositório

Clone o repositório para sua máquina local:

```sh
git clone https://github.com/seu-usuario/rocket-simples.git
cd rocket-simples
```
### Instala cargo-watch

```sh
cargo install cargo-watch
```

### Inicia o aplicativo

```sh
cargo watch -x run
```

### Funcionalidades

- Cadastrar um novo cliente
- Listar todos os clientes
- Deletar um clientes

### Exemplos de uso 

#### Listagem de Clientes


Acesse a URL http://localhost:8000/clientes para ver a lista de clientes.
Adicionar Novo Cliente

Acesse a URL http://localhost:8000/clientes/new para adicionar um novo cliente.

### Código Fonte

[https://github.com/talespalma/rocket-simples](https://github.com/talespalma/rocket-simples)

### Desenvolvido por: Tales Palma

### Contato

[https://github.com/talespalma](https://github.com/talespalma)
[https://www.linkedin.com/in/tales-palma/](https://www.linkedin.com/in/tales-palma/)
