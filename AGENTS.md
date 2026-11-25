# lib-kv-map

`kv_map` — a pure functional key-value map. The store is a Handl record
**value** you hold and pass back — `kv_map.new() -> s`,
`kv_map.set(s,k,v) -> s'`, `kv_map.get(s,k)` — state is explicit data,
not ambient module state like `kv`/`kv_hashmap` (which stay native).

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the `kv_map.*` cap/op declarations |
| `pkg.hnl` | manifest — dep `obj` (`branch:main`), `modules: ["kv_map"]` |
| `scripts/hnl-gate.sh` | `.hnl` gate |
| `workflows/gate.yaml` | CI — shared `handl-gates` campaign |

## Contract

- Pure `.hnl` over the `obj.*` substrate + record field access — every
  op returns a new store value; nothing is mutated in place.
- No Rust crate; choose this over `kv` when you need a passable,
  serializable store value.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
