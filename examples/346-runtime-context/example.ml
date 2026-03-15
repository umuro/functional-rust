(* 346: Runtime Context
   Thread-local (Domain-local in OCaml 5) context propagation.
   Each domain has its own context slot; nesting saves/restores the old value. *)

(* Domain-local storage for an optional string context *)
let context : string option Domain.DLS.key =
  Domain.DLS.new_key (fun () -> None)

let set_context ctx = Domain.DLS.set context (Some ctx)
let get_context ()  = Domain.DLS.get context
let clear_context ()= Domain.DLS.set context None

(* Run [f] with [ctx] active, then restore the previous context *)
let with_context ctx f =
  let old = get_context () in
  set_context ctx;
  let result = f () in
  (match old with
   | Some c -> set_context c
   | None   -> clear_context ());
  result

let () =
  (* Basic set / get / clear *)
  set_context "test";
  assert (get_context () = Some "test");
  clear_context ();
  assert (get_context () = None);
  Printf.printf "context set/clear: ok\n%!";

  (* Nested with_context restores outer *)
  set_context "outer";
  let inner = with_context "inner" get_context in
  assert (inner = Some "inner");
  assert (get_context () = Some "outer");
  Printf.printf "nested context: inner=%s, restored=%s\n%!"
    (Option.value inner ~default:"none")
    (Option.value (get_context ()) ~default:"none");

  (* Each domain has its own context *)
  set_context "main-domain";
  let d = Domain.spawn (fun () ->
    (* New domain starts with None *)
    let initial = get_context () in
    set_context "child-domain";
    (initial, get_context ()))
  in
  let (initial, child_ctx) = Domain.join d in
  assert (initial = None);
  assert (child_ctx = Some "child-domain");
  (* Main domain context is unchanged *)
  assert (get_context () = Some "main-domain");
  Printf.printf "domain isolation: main=%s child=%s\n%!"
    (Option.value (get_context ()) ~default:"none")
    (Option.value child_ctx ~default:"none")
