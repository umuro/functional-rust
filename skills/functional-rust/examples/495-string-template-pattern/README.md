# 495: String Template Pattern

**Difficulty:** 2  **Level:** Intermediate

Replace `{{variable}}` placeholders with values from a map — a lightweight template engine in pure Rust.

## The Problem This Solves

Code generation, email bodies, SQL fragment builders, configuration file generation — all share a pattern: a fixed skeleton with slots that get filled in at runtime. You could use `format!` with positional arguments, but that requires knowing all variables at compile time and gets unwieldy with many fields.

A template engine decouples the *structure* (written as a string with named placeholders) from the *data* (a map of variable names to values). The template is a first-class value you can load from a file, pass around, and render multiple times with different data.

Building a minimal version teaches you `str` manipulation patterns, iterator chaining, and how professional template engines (Tera, Handlebars-rs, MiniJinja) work under the hood.

## The Intuition

Mail merge. You have a letter template: "Dear {{name}}, your order {{order_id}} has shipped." You run it through the merge engine with a spreadsheet of customer data. Each row fills in the placeholders; the structure stays the same. The template is separate from the data.

## How It Works in Rust

1. **Find and replace with a `HashMap`**:
   ```rust
   fn render(template: &str, vars: &HashMap<&str, &str>) -> String {
       let mut result = template.to_string();
       for (key, val) in vars {
           result = result.replace(&format!("{{{{{}}}}}", key), val);
       }
       result
   }
   ```
2. **Single-pass with `find`** — more efficient for large templates:
   ```rust
   fn render(template: &str, vars: &HashMap<&str, &str>) -> String {
       let mut out = String::with_capacity(template.len());
       let mut rest = template;
       while let Some(start) = rest.find("{{") {
           out.push_str(&rest[..start]);
           let end = rest[start..].find("}}").map(|i| start + i + 2).unwrap_or(rest.len());
           let key = &rest[start+2..end-2];
           out.push_str(vars.get(key).copied().unwrap_or(""));
           rest = &rest[end..];
       }
       out.push_str(rest);
       out
   }
   ```
3. **Usage**:
   ```rust
   let mut vars = HashMap::new();
   vars.insert("name", "Alice");
   vars.insert("lang", "Rust");
   let result = render("Hello {{name}}, welcome to {{lang}}!", &vars);
   ```

## What This Unlocks

- **Runtime-configurable output** — templates loaded from files or databases, not hardcoded `format!` calls.
- **Separation of concerns** — designers edit templates; engineers supply data. No recompilation for copy changes.
- **Crate literacy** — once you've written a minimal version, Tera and MiniJinja are easy to evaluate and adopt.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Template engine | `Printf`-style or `Format` | Manual find/replace or `tera`/`minijinja` crates |
| Placeholder syntax | `%s`, `%d` positional | Custom `{{key}}` named |
| Variable map | `Hashtbl` | `HashMap<&str, &str>` |
| Crate option | `jingoo` | `tera`, `handlebars`, `minijinja` |
