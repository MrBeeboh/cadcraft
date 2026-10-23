//! Agent-facing helpers for `cadcraft-cli`: the `describe` catalog, `--cmd-file` parsing and the
//! machine-readable error classification used by `--json` / `CADCRAFT_JSON=1`. All additive: with
//! neither set, errors stay plain text with exit status 1.

use serde_json::{Value, json};

/// `CADCRAFT_JSON=1` (or `true`) turns on JSON errors without changing the command line.
pub fn json_env() -> bool {
    std::env::var("CADCRAFT_JSON").is_ok_and(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes"))
}

/// Error kind and exit code: 2 usage, 3 file/io, 4 a command or conversion failed.
pub fn classify(e: &str) -> (&'static str, u8) {
    let l = e.to_ascii_lowercase();
    if l.starts_with("usage:")
        || l.contains("unknown option")
        || l.contains(" needs ")
        || l.starts_with("--window")
        || l.starts_with("--view")
        || l.contains("use only one of")
    {
        ("usage", 2)
    } else if l.contains("os error") || l.contains("no such file") || l.contains("permission denied") || l.contains("cannot connect") {
        ("io", 3)
    } else {
        ("command", 4)
    }
}

/// The commands of a `--cmd-file`: one `id {json}` per line; blank lines and `#` comments skipped.
pub fn cmd_file_lines(text: &str) -> Vec<String> {
    text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')).map(String::from).collect()
}

/// The CLI surface as JSON, for an agent to read once instead of parsing `--help`.
pub fn describe() -> Value {
    let framing = json!([
        {"name": "--window", "value": "x0,y0,x1,y1", "help": "show this model-space rectangle"},
        {"name": "--view", "value": "extents|current", "help": "fit extents or keep the current view"}
    ]);
    json!({
        "tool": "cadcraft-cli",
        "version": env!("CARGO_PKG_VERSION"),
        "read_formats": ["dxf", "dwg"],
        "write_formats": ["dxf", "dwg", "svg", "png", "pdf"],
        "image_formats": ["svg", "png", "pdf"],
        "global_options": [
            {"name": "--json", "help": "errors as one JSON line on stderr with exit codes 2/3/4"}
        ],
        "env": [{"name": "CADCRAFT_JSON", "help": "1 = same as --json"}],
        "exit_codes": {"0": "ok", "1": "error (default plain-text mode)", "2": "usage (json mode)", "3": "file/io (json mode)", "4": "command failed (json mode)"},
        "subcommands": [
            {"name": "info", "args": ["FILE"], "output": "json", "help": "drawing summary"},
            {"name": "validate", "args": ["FILE"], "output": "json", "help": "open and report skipped records, undrawable entities, extents"},
            {"name": "convert", "args": ["IN", "OUT"], "options": framing, "help": "convert/export by extension"},
            {"name": "run", "args": ["[FILE]"], "output": "one JSON value per --cmd result on stdout; log on stderr", "options": [
                {"name": "--sample"}, {"name": "--metric"},
                {"name": "--script", "value": "TEXT", "help": "command-line script (\\n = newline)"},
                {"name": "--script-file", "value": "F.scr"},
                {"name": "--cmd", "value": "id {json}", "repeatable": true, "help": "see `commands` for ids and params"},
                {"name": "--cmd-file", "value": "F.txt", "help": "one `id {json}` per line"},
                {"name": "--save", "value": "OUT.dxf|dwg"},
                {"name": "--export", "value": "OUT.png|svg|pdf"},
                {"name": "--dry-run", "help": "run everything, write no files"},
                {"name": "--report", "value": "OUT.json|-", "help": "per-step results, log, outputs"},
                {"name": "--window", "value": "x0,y0,x1,y1"}, {"name": "--view", "value": "extents|current"}
            ]},
            {"name": "commands", "args": ["[FILTER]"], "output": "json", "help": "engine command catalog with params"},
            {"name": "describe", "output": "json", "help": "this document"},
            {"name": "mcp", "options": [{"name": "--connect", "value": "HOST:PORT"}, {"name": "--lazy"}, {"name": "--launch"}], "help": "MCP server on stdio"},
            {"name": "sample", "args": ["bracket|floorplan", "OUT"]},
            {"name": "perf", "args": ["[N|FILE]"]}
        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_errors() {
        assert_eq!(classify("usage:\n  cadcraft-cli ...").1, 2);
        assert_eq!(classify("unknown option --x").1, 2);
        assert_eq!(classify("a.dxf: No such file or directory (os error 2)").1, 3);
        assert_eq!(classify("unknown command `no.such`").1, 4);
    }

    #[test]
    fn cmd_file_skips_comments_and_blanks() {
        assert_eq!(cmd_file_lines("# c\n\n line {}\n  zoom {\"mode\":\"extents\"}\n"), vec!["line {}", "zoom {\"mode\":\"extents\"}"]);
    }

    #[test]
    fn describe_lists_every_subcommand() {
        let d = describe();
        let names: Vec<&str> = d["subcommands"].as_array().unwrap().iter().filter_map(|s| s["name"].as_str()).collect();
        for n in ["info", "validate", "convert", "run", "commands", "describe", "mcp", "sample", "perf"] {
            assert!(names.contains(&n), "{n}");
        }
    }
}
