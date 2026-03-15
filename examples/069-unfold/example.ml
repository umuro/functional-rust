(* 069: Unfold — generate a sequence from a seed *)

(* Approach 1: Manual list unfold *)
let rec unfold f seed =
  match f seed with
  | None -> []
  | Some (value, next_seed) -> value :: unfold f next_seed

(* Generate range [a, b) *)
let range a b =
  unfold (fun i -> if i >= b then None else Some (i, i + 1)) a

(* Fibonacci sequence up to limit *)
let fibs_up_to limit =
  unfold (fun (a, b) ->
    if a > limit then None
    else Some (a, (b, a + b))
  ) (0, 1)

(* Approach 2: Collatz sequence *)
let collatz n =
  unfold (fun n ->
    if n = 1 then Some (1, 0)
    else if n = 0 then None
    else Some (n, if n mod 2 = 0 then n / 2 else 3 * n + 1)
  ) n

(* Approach 3: Using Seq.unfold *)
let range_seq a b =
  Seq.unfold (fun i -> if i >= b then None else Some (i, i + 1)) a

let seq_to_list s = List.of_seq s

(* Tests *)
let () =
  assert (range 1 6 = [1; 2; 3; 4; 5]);
  assert (range 5 5 = []);
  assert (fibs_up_to 20 = [0; 1; 1; 2; 3; 5; 8; 13]);
  assert (collatz 6 = [6; 3; 10; 5; 16; 8; 4; 2; 1]);
  assert (seq_to_list (range_seq 1 4) = [1; 2; 3]);
  Printf.printf "✓ All tests passed\n"
