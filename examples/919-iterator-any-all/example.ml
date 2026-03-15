(* 919: Existential checks — any (∃) and all (∀) — both short-circuit

   OCaml's List module: List.exists (any) and List.for_all (all).
   Both short-circuit: exists stops on first true, for_all on first false. *)

(* OCaml stdlib equivalents:
     List.exists   : ('a -> bool) -> 'a list -> bool   -- any
     List.for_all  : ('a -> bool) -> 'a list -> bool   -- all

   We also show custom implementations to see the short-circuit mechanics. *)

(* Custom any — stops at first true *)
let rec any pred = function
  | [] -> false
  | x :: _ when pred x -> true
  | _ :: rest -> any pred rest

(* Custom all — stops at first false *)
let rec all pred = function
  | [] -> true            (* vacuous truth: all [] p = true *)
  | x :: _ when not (pred x) -> false
  | _ :: rest -> all pred rest

(* none: ¬∃ = ∀¬ *)
let none pred lst = not (any pred lst)

(* count_if: how many elements satisfy pred *)
let count_if pred lst =
  List.fold_left (fun acc x -> if pred x then acc + 1 else acc) 0 lst

let () =
  (* any / exists *)
  assert (any (fun x -> x = 2) [1; 2; 3]);
  assert (not (any (fun x -> x = 9) [1; 2; 3]));
  assert (List.exists (fun x -> x = 2) [1; 2; 3]);

  (* all / for_all *)
  assert (all (fun x -> x mod 2 = 0) [2; 4; 6]);
  assert (not (all (fun x -> x mod 2 = 0) [1; 2; 3]));
  assert (List.for_all (fun x -> x mod 2 = 0) [2; 4; 6]);

  (* vacuous truth / empty *)
  assert (all (fun _ -> false) []);   (* vacuously true *)
  assert (not (any (fun _ -> true) []));   (* no elements *)

  (* none *)
  assert (none (fun x -> x < 0) [1; 2; 3]);
  assert (not (none (fun x -> x < 0) [1; -1; 3]));

  (* count_if *)
  assert (count_if (fun x -> x mod 2 = 0) [1; 2; 3; 4; 5; 6] = 3);

  (* short-circuit demonstration via side effect counter *)
  let steps = ref 0 in
  let _ = any (fun x -> incr steps; x = 3) [1; 2; 3; 4; 5] in
  assert (!steps = 3);  (* stopped after finding 3 *)

  let steps2 = ref 0 in
  let _ = all (fun x -> incr steps2; x < 10) [1; 2; 3; 11; 5] in
  assert (!steps2 = 4);  (* stopped after finding 11 *)

  print_endline "919-iterator-any-all: all tests passed"
