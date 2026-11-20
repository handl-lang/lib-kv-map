# lib-kv-map

`kv_map` — a pure functional key-value map. The store is a Handl record
**value** you hold and pass back — `kv_map.new() -> s`,
`kv_map.set(s,k,v) -> s'`, `kv_map.get(s,k)` — state is explicit data, not
ambient module state like `kv`/`kv_hashmap` (which stay native). Pure
`.hnl` over the `obj.*` substrate + record field access.

## Layout

```text
src/index.hnl           the cap/op declarations (kv_map.*)
scripts/hnl-gate.sh     the shared hnl typecheck/lint/test gate
ci.sh                   bupha entry
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
