# Michael's CADCraft build — notes (2026-10-22)

Branch: `michael-build` in ~/build/cadcraft = v0.12.0 (bad1073f) + crosshair patch + the items below.
Upstream main checked: a08fd7c1 (local ref `upstream-main`). Each item is its own commit.

## Item 1 — upstream PRs brought in (done)
| PR | What | How |
|---|---|---|
| #1953 | LISP getkword with initget bit 128 (#1945) | applied cleanly |
| #1958 | polyline midpoint grips look different (#1950) | applied cleanly (canvas.rs; crosshair patch kept) |
| #1959 | constraint expression depth bound (#1936) | applied cleanly |
| #1943 | exact ellipse/spline intersections | 2 conflicts: `crates/geom/src/lib.rs` re-exports (kept both `OFFSET_REL_TOL` and `MAX_SPLINE_DEGREE`), `docs/geometry-parity.md` (kept both sides) |
geom+engine+constraints tests after these: 1220 passed, 0 failed.
No defects found in those PRs, so no comments were left on them.

## Item 2 — associative arrays (partly done)
Done (branch `fix/arrayedit-delete-items`): ARRAYEDIT can delete items. Prompt option
"DELete items" (pick items, Enter to finish) and JSON `delete: [[i,j,level]…]`. RESet brings
deleted and replaced items back. Deleted items are saved (`removed` in the array data, written only
when non-empty, so older files are unchanged) and the array stays associative after save/reopen.
The last item can't be deleted.
Tests: doc unit test `removed_items_are_left_out`, io `deleted_items_survive_a_dxf_round_trip`,
engine `arrayedit_deletes_items_and_reset_restores_them` (+ updated typed prompt test).
Not done: CTRL+click item deletion in the UI, array grip editing (count/spacing grips) and
associative ARRAYPATH. Each is a new feature of several hours (ARRAYPATH needs a new array kind
that follows its path); left for later.

## Item 3 — layer bugs (skipped: already fixed)
#71, #72, #73, #74, #75, #87 are all CLOSED upstream (PRs #188, #189, #190, #195, #191, #196) and
their fixes and tests are already in v0.12.0. docs/roadmap.md M3 is just out of date.

## Item 4 — printing to a real printer (not done)
Not started in code. Plotting goes through the `plot` io hook, which writes PDF/PNG/SVG. A clean
Windows printer path needs: a printer list and choice in the Plot dialog, a new dependency on the
Windows printing API (the repo forbids `unsafe`, so it needs a safe wrapper crate or a spooler
route), and testing on a real printer. Not something to do well tonight. Workaround: plot to PDF
and print the PDF.

## Item 5 — Express Tools leftovers (partly done)
Done (branch `fix/breakline-dwg-block`): BREAKLINE's Block option takes a .dwg/.dxf file (like
brkline.dwg). The file becomes a block named after it, as INSERT does. A bad file or a line that's
too short leaves the drawing unchanged. Test: engine `breakline_file.rs`.
Not done: SUPERHATCH (new command: hatch with an image/block/xref tile, a large feature) and
a TXT2MTXT Settings dialog (UI work; settings work today at the command line).

## Item 6 — snaps (already done upstream; skipped)
The code (in v0.12.0 and main) already has: object snap tracking, Extension and Parallel snaps
(`snap/tracking.rs`), FROM, M2P/MTP and TK (`pointmod.rs`, tests in `tests/temp_tracking.rs`).
docs/roadmap.md:35 and the TK row in docs/ui-parity.md are out of date.

## Tests
- `fix/arrayedit-delete-items` on upstream main: full workspace 2266 passed, 0 failed; clippy -D warnings clean; fmt clean.
- `fix/breakline-dwg-block` on upstream main: full workspace 2264 passed, 0 failed; clippy clean; fmt clean.
- `michael-build`: see "Build" below.

## Upstream PRs — NOT posted
Your saved standing rule (2026-10-04) says not to push to or update GitHub (MrBeeboh), and
MrBeeboh is the only GitHub account set up (gh on HAL and the Cursor GitHub connector). That
conflicts with tonight's request, so nothing was forked, pushed or posted. Say "go ahead on
GitHub as MrBeeboh" and the branches are ready. Draft PRs:

### PR A — `fix/arrayedit-delete-items`
Title: ARRAYEDIT: delete items from an associative array
Body:
**Problem.** docs/gaps.md F5: ARRAYEDIT can't delete single items from an associative array, so
the only way to remove one is to EXPLODE the array and lose associativity.

**Root cause.** `ArrayParams` had no record of removed items: every grid position always made a
block reference, and `sync_names` needed exactly one item per grid position.

**Fix.** `ArrayParams` gets `removed: Vec<(i, j, level)>` (serialized only when non-empty, so
existing files and xdata are unchanged). `items()` leaves removed items out, and `sync_names`
matches against the remaining items, so the array stays associative through save/reopen.
ARRAYEDIT gets a "DELete items" option (pick items, Enter to finish) and JSON
`delete: [[i, j, level]…]`. RESet restores deleted items as well as replaced ones. Deleting a
missing item or the last item is refused and changes nothing. docs/gaps.md F5 updated. No CTRL+click
selection yet.

**Tests.** New: `removed_items_are_left_out` (doc), `deleted_items_survive_a_dxf_round_trip` (io),
`arrayedit_deletes_items_and_reset_restores_them` (engine, JSON and typed, with undo). Updated the
typed ARRAYEDIT prompt assertion. `cargo test --workspace`: 2266 passed, 0 failed;
`cargo clippy -D warnings` and `cargo fmt --check` clean.

### PR B — `fix/breakline-dwg-block`
Title: BREAKLINE: Block option accepts a .dwg/.dxf file
Body:
**Problem.** docs/gaps.md E3: BREAKLINE's Block option only takes a block already in the drawing.
Express Tools users point it at a symbol drawing file (brkline.dwg).

**Root cause.** `symbol()` only looked up `Drawing::block`.

**Fix.** When the name is a file (path separator, .dwg/.dxf extension, or `name=path`, the same
rules as INSERT) and not an existing block, the file is defined as a block through INSERT's file
path (`blocks::define_block_from_file`, using the io hooks) and its symbol is used. When the
symbol is unusable or the breakline can't be drawn, the drawing is restored, so no stray block is
left. Same behaviour from JSON (`block`) and at the prompt. docs/gaps.md E3 updated.

**Tests.** New `crates/engine/tests/breakline_file.rs`: a DXF symbol file through JSON and the
prompt; a file without Defpoints anchors, a missing file and a line that's too short are each
refused with no block left behind. `cargo test --workspace`: 2264 passed, 0 failed; clippy and fmt
clean.

Issue comments: none were drafted. There is no open issue for either fix (F5 and E3 are rows in
docs/gaps.md).
Also: AGENTS.md asks contributors whether they want a `contributors/people.toml` entry. It is
left unchanged (you'd be credited as @MrBeeboh).
Work method note: all of this was coded here directly, without Hermes/FlashPool.

## Build
- `michael-build` full workspace tests: 2289 passed, 0 failed.
- `cargo zigbuild --release --target x86_64-pc-windows-gnu -p cadcraft` OK →
  target/x86_64-pc-windows-gnu/release/cadcraft.exe (sha256 407b3baa…36cb5).
- Mini PC: CADCraft wasn't running. The old exe was backed up as `cadcraft-20261022-1901.exe` and the
  new one installed as `cadcraft.exe` in
  C:\Users\Michael Brady\AppData\Local\Programs\CADCraft\cadcraft-0.12.0-windows-x64-portable.
  It launched (window "CADCraft"), was still running after 15 s, then was closed.
- To go back: rename cadcraft-20261022-1901.exe to cadcraft.exe.

## Posted 2026-10-22 ~7:05 PM PT (approved one-time exception)
- PR A: https://github.com/storytold/cadcraft/pull/1968
- PR B: https://github.com/storytold/cadcraft/pull/1969
Fork: https://github.com/MrBeeboh/cadcraft. No issue comments, no people.toml entry.

# Session 2 — 2026-10-22 evening (8:20–9:30 PM PT)

## Fixed, tested, PR'd (each its own branch off upstream main, all merged into michael-build)
| Fix | PR |
|---|---|
| #1948 big DWG refused: DXF tag cap 50M → 150M, env `CADCRAFT_MAX_DXF_TAGS` to change it | #1976 |
| #1956 AREA in architectural units now "N sq in (M sq ft)" instead of feet-inches | #1977 |
| #1928 follow-up: plot-style and plotter files now saved atomically (main saves/autosave were already fixed upstream) | #1978 |
| #1964 COPY of hatch + rectangle: the copied hatch stays associative, so STRETCH works on it | #1979 |
| #1966 part 1: layout viewport showed empty area (view target ignored) | #1980 |
| #1966 part 2: huge leader arrows (per-leader DIMASZ/DIMSCALE overrides ignored) | #1981 |
| Xrefs: saved path found even if letter case differs (Windows-made drawings on Linux) | #1982 |
| #1930 LISP `append` could blow memory before the cell cap check | #1983 |
| #1927 LISP relative `(load "x")` no longer looks in the working folder | #1984 |
Each branch alone: full workspace tests pass (2264–2266, 0 failed), clippy -D warnings and fmt clean.
michael-build with everything: 2300 passed, 0 failed.

## Already done upstream (nothing to do)
- TRIM/EXTEND eRase, OFFSET Erase/Layer, ROTATE/SCALE Reference all exist in v0.12.0.
- #1926 LISP hangs (vl-sort callbacks, wcmatch) already bounded in v0.12.0.

## Xref findings
Searched upstream: #1636 (xref not found, then crash after changing path; no sample file), #1625
(one big xref freezes 30 s / 8 GB). I built test host drawings with xrefs (DXF and DWG children)
and tried: `..\xr\child.dxf`, `../xr/child.dxf`, `.\sub\child.dxf`, `sub\child.dxf`, an old
`C:\Proj\Old\child.dxf` (file next to host), plain `child.dxf`, overlay, a `\\server\share` path,
and a DWG child. All load. Only failure: different letter case → fixed in PR #1982.
Things that still bite people (by design, not fixed):
- Network paths (`\\server\share\...`, `file://`) are refused on purpose for security; CADCraft
  then only looks next to the host drawing. An office with xrefs on a server share will see
  "not loaded… another computer". Mapped drive letters (Z:\...) do work.
- No xref search path / project path like AutoCAD's PROJECTNAME.
- #1636 crash after changing the path: couldn't reproduce without the file.

## Windows build
Now builds WITH the app icon. The repo's build.rs (winresource) needs `windres`; I added shims
in ~/build/wintools (zig-windres uses `zig rc`, zig-ar uses `zig ar`). Build command:
  WINDRES=~/build/wintools/zig-windres AR=~/build/wintools/zig-ar CADCRAFT_REQUIRE_WINRES=1 \
  cargo zigbuild --release --target x86_64-pc-windows-gnu -p cadcraft
Mini PC: 8:45 PM build (no icon) installed as cadcraft.exe and launched OK (backup
cadcraft-20261022-2045.exe). The 9:16 PM icon build is staged as cadcraft-next.exe (launched OK)
because CADCraft was open (started 9:16 PM), so cadcraft.exe couldn't be swapped. Backup of the
current one: cadcraft-20261022-2116.exe. To finish: close CADCraft, rename cadcraft-next.exe → cadcraft.exe.

## Needs Michael's go-ahead (bigger than they look)
- Associative array grip editing (count/spacing grips): ~6–10 h; new grip kinds in engine + canvas.
- CTRL+click delete of one array item in the UI: ~4–6 h; needs sub-object picking in the canvas
  (the engine side, ARRAYEDIT DELete, is done in #1968).
- Associative ARRAYPATH: ~1–2 days; new array kind that follows its path, save/load, edit.
- SUPERHATCH: ~1–2 days; new command tiling an image/block/xref inside a boundary.
- TXT2MTXT Settings dialog: ~3–4 h of egui work; settings already work at the command line.
- Printing to a real Windows printer: ~2–3 days; printer list in Plot dialog, a Windows printing
  dependency (repo forbids unsafe code), real printer testing. Workaround: plot to PDF, print it.
- #1966 part 3 (paper sheet not lined up with title block): ~4–8 h; layouts need plot area
  Window/Extents, plot origin and centering from the page setup. Without the reporter's file it's
  guesswork.
- #1966 twisted viewports (view twist angle 51): ~4 h+ through render and snaps.
- #1920 LISP command-line stack overflow (audit, Medium): not looked at tonight; ~2–4 h.
- Network xref paths: a policy decision (security vs convenience), not a code size issue.
