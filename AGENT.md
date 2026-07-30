
## Token efficiency
- Skip recaps unless the result is ambiguous or you need more input.

### Archivos
```bash
rtk ls .                        # Arbol de directorios optimizado
rtk read file.rs                # Lectura inteligente
rtk find "*.rs" .               # Resultados compactos
rtk grep "pattern" .            # Busqueda agrupada por archivo
```

### Git
```bash
rtk git status                  # Estado compacto
rtk git log -n 10               # Commits en una linea
rtk git diff                    # Diff condensado
rtk git push                    # -> "ok main"
```

### Tests
```bash
rtk jest                        # Jest compacto
rtk vitest                      # Vitest compacto
rtk pytest                      # Tests Python (-90%)
rtk go test                     # Tests Go (-90%)
rtk cargo test                  # Tests Rust (-90%)
rtk test <cmd>                  # Solo fallos (-90%)
```

### Build & Lint
```bash
rtk lint                        # ESLint agrupado por regla
rtk tsc                         # Errores TypeScript agrupados
rtk cargo build                 # Build Cargo (-80%)
rtk ruff check                  # Lint Python (-80%)
```

### Analiticas
```bash
rtk gain                        # Estadisticas de ahorro
rtk gain --graph                # Grafico ASCII (30 dias)
rtk discover                    # Descubrir ahorros perdidos
```
