(* 735: Typestate builder — required fields enforced at compile time in OCaml *)
(* Rust uses phantom type parameters (Set / Unset) to ensure build() is only
   callable when all required fields have been provided.

   In OCaml we use the same phantom-type approach:
   type ('has_host, 'has_port) builder
   The 'has_host and 'has_port type parameters are abstract tags that are
   instantiated to different types when the corresponding field is set. *)

(* ── State marker types ───────────────────────────────────────────────────── *)
type set   = private Set_
type unset = private Unset_

(* ── The product we're building ─────────────────────────────────────────── *)
type http_client = {
  host        : string;
  port        : int;
  timeout_ms  : int;
  max_retries : int;
}

(* ── Builder ─────────────────────────────────────────────────────────────── *)
(* The phantom parameters track which required fields have been set.
   ('h, 'p) means has_host=h, has_port=p. *)

type ('h, 'p) builder = {
  b_host        : string option;
  b_port        : int option;
  b_timeout_ms  : int;
  b_max_retries : int;
}

(* Entry point: both required fields unset *)
let new_builder () : (unset, unset) builder = {
  b_host        = None;
  b_port        = None;
  b_timeout_ms  = 5_000;
  b_max_retries = 3;
}

(* Setting host transitions 'has_host from unset → set.
   The port phantom type is preserved unchanged. *)
let with_host h (b : (unset, 'p) builder) : (set, 'p) builder =
  { b with b_host = Some h }

(* Setting port transitions 'has_port from unset → set. *)
let with_port p (b : ('h, unset) builder) : ('h, set) builder =
  { b with b_port = Some p }

(* Optional setters: available in any state *)
let with_timeout_ms ms b = { b with b_timeout_ms = ms }
let with_max_retries n  b = { b with b_max_retries = n }

(* build() is only callable when BOTH required fields are set.
   Calling it on an (unset, _) or (_, unset) builder is a type error. *)
let build (b : (set, set) builder) : http_client = {
  host        = Option.get b.b_host;
  port        = Option.get b.b_port;
  timeout_ms  = b.b_timeout_ms;
  max_retries = b.b_max_retries;
}

let () =
  (* Full build succeeds *)
  let c = new_builder ()
    |> with_host "localhost"
    |> with_port 3000
    |> build
  in
  assert (c.host = "localhost");
  assert (c.port = 3000);
  assert (c.timeout_ms = 5_000);
  print_endline "full build: ok";

  (* Custom timeout and retries *)
  let c2 = new_builder ()
    |> with_host "example.com"
    |> with_port 443
    |> with_timeout_ms 1_000
    |> with_max_retries 0
    |> build
  in
  assert (c2.timeout_ms = 1_000);
  assert (c2.max_retries = 0);
  print_endline "custom options: ok";

  (* Order of host/port does not matter *)
  let c3 = new_builder () |> with_host "a" |> with_port 1 |> build in
  let c4 = new_builder () |> with_port 1 |> with_host "a" |> build in
  assert (c3.host = c4.host);
  assert (c3.port = c4.port);
  print_endline "order does not matter: ok";

  (* The following would be a COMPILE ERROR:
       new_builder () |> with_host "x" |> build   (* port still unset *)
       new_builder () |> with_port 80 |> build    (* host still unset *)
  *)

  print_endline "All assertions passed."
