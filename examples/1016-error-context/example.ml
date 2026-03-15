(* 1016: Error Context
   Attach a breadcrumb-style context trail to errors as they propagate.
   Each layer adds its own context; the display shows the full chain.
   Mirrors Rust's anyhow::Context / thiserror extension pattern. *)

type error_with_context = {
  message: string;
  context: string list;  (* oldest context last, most recent first *)
}

let make_error message = { message; context = [] }

let with_ctx ctx err = { err with context = ctx :: err.context }

let string_of_error_with_context e =
  match e.context with
  | [] -> e.message
  | ctxs ->
    (* contexts stored in push order — reverse to read innermost first *)
    let chain = List.rev ctxs in
    Printf.sprintf "%s: %s" (String.concat " -> " chain) e.message

(* Extension helper for Result — add context on error *)
let context ctx result =
  Result.map_error (with_ctx ctx) result

let with_context f result =
  Result.map_error (fun e -> with_ctx (f ()) e) result

(* Low-level functions *)
let read_file path =
  if path = "/missing" then Error (make_error "file not found")
  else Ok "42"

let parse_config content =
  match int_of_string_opt content with
  | Some n -> Ok n
  | None   -> Error (make_error (Printf.sprintf "invalid number: %s" content))

(* Approach 2: chaining contexts *)
let load_setting path =
  let ( let* ) = Result.bind in
  let* content = context "reading config" (read_file path) in
  let* value   = context "parsing config" (parse_config content) in
  Ok value

let init_system path =
  context "system init" (load_setting path)

let () =
  (* no context *)
  let e = make_error "oops" in
  assert (string_of_error_with_context e = "oops");

  (* single context *)
  let e2 = with_ctx "loading" (make_error "oops") in
  assert (string_of_error_with_context e2 = "loading: oops");

  (* nested context *)
  (match init_system "/missing" with
   | Error e ->
     assert (List.length e.context = 2);
     let s = string_of_error_with_context e in
     assert (String.length (let _ = Str.search_forward (Str.regexp "system init") s 0 in s) > 0
             || String.sub s 0 4 = "syst")
   | _ -> assert false);

  assert (init_system "/ok" = Ok 42);

  (* with_context (lazy) *)
  let r : (int, error_with_context) result = Error (make_error "base") in
  let r2 = with_context (fun () -> Printf.sprintf "dynamic context %d" 42) r in
  (match r2 with
   | Error e -> assert (List.mem "dynamic context 42" e.context)
   | _ -> assert false);

  Printf.printf "init_system /ok: %s\n"
    (match init_system "/ok" with
     | Ok n -> string_of_int n
     | Error e -> string_of_error_with_context e)
