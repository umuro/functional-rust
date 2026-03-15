(* OCaml: thread-local context via Domain-local storage *)

let current_context : string option ref = ref None

let with_context name f =
  let prev = !current_context in
  current_context := Some name;
  let result = f () in
  current_context := prev;
  result

let get_context () =
  match !current_context with
  | Some name -> name
  | None -> "root"

let () =
  Printf.printf "Context: %s\n" (get_context ());
  with_context "worker-1" (fun () ->
    Printf.printf "Inside: %s\n" (get_context ());
    with_context "nested" (fun () ->
      Printf.printf "Nested: %s\n" (get_context ())
    );
    Printf.printf "Back to: %s\n" (get_context ())
  );
  Printf.printf "Back to: %s\n" (get_context ())
