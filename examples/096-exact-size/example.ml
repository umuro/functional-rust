(* 096: Exact Size *)
(* OCaml: List.length is O(n), Array.length is O(1) *)

let array_len arr = Array.length arr  (* O(1) *)
let list_len lst = List.length lst    (* O(n) *)

(* Sized iteration *)
let enumerate lst =
  List.mapi (fun i x -> (i, x)) lst

let chunks_exact n lst =
  let len = List.length lst in
  let full = len / n in
  let rec aux acc i = function
    | _ when i >= full -> List.rev acc
    | lst ->
      let chunk = List.filteri (fun j _ -> j < n) lst in
      let rest = List.filteri (fun j _ -> j >= n) lst in
      aux (chunk :: acc) (i + 1) rest
  in
  aux [] 0 lst

(* Tests *)
let () =
  assert (array_len [|1;2;3|] = 3);
  assert (enumerate ["a";"b";"c"] = [(0,"a"); (1,"b"); (2,"c")]);
  assert (chunks_exact 2 [1;2;3;4;5] = [[1;2]; [3;4]]);
  Printf.printf "✓ All tests passed\n"
