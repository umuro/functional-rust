(* Higher-ranked types in OCaml using polymorphic record fields *)
(* Standard OCaml doesn't have rank-2 types without the forall trick *)

(* Simulating: a function that works for ANY type *)
type 'a identity = { apply: 'a -> 'a }
(* But we want ∀'a. 'a -> 'a: *)
type universal_id = { apply: 'a. 'a -> 'a }

let id = { apply = fun x -> x }

let () =
  Printf.printf "id int: %d\n" (id.apply 42);
  Printf.printf "id str: %s\n" (id.apply "hello");

  (* Polymorphic function applied to different types *)
  let process_with f =
    let s = f "hello" in
    let n = f 42 in  (* would fail without rank-2! *)
    ignore (s, n)
  in
  ignore process_with  (* OCaml can't do this without forall trick *)
