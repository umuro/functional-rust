(* 1005: Error Chaining
   Wrap low-level errors with context using map_error / Result.bind.
   The 'with_context' pattern attaches a context string to any error. *)

type io_error =
  | NotFound
  | Corrupted of string

let string_of_io_error = function
  | NotFound       -> "not found"
  | Corrupted s    -> Printf.sprintf "corrupted: %s" s

(* App error bundles context + underlying io error *)
type app_error = {
  context: string;
  source: io_error;
}

let string_of_app_error e =
  Printf.sprintf "%s: %s" e.context (string_of_io_error e.source)

(* Low-level function *)
let read_file path =
  match path with
  | "/missing" -> Error NotFound
  | "/bad"     -> Error (Corrupted "invalid utf-8")
  | _          -> Ok "data"

(* Approach 1: map_error to add context *)
let load_config path =
  Result.map_error (fun source ->
    { context = Printf.sprintf "loading %s" path; source }
  ) (read_file path)

(* Approach 2: helper that mirrors the with_context extension trait *)
let with_context ctx result =
  Result.map_error (fun source -> { context = ctx (); source }) result

let load_config_ext path =
  with_context (fun () -> Printf.sprintf "loading %s" path) (read_file path)

let () =
  assert (load_config "/ok" = Ok "data");

  (match load_config "/missing" with
   | Error e ->
     assert (e.context = "loading /missing");
     (match e.source with NotFound -> () | _ -> assert false)
   | _ -> assert false);

  (match load_config "/bad" with
   | Error e ->
     let s = string_of_app_error e in
     assert (String.length s > 0);
     assert (try let _ = Str.search_forward (Str.regexp "corrupted") s 0 in true
             with Not_found -> false || String.sub s 0 5 = "loadi")
   | _ -> assert false);

  assert (load_config_ext "/ok" = Ok "data");
  (match load_config_ext "/missing" with
   | Error e -> assert (e.context = "loading /missing")
   | _ -> assert false);

  let err = { context = "loading /missing"; source = NotFound } in
  assert (string_of_app_error err = "loading /missing: not found");

  Printf.printf "load_config /ok: %s\n"
    (match load_config "/ok" with Ok s -> s | Error e -> string_of_app_error e)
