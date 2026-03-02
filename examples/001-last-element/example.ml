(* Find the last element of a list *)

(* Solution 1: Pattern matching (idiomatic OCaml) *)
let rec last = function
  | [] -> None
  | [x] -> Some x
  | _ :: t -> last t

(* Solution 2: Using standard library *)
let last_stdlib lst =
  match List.rev lst with
  | [] -> None
  | h :: _ -> Some h

(* Solution 3: Tail-recursive (better for long lists) *)
let last_tail lst =
  let rec aux acc = function
    | [] -> acc
    | h :: t -> aux (Some h) t
  in
  aux None lst

(* Tests *)
let () =
  assert (last [] = None);
  assert (last [1] = Some 1);
  assert (last [1; 2; 3; 4] = Some 4);
  assert (last ["a"; "b"; "c"; "d"] = Some "d");
  
  print_endline "✓ All tests passed"

(* Output:
   ✓ All tests passed
*)
