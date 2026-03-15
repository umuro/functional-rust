(* 492: OsStr / OsString — platform string handling in OCaml *)
(* Rust distinguishes OsStr (platform-native, possibly non-UTF-8) from &str.
   On Linux, filenames are arbitrary bytes. On Windows they are WTF-16.

   In OCaml, the Filename module and Sys module work with plain OCaml strings,
   which are byte sequences. OCaml does not have a separate OsStr type.
   The equivalent OCaml approach:
   - Use plain strings for path operations (works correctly on Linux/macOS).
   - Use Bytes for raw byte-level path manipulation if needed.
   - On Windows, use the Windows API wrappers or the `win-utf8` approach. *)

(* Demonstrate that OCaml Filename functions work on plain strings *)

(* "OsStr roundtrip": path → basename → still a valid string *)
let osstr_roundtrip s =
  (* On Unix, Sys.file_exists / Filename.basename work on arbitrary-byte strings *)
  let base = Filename.basename s in
  base  (* returned as a plain string — always valid in OCaml *)

(* Extract extension (mirrors the OsStr path extension example) *)
let path_extension path =
  let base = Filename.basename path in
  match String.rindex_opt base '.' with
  | None   -> None
  | Some 0 -> None
  | Some i -> Some (String.sub base (i + 1) (String.length base - i - 1))

(* Lossless string-to-string: OCaml has no lossy conversion concept,
   since its strings hold arbitrary bytes already *)
let to_string_lossless s = s

(* Environment variable access — returns a string option, no OsStr needed *)
let get_env_var name =
  match Sys.getenv_opt name with
  | Some v -> v
  | None   -> ""

let () =
  (* roundtrip *)
  let s = "hello" in
  assert (osstr_roundtrip s = "hello");
  Printf.printf "osstr_roundtrip: %s\n" (osstr_roundtrip s);

  (* path extension via Filename *)
  assert (path_extension "f.rs" = Some "rs");
  assert (path_extension "noext" = None);
  print_endline "path_extension: ok";

  (* lossy (no-op in OCaml — strings are bytes) *)
  let hi = to_string_lossless "hi" in
  assert (hi = "hi");
  print_endline "to_string_lossless: ok";

  (* environment variable *)
  let path = get_env_var "PATH" in
  Printf.printf "PATH starts with: %s...\n"
    (if String.length path > 20 then String.sub path 0 20 else path);

  print_endline "All assertions passed."
