(* Typed event handler with priority in OCaml *)
type priority = High | Normal | Low

type 'a handler = {
  priority: priority;
  handle: 'a -> bool;  (* returns true to stop propagation *)
}

let priority_val = function High -> 0 | Normal -> 1 | Low -> 2

let dispatch handlers event =
  let sorted = List.sort (fun a b -> compare (priority_val a.priority) (priority_val b.priority)) handlers in
  List.exists (fun h -> h.handle event) sorted

type ui_event = Click of int * int | KeyPress of char

let () =
  let handlers = [
    { priority = Normal; handle = (function Click _ -> Printf.printf "Normal click\n"; false | _ -> false) };
    { priority = High;   handle = (function Click (x,y) when x < 0 -> Printf.printf "Out of bounds! (%d,%d)\n" x y; true | _ -> false) };
    { priority = Low;    handle = (function _ -> Printf.printf "Fallback handler\n"; false) };
  ] in
  let _ = dispatch handlers (Click (10, 20)) in
  let _ = dispatch handlers (Click (-1, 5)) in
  ()
