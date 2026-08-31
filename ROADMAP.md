# Roadmap de Validação — Acompanhamento

## Change: `add-validation-modules`
**Localização**: `openspec/changes/add-validation-modules/`
**Status**: ✅ Implementação completa

---

## Fase 2: Módulos de Validação

### ✅ Planejamento
- [x] Proposal (`proposal.md`)
- [x] Design (`design.md`)
- [x] Tasks (`tasks.md`)
- [x] Skip specs (comportamento não muda, só código novo)

### ✅ 1. Setup
- [x] 1.1 Adicionar `serde_yaml` ao Cargo.toml
- [x] 1.2 Criar `src/validation/mod.rs` — parser YAML compartilhado
- [x] 1.3 Adicionar `pub mod validation` ao `src/lib.rs`

### ✅ 2. Agents Module (`src/validation/agents.rs`)
- [x] 2.1 Validador de schema de agente (name, description, mode, permissions)
- [x] 2.2 Validador de category JSON (0-category.json)
- [x] 2.3 Detecção de ciclos no delegation graph (DFS)
- [x] 2.4 Testes unitários
- [x] 2.5 Walk test (`tests/agent_walk.rs`)

### ✅ 3. Skills Module (`src/validation/skills.rs`)
- [x] 3.1 Validador de SKILL.md (name, triggers)
- [x] 3.2 Validador de router.sh (shebang + executável)
- [x] 3.3 Validação de arquivos referenciados
- [x] 3.4 Testes unitários
- [x] 3.5 Walk test (`tests/skill_walk.rs`)

### ✅ 4. Commands Module (`src/validation/commands.rs`)
- [x] 4.1 Validador de frontmatter de comando
- [x] 4.2 Validação de referências de dependência
- [x] 4.3 Testes unitários
- [x] 4.4 Walk test (`tests/command_walk.rs`)

### ✅ 5. Evals Module (`src/evals/mod.rs`)
- [x] 5.1 Tipos do schema de eval cases
- [x] 5.2 Parser + validador YAML
- [x] 5.3 Tipos de results JSON
- [x] 5.4 Gerador de dashboard HTML
- [x] 5.5 Testes unitários

### ✅ 6. CLI Integration
- [x] 6.1 Estender `validate` com flags `--agents`, `--skills`, `--commands`, `--evals`
- [x] 6.2 Roteamento para validadores
- [x] 6.3 Testar `myagentcontrol validate` com cada flag

### ✅ 7. Verificação
- [x] 7.1 `cargo test` — todos passam
- [x] 7.2 `cargo clippy -- -D warnings`
- [x] 7.3 `cargo fmt --check`
- [x] 7.4 Commit e push

---

## Resumo de Progresso

| Módulo | Spec | Código | Testes | Walk Test | Status |
|--------|------|--------|--------|-----------|--------|
| Foundation | ✅ | ✅ | ✅ | ✅ | **Pronto** |
| Context | ✅ | ✅ | ✅ | ✅ | **Pronto** |
| Registry | ✅ | ✅ | ✅ | — | **Pronto** |
| Agents | ✅ | ✅ | ✅ | ✅ | **Pronto** |
| Skills | ✅ | ✅ | ✅ | ✅ | **Pronto** |
| Commands | ✅ | ✅ | ✅ | ✅ | **Pronto** |
| Evals | ✅ | ✅ | ✅ | — | **Pronto** |
| Walk Tests (expandido) | ✅ | ✅ | — | ✅ | **Pronto** |
| Polish/Release | 🔲 | 🔲 | 🔲 | — | **Fase 4** |
