# CRUD API - Rust Axum

Uma API REST simples construída com Rust e Axum para gerenciar usuários em memória.

## Requisitos

- Rust 1.91.1+
- Docker (opcional)
- Cargo

## Instalação

### Local

```bash
# Clonar o repositório
git clone https://github.com/DanielDeAzevedoCordeiro1/API-Rust
cd crud

# Compilar
cargo build

# Rodar
cargo run
```

A API estará disponível em `http://localhost:9000`

### Docker

```bash
# Construir imagem
docker build -t crud_image .

# Rodar container
docker run -p 9000:9000 --name crud_container crud_image
```

## Endpoints

### Criar Usuário (POST)
```bash
http POST localhost:9000/user name="João Silva" age:=25
```

**Resposta:**
```json
{
  "payload": {
    "id": "uuid-gerado",
    "name": "João Silva",
    "age": 25
  },
  "status": 201,
  "error": null
}
```

### Buscar Usuário (GET)
```bash
http GET localhost:9000/user/0
```

### Listar Todos (GET)
```bash
http GET localhost:9000/all
```

## Estrutura de Dados

### UserRequest
```rust
{
  "name": String,
  "age": u8
}
```

### User
```rust
{
  "id": String,
  "name": String,
  "age": u8
}
```

### UserResponse
```rust
{
  "payload": Option<User>,
  "status": u16,
  "error": Option<String>
}
```
## Licença

MIT
