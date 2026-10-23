# cadcraft-cli: agent guide

Everything here is additive. Without the new flags, output and exit codes are unchanged.

## Discover
- `cadcraft-cli describe`: the CLI surface as JSON (subcommands, options, formats, exit codes).
- `cadcraft-cli commands [FILTER]`: engine command ids and their JSON params (use with `--cmd`).

## Check before you work
- `cadcraft-cli validate FILE.dxf`: JSON `{clean, entities, extents, warnings, dxf_save_loss}`.
- `cadcraft-cli info FILE.dxf`: full summary as JSON.

## Run a batch
```
cadcraft-cli --json run plan.dxf --cmd-file steps.txt --save out.dxf --export out.pdf --report report.json
```
- `--cmd-file`: one `id {json}` per line; `#` comments and blank lines are skipped.
- `--dry-run`: run every step, write no files. The report lists what would be written.
- `--report OUT.json` (or `-` for stderr): per-step `{step, kind, input, ok, result|error}`, the log,
  the outputs and the entity count. It is written on failure too, so the failing step is identified.
- Execution stops at the first failing step. Nothing is saved after a failure.

## Errors
With `--json` (anywhere on the line) or `CADCRAFT_JSON=1`, an error is one JSON line on stderr:
`{"ok":false,"error":"...","kind":"usage|io|command","exit_code":N}`.
The exit codes are 2 for usage, 3 for file/io and 4 for a failed command. In default mode every error still exits 1.

## Drafting notes
- Text background mask: there is no mask flag on text. Put a `wipeout` under the text instead
  (`cadcraft-cli commands wipeout`).
- Lineweights and linetype scale: the `lweight` and `ltscale` commands exist (`commands lweight`).
