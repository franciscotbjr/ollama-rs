# Blockers Log

Bloqueios conhecidos são impedimentos ou problemas identificados durante o desenvolvimento que ainda não foram resolvidos e que podem afetar sessões futuras.

## Tipos de Bloqueios

| Tipo | Exemplo |
|------|---------|
| Técnico | "Endpoint X retorna 500 quando payload > 1MB" |
| Dependência | "Aguardando release da lib/dependência Y para implementar feature Z" |
| Decisão pendente | "Precisa definir se usaremos WebSocket ou SSE para streaming" |
| Ambiente | "Testes de integração falham no CI por timeout" |

---

## Bloqueios Ativos

| Date | Type | Blocker | Impact | Reference |
|------|------|---------|--------|-----------|
| 2026-01-16 | Decisão pendente | Definir abordagem para streaming (tokio-stream vs async-stream) | Bloqueia endpoints generate/chat | DEV_NOTES.md |

## Bloqueios Resolvidos

| Date | Type | Blocker | Resolution | Resolved |
|------|------|---------|------------|----------|
