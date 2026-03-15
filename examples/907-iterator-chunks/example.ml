(* 263. Fixed-size chunks iteration - OCaml *)

let chunks n lst =
  let rec aux acc current count = function
    | [] ->
      if current = [] then List.rev acc
      else List.rev (List.rev current :: acc)
    | x :: xs ->
      if count = n then aux (List.rev current :: acc) [x] 1 xs
      else aux acc (x :: current) (count + 1) xs
  in
  aux [] [] 0 lst

let () =
  let nums = [1; 2; 3; 4; 5; 6; 7] in
  let cs = chunks 3 nums in
  Printf.printf "Chunks of 3:\n";
  List.iter (fun chunk ->
    Printf.printf "[%s]\n" (String.concat "; " (List.map string_of_int chunk))
  ) cs;

  let sums = List.map (List.fold_left (+) 0) cs in
  Printf.printf "Chunk sums: %s\n"
    (String.concat ", " (List.map string_of_int sums));

  let batches = chunks 4 (List.init 10 (fun i -> i + 1)) in
  List.iteri (fun i batch ->
    Printf.printf "Batch %d: [%s]\n" i
      (String.concat ", " (List.map string_of_int batch))
  ) batches
