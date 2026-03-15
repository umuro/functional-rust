(* OCaml: linked list is the default! *)

let () =
  let lst = [1;2;3;4;5] in
  List.iter (Printf.printf "%d ") lst; print_newline ();
  let sum = List.fold_left (+) 0 lst in
  Printf.printf "Sum: %d\n" sum;
  let rev = List.rev lst in
  List.iter (Printf.printf "%d ") rev; print_newline ();
  match List.filter (fun x -> x mod 2 = 0) lst with
  | evens -> List.iter (Printf.printf "%d ") evens; print_newline ()
