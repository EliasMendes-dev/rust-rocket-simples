# Adições feitas no projeto

Este arquivo documenta as melhorias que eu adicionei ao projeto original do curso.

## O que foi adicionado

### 1. Página ativa no menu

- Na barra de navegação (`templates/partials/_header.html.tera`), usei a variável `current`.
- O valor `home` ou `clientes` define se o link deve receber a classe `active` do Bootstrap.
- Isso muda a cor do item ativo e melhora a usabilidade.

### 2. Ano automático no rodapé

- Adicionei `src/utils/date_utils.rs` com `chrono`.
- A função `get_current_year()` retorna o ano atual e insere em `templates/partials/_footer.html.tera`.
- Isso evita atualizar o ano manualmente todo ano.

### 3. Banco de dados real

- Substituí a simulação original por banco MySQL real.
- `src/database/mod.rs` conecta diretamente a `clientes_rust_db`.
- `database/init.sql` cria o banco, a tabela e já injeta alguns registros de exemplo.
- Isso torna o projeto funcional e pronto para uso de verdade.

### 4. Assets estáticos

- O projeto já estava usando Bootstrap CSS/JS, mas mantive o `static/` com `css/`, `js/` e `fonts/`.
- Adicionei `static/js/site.js` vazio para futuras interações.
- Isso deixa o projeto organizado e pronto para comportamento client-side.

## Fluxo completo do CRUD com contexto

### Criação de cliente

1. Usuário acessa `/clientes/novo`.
2. O template `templates/clientes/novo.html.tera` exibe o formulário.
3. O formulário envia `POST /clientes/criar`.
4. `src/controllers/clientes_controller.rs` recebe os dados no `Form<ClienteDto>`.
5. O controller chama `criar_cliente` em `src/servicos/cliente_servico.rs`.
6. O serviço chama `cliente_repository::criar_cliente`.
7. O repositório insere o registro no banco com SQL.
8. O controller redireciona para `/clientes`.

### Leitura de clientes

1. Usuário abre `/clientes`.
2. O controller `index` chama `get_clientes` no serviço.
3. O serviço chama `cliente_repository::listar`.
4. O repository executa `SELECT id, nome, cpf FROM clientes`.
5. O template exibe todos os clientes em tabela.
6. Mostra também o total de clientes com `{{ clientes | length }}`.

### Edição de cliente

1. A lista mostra botão editar para cada cliente.
2. Clique em editar vai para `/clientes/<id>/editar`.
3. O controller busca o cliente por ID e renderiza `templates/clientes/editar.html.tera`.
4. O usuário edita e envia `POST /clientes/<id>/editar`.
5. O controller atualiza via serviço e repository.
6. A query `UPDATE clientes SET nome = ?, cpf = ? WHERE id = ?` altera o registro.
7. A página redireciona de volta para a lista.

### Exclusão de cliente

1. Cada linha de cliente tem botão delete.
2. O formulário de exclusão usa `onsubmit="return confirm(...)"`.
3. Se o usuário confirma, o POST vai para `/clientes/<id>/excluir`.
4. O controller chama `excluir_cliente` no serviço.
5. O repository executa `DELETE FROM clientes WHERE id = ?`.
6. Após o redirect, a tabela atualiza e o cliente não aparece mais.

## Como aplicar essas melhorias em outro projeto

### Passo 1: adicionar o layout ativo

1. No layout do menu, crie uma variável `current` no contexto do template.
2. Use algo como:

```tera
<a class="nav-link {% if current == 'clientes' %}active{% endif %}" href="/clientes">
```

3. Em cada controller, passe `current: "home"` ou `current: "clientes"`.

### Passo 2: ano automático

1. Adicione `chrono` no `Cargo.toml`.
2. Crie utilitário:

```rust
use chrono::{Datelike, Local};

pub fn get_current_year() -> i32 {
    Local::now().year()
}
```

3. No template do footer, use `{{ year }}`.
4. Passe `year: get_current_year()` no contexto.

### Passo 3: banco real com SQLx

1. Configure `sqlx` no `Cargo.toml` com `mysql`, `runtime-tokio`, `macros`.
2. Crie conexão com `MySqlPool::connect(...)`.
3. Use `query_as` para `SELECT` e `query` para `INSERT/UPDATE/DELETE`.
4. Trate dados em `models` com `FromRow` e `Serialize`.

### Passo 4: separar camadas

- `controllers`: rotas e HTTP
- `servicos`: regras de negócio
- `repositorios`: SQL direto
- `dtos`: formulários
- `models`: entidades de domínio

### Passo 5: confirmação de exclusão

1. Use formulário com ação POST e botão.
2. Adicione `onsubmit="return confirm('Deseja realmente excluir este cliente?');"`
3. O backend recebe a exclusão em POST e redireciona para atualizar a página.

### Passo 6: assets organizados

- Coloque CSS e JS em `static/`
- Monte com `FileServer::from(relative!("static"))`
- Inclua no layout:

```html
<script src="/static/js/bootstrap.bundle.min.js"></script>
<script src="/static/js/site.js"></script>
```

## Resultado final

Essas melhorias deixam o projeto:
- mais completo e funcional
- mais parecido com aplicação real
- modularizado e mais fácil de manter
- com feedback visual no menu e rodapé atualizado
- usando banco de dados real em vez de simulação


