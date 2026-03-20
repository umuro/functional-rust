📖 **[View on hightechmind.io →](https://hightechmind.io/rust/766-config-file-parsing)**

---

# 766-config-file-parsing — Config File Parsing

## Problem Statement

Configuration files — INI, TOML, YAML — are the primary mechanism for parametrizing deployed software without recompilation. INI format predates them all and remains ubiquitous in tools like `git config`, Windows registry exports, and `openssl.cnf`. Parsing INI manually teaches section-based hierarchical configuration, comment handling, and graceful error reporting for malformed config files.

## Learning Outcomes

- Parse INI-style `[section]` headers and `key = value` assignments
- Store configuration in a two-level `HashMap<String, HashMap<String, String>>`
- Handle inline comments (`#` and `;`), blank lines, and trailing whitespace
- Return typed `ParseError` variants for invalid lines and duplicate sections
- Implement typed accessors: `get_section(section, key) -> Option<&str>`

## Rust Application

`Config` holds a `global: HashMap<String, String>` for key-value pairs before the first section, and `sections: HashMap<String, HashMap<String, String>>`. `parse_config` iterates lines: blank/comment lines are skipped, `[section]` lines update the current section, `key = value` lines insert into the current map. `ParseError::InvalidLine` reports the line number and content; `ParseError::DuplicateSection` reports the section name.

## OCaml Approach

OCaml's `Inifiles` library parses INI files with similar section/key structure. The `Config` module from `caml-gettext` handles POSIX-style configuration. For TOML, `toml-ocaml` and `otoml` are available. The parsing pattern uses `String.split_on_char` for line splitting and `String.trim` for whitespace normalization — identical operations to Rust's `split('\n')` and `trim()`.

## Key Differences

1. **HashMap access**: Rust's `HashMap::get` returns `Option<&V>`; OCaml's `Hashtbl.find_opt` returns `option 'v` — equivalent semantics.
2. **Line iteration**: Rust's `str::lines()` is lazy and handles both `\n` and `\r\n`; OCaml's `String.split_on_char '\n'` requires manual `\r` stripping.
3. **Duplicate detection**: Rust checks `HashMap::contains_key` before insert; OCaml uses `Hashtbl.mem` — same pattern.
4. **Typed config**: Production code uses `serde` with a TOML/INI deserializer to map config directly to a typed struct; OCaml's `ppx_sexp_conv` does the same for S-expression configs.

## Exercises

1. Add support for `include = /path/to/other.ini` directives that merge another config file's sections and keys into the current config.
2. Implement `Config::to_string()` that serializes the config back to INI format, preserving sections and key-value pairs.
3. Write typed getters: `get_bool(section, key) -> Option<bool>`, `get_u64(section, key) -> Option<u64>`, and `get_list(section, key) -> Vec<String>` (comma-separated values).
