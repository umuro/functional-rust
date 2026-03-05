(* Counting Sort in OCaml *)

let counting_sort arr =
  if Array.length arr = 0 then () else
  let max_val = Array.fold_left max arr.(0) arr in
  let count = Array.make (max_val + 1) 0 in
  Array.iter (fun x -> count.(x) <- count.(x) + 1) arr;
  let i = ref 0 in
  Array.iteri (fun v cnt ->
    for _ = 1 to cnt do arr.(!i) <- v; incr i done
  ) count

let () =
  let arr = [|4; 2; 2; 8; 3; 3; 1|] in
  counting_sort arr;
  Printf.printf "Sorted: [|%s|]\n"
    (String.concat "; " (Array.to_list (Array.map string_of_int arr)))
