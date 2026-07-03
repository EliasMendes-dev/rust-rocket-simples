# rust-rocket-simples

Projeto simples em Rust usando Rocket e Tera para um CRUD de clientes com banco MySQL.

## Visão geral

O projeto implementa:
- rota `GET /` para página inicial
- rota `GET /clientes` para listar clientes
- rota `GET /clientes/novo` para abrir formulário de criação
- rota `POST /clientes/criar` para inserir um cliente no banco
- rota `GET /clientes/<id>/editar` para carregar os dados de edição
- rota `POST /clientes/<id>/editar` para atualizar o cliente
- rota `POST /clientes/<id>/excluir` para excluir o cliente
- templates Tera para renderização de páginas
- gerenciamento de layout com `application.html.tera`
- assets estáticos servidos em `/static`
- conexão real com MySQL via `sqlx`

## Stack

- Rust 2024
- Rocket 0.5.1
- Rocket Dyn Templates + Tera
- SQLx com MySQL
- Chrono para data/ano atual
- Bootstrap para frontend

## Estrutura do projeto

- `Cargo.toml` -> dependências
- `Rocket.toml` -> configuração do Rocket
- `database/mod.rs` -> módulo de conexão MySQL
- `database/init.sql` -> script de criação de banco e tabela
- `src/main.rs` -> montagem da aplicação Rocket
- `src/controllers/` -> rotas e lógica HTTP
- `src/servicos/` -> regras de serviço
- `src/repositorios/` -> acesso ao banco de dados
- `src/dtos/` -> DTOs de formulário
- `src/models/` -> modelos de domínio
- `src/utils/` -> utilitários, como ano atual
- `templates/` -> layouts e views Tera
- `static/` -> CSS, JS e fontes estáticas

## Como rodar

1. Instale Rust e Cargo.
2. Instale MySQL e crie o banco federado.
3. Execute o script SQL:

```bash
mysql -u root -p < database/init.sql
```

4. Ajuste a string de conexão em `src/database/mod.rs`, se necessário:

```rust
MySqlPool::connect("mysql://root:root@localhost/clientes_rust_db")
```

5. Rode o projeto:

```bash
cargo run
```

6. Acesse no navegador:

```
http://127.0.0.1:8000
```

## Fluxo de execução

### Carregamento inicial
1. Rocket inicia em `src/main.rs`.
2. Conecta ao banco com `database::conectar()`.
3. Monta rotas e serve arquivos estáticos em `/static`.

### Listagem de clientes
1. `GET /clientes` chama `clientes_controller::index`.
2. Ele chama `get_clientes(pool)` em `src/servicos/cliente_servico.rs`.
3. O serviço chama `cliente_repository::listar(pool)`.
4. A query SQL retorna todos os clientes.
5. O template `templates/clientes/index.html.tera` exibe a tabela.

### Criar cliente
1. `GET /clientes/novo` exibe o formulário em `templates/clientes/novo.html.tera`.
2. O usuário preenche `nome` e `cpf`.
3. `POST /clientes/criar` chama `clientes_controller::criar`.
4. O controller chama `criar_cliente(pool, &cliente_dto)`.
5. O serviço chama `cliente_repository::criar_cliente(pool, cliente)`.
6. O repositório executa `INSERT INTO clientes (nome, cpf)`.
7. Após sucesso, redireciona de volta para `/clientes`.

### Editar cliente
1. O botão de editar aponta para `/clientes/{{ cliente.id }}/editar`.
2. `GET /clientes/<id>/editar` carrega o cliente com `get_cliente_por_id`.
3. O template `templates/clientes/editar.html.tera` préenche o formulário.
4. `POST /clientes/<id>/editar` chama `clientes_controller::atualizar`.
5. O controller chama `atualizar_cliente(pool, id, &cliente)`.
6. O repositório executa `UPDATE clientes SET nome = ?, cpf = ? WHERE id = ?`.
7. Após sucesso, redireciona para `/clientes`.

### Excluir cliente
1. O botão excluir usa um `<form method="post" action="/clientes/{{ cliente.id }}/excluir">`.
2. A página usa `onsubmit="return confirm('Deseja realmente excluir este cliente?');"`.
3. Ao confirmar, `POST /clientes/<id>/excluir` chama `clientes_controller::excluir`.
4. O controller chama `excluir_cliente(pool, id)`.
5. O repositório executa `DELETE FROM clientes WHERE id = ?`.
6. O redirecionamento para `/clientes` atualiza a lista.

## Detalhes importantes

- `src/dtos/cliente_dto.rs` define `ClienteDto` com `FromForm`, usado para capturar dados de formulário.
- `src/models/cliente.rs` define `Cliente` com `Serialize` e `FromRow` para renderização e mapeamento SQL.
- `templates/partials/_header.html.tera` define o menu e usa `current` para marcar a página ativa.
- `templates/partials/_footer.html.tera` exibe o ano atual.
- `src/utils/date_utils.rs` usa `chrono::Local` para retornar o ano presente.
- `templates/layouts/application.html.tera` inclui CSS/JS estático e o layout base.

## Observações

- O `static/js/site.js` está vazio, mas já estruturado para futuras interações JavaScript.
- As dependências `rocket_dyn_templates` e `sqlx` estão configuradas em `Cargo.toml`.
- O banco é MySQL, mas o mesmo padrão de repositório pode ser adaptado para PostgreSQL ou SQLite.

## Como testar

- Verifique se a página inicial carrega.
- Crie um cliente e confirme o retorno para a lista.
- Edite um cliente e confirme os dados alterados.
- Exclua um cliente e confirme que ele desaparece da tabela.

---

Veja também `README-ADICOES.md` para as adições específicas feitas por mim e como aplicar essas melhorias a outro projeto.
