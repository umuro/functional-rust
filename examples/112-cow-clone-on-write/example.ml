(* Example 112: Cow<T> Clone-on-Write — Lazy Cloning *)

(* OCaml doesn't need CoW — GC handles sharing.
   But we can demonstrate the concept of deferred copying. *)

(* Approach 1: Conditional modification *)
let normalize_whitespace s =
  if String.contains s '\t' then
    String.map (fun c -> if c = '\t' then ' ' else c) s
  else
    s  (* no allocation if no tabs *)

let approach1 () =
  let clean = "hello world" in
  let dirty = "hello\tworld" in
  let r1 = normalize_whitespace clean in
  let r2 = normalize_whitespace dirty in
  assert (r1 = "hello world");
  assert (r2 = "hello world");
  Printf.printf "Clean: %s, Fixed: %s\n" r1 r2

(* Approach 2: Default with override *)
let with_default default_val override_opt =
  match override_opt with
  | Some v -> v
  | None -> default_val

let approach2 () =
  let default = "default_config" in
  let r1 = with_default default None in
  let r2 = with_default default (Some "custom") in
  assert (r1 = "default_config");
  assert (r2 = "custom");
  Printf.printf "r1=%s, r2=%s\n" r1 r2

(* Approach 3: Batch processing with conditional transform *)
let process_items items transform_fn =
  List.map (fun item ->
    if String.length item > 5 then transform_fn item
    else item
  ) items

let approach3 () =
  let items = ["hi"; "hello"; "extraordinary"] in
  let result = process_items items String.uppercase_ascii in
  assert (result = ["hi"; "hello"; "EXTRAORDINARY"]);
  Printf.printf "Processed: %s\n" (String.concat ", " result)

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
