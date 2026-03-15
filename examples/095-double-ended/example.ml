(* 095: Double-Ended Iteration *)
(* OCaml lists are singly-linked — no efficient reverse iteration *)
(* We simulate with arrays *)

let iter_both arr =
  let n = Array.length arr in
  let front = ref 0 in
  let back = ref (n - 1) in
  let next_front () =
    if !front > !back then None
    else (let v = arr.(!front) in incr front; Some v)
  in
  let next_back () =
    if !back < !front then None
    else (let v = arr.(!back) in decr back; Some v)
  in
  (next_front, next_back)

let is_palindrome_arr arr =
  let n = Array.length arr in
  let rec check i = i >= n / 2 || (arr.(i) = arr.(n - 1 - i) && check (i + 1)) in
  check 0

(* Tests *)
let () =
  let arr = [|1; 2; 3; 4; 5|] in
  let (nf, nb) = iter_both arr in
  assert (nf () = Some 1);
  assert (nb () = Some 5);
  assert (nf () = Some 2);
  assert (nb () = Some 4);
  assert (is_palindrome_arr [|1; 2; 3; 2; 1|]);
  assert (not (is_palindrome_arr [|1; 2; 3|]));
  Printf.printf "✓ All tests passed\n"
