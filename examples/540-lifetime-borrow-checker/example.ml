(* Borrow checker concepts in OCaml — enforced by discipline, not type system *)

(* OCaml: you CAN have multiple mutable aliases — programmer must be careful *)
let () =
  let x = ref 42 in
  let alias1 = x in  (* both point to same ref *)
  let alias2 = x in

  (* This is "data race" prone in concurrent code — OCaml allows it *)
  alias1 := 100;
  alias2 := 200;
  Printf.printf "x = %d (aliased mutation — potentially unsafe)\n" !x;

  (* Safe pattern: use x, then modify — explicit discipline *)
  let data = ref [| 1; 2; 3 |] in
  let snapshot = Array.copy !data in  (* defensive copy, not a borrow *)
  !data.(0) <- 99;
  Printf.printf "snapshot[0] = %d, data[0] = %d\n" snapshot.(0) !data.(0)
