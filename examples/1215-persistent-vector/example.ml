(* Simplified persistent vector: Nil | One | Two carve the tree,
   push/pop return new vectors, old versions keep pointing at their
   own subtrees.  No mutation anywhere. *)

type 'a pvec = Nil | One of 'a | Two of 'a pvec * 'a pvec

let rec length = function
  | Nil -> 0
  | One _ -> 1
  | Two (l, r) -> length l + length r

let push v x = match v with Nil -> One x | _ -> Two (v, One x)

let rec pop = function
  | Nil -> None
  | One x -> Some (x, Nil)
  | Two (l, r) -> (
      match pop r with
      | None -> None
      | Some (x, Nil) -> Some (x, l)
      | Some (x, r') -> Some (x, Two (l, r')))

let from_list xs = List.fold_left push Nil xs

let rec to_list = function
  | Nil -> []
  | One x -> [ x ]
  | Two (l, r) -> to_list l @ to_list r

let () =
  let v = from_list [ 1; 2; 3; 4; 5 ] in
  assert (length v = 5);
  assert (to_list v = [ 1; 2; 3; 4; 5 ]);
  let rec drain v acc =
    match pop v with Some (x, rest) -> drain rest (x :: acc) | None -> acc
  in
  (* drain yields LIFO, then acc is reversed back to chronological pop order *)
  assert (List.rev (drain v []) = [ 5; 4; 3; 2; 1 ]);
  (* persistence: v1 still has 3 elements after pushing into v2 *)
  let v1 = from_list [ 10; 20; 30 ] in
  let v2 = push v1 40 in
  assert (to_list v1 = [ 10; 20; 30 ]);
  assert (to_list v2 = [ 10; 20; 30; 40 ]);
  print_endline "ok"
