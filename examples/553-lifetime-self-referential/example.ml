(* Self-referential structs in OCaml -- GC handles easily *)
type node = {
  mutable value: int;
  mutable self_ref: node option;
}

let make_self_ref v =
  let n = { value = v; self_ref = None } in
  n.self_ref <- Some n;  (* points to itself -- GC traces this cycle *)
  n

let () =
  let n = make_self_ref 42 in
  Printf.printf "value: %d\n" n.value;
  match n.self_ref with
  | Some self_ -> Printf.printf "self.value: %d\n" self_.value
  | None -> ()
