(* 093: Windows and Chunks *)

let windows n lst =
  let rec aux acc = function
    | [] -> List.rev acc
    | _ :: rest as l ->
      let window = List.filteri (fun i _ -> i < n) l in
      if List.length window = n then aux (window :: acc) rest
      else List.rev acc
  in
  aux [] lst

let chunks n lst =
  let rec aux acc current count = function
    | [] -> List.rev (if current = [] then acc else List.rev current :: acc)
    | x :: xs ->
      if count = n then aux (List.rev current :: acc) [x] 1 xs
      else aux acc (x :: current) (count + 1) xs
  in
  aux [] [] 0 lst

(* Tests *)
let () =
  assert (windows 3 [1;2;3;4;5] = [[1;2;3];[2;3;4];[3;4;5]]);
  assert (windows 2 [1;2;3] = [[1;2];[2;3]]);
  assert (chunks 2 [1;2;3;4;5] = [[1;2];[3;4];[5]]);
  assert (chunks 3 [1;2;3;4;5;6] = [[1;2;3];[4;5;6]]);
  Printf.printf "✓ All tests passed\n"
