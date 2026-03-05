(* Observer pattern using closures in OCaml *)

type 'a event_system = {
  mutable handlers: ('a -> unit) list;
}

let make_event_system () = { handlers = [] }

let subscribe sys handler =
  sys.handlers <- handler :: sys.handlers

let emit sys event =
  List.iter (fun h -> h event) sys.handlers

(* Typed event *)
type button_event = Click of int * int | Hover of int * int | KeyPress of char

let () =
  let sys = make_event_system () in

  (* Logger handler *)
  subscribe sys (function
    | Click (x, y) -> Printf.printf "[LOG] Click at (%d,%d)\n" x y
    | Hover (x, y) -> Printf.printf "[LOG] Hover at (%d,%d)\n" x y
    | KeyPress c   -> Printf.printf "[LOG] Key: %c\n" c);

  (* Counter handler (stateful) *)
  let click_count = ref 0 in
  subscribe sys (function
    | Click _ -> incr click_count
    | _ -> ());

  (* Fire events *)
  emit sys (Click (10, 20));
  emit sys (Hover (15, 25));
  emit sys (Click (30, 40));
  emit sys (KeyPress 'a');

  Printf.printf "Total clicks: %d\n" !click_count
