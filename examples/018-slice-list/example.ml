(* Slice List *)
(* OCaml 99 Problems #18 *)

let slice_list lst i k =
  if i < 1 || k < i then []
  else
    let rec aux acc n = function
      | [] -> List.rev acc
      | x :: t ->
        if n > k then List.rev acc
        else if n < i then aux acc (n + 1) t
        else aux (x :: acc) (n + 1) t
    in
    aux [] 1 lst

(* Tests *)
let () =
  assert (slice_list [1; 2; 3; 4; 5; 6; 7; 8; 9; 10] 3 7 = [3; 4; 5; 6; 7]);
  assert (slice_list [10; 20; 30] 2 2 = [20]);
  assert (slice_list [1; 2; 3] 1 3 = [1; 2; 3]);
  assert (slice_list [1; 2; 3] 2 100 = [2; 3]);
  assert (slice_list [1; 2; 3] 0 2 = []);
  print_endline "✓ OCaml tests passed"
