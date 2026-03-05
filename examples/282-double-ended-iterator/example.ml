(* 282. DoubleEndedIterator and rev() - OCaml *)
(* OCaml: List.rev creates new reversed list *)

let () =
  let nums = [1; 2; 3; 4; 5] in
  let reversed = List.rev nums in
  Printf.printf "Reversed: %s\n"
    (String.concat ", " (List.map string_of_int reversed));

  (* Consume from both ends simultaneously *)
  let arr = Array.of_list nums in
  let n = Array.length arr in
  let front = ref 0 and back = ref (n - 1) in
  Printf.printf "From both ends: " ;
  while !front <= !back do
    if !front = !back then Printf.printf "%d " arr.(!front)
    else Printf.printf "%d %d " arr.(!front) arr.(!back);
    incr front; decr back
  done;
  print_newline ();

  (* Reverse iterate *)
  for i = n - 1 downto 0 do
    Printf.printf "%d " arr.(i)
  done;
  print_newline ()
