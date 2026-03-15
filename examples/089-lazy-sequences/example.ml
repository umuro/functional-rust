(* 089: Lazy Sequences *)

(* Approach 1: Lazy list via thunks *)
let naturals =
  let rec aux n () = Seq.Cons (n, aux (n + 1)) in
  aux 0

let take n s = List.of_seq (Seq.take n s)

(* Approach 2: Lazy fibonacci *)
let fibs =
  let rec aux a b () = Seq.Cons (a, aux b (a + b)) in
  aux 0 1

(* Approach 3: from_fn equivalent *)
let from_fn f =
  let state = ref 0 in
  let rec aux () =
    match f !state with
    | None -> Seq.Nil
    | Some v -> state := !state + 1; Seq.Cons (v, aux)
  in
  aux

let powers_of_2 = from_fn (fun n -> if n >= 10 then None else Some (1 lsl n))

(* Tests *)
let () =
  assert (take 5 naturals = [0; 1; 2; 3; 4]);
  assert (take 8 fibs = [0; 1; 1; 2; 3; 5; 8; 13]);
  assert (take 4 powers_of_2 = [1; 2; 4; 8]);
  Printf.printf "✓ All tests passed\n"
