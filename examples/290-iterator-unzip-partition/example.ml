(* 290. Advanced splitting patterns - OCaml *)

type ('a, 'b) either = Left of 'a | Right of 'b

let partition_map f lst =
  let rec aux lefts rights = function
    | [] -> (List.rev lefts, List.rev rights)
    | x :: xs ->
      match f x with
      | Left l -> aux (l :: lefts) rights xs
      | Right r -> aux lefts (r :: rights) xs
  in
  aux [] [] lst

let () =
  let data = ["1"; "two"; "3"; "four"; "5"] in
  let (nums, words) = partition_map (fun s ->
    match int_of_string_opt s with
    | Some n -> Left n
    | None -> Right s
  ) data in
  Printf.printf "Numbers: %s\n" (String.concat ", " (List.map string_of_int nums));
  Printf.printf "Words: %s\n" (String.concat ", " words);

  (* Trisect: neg, zero, pos *)
  let nums = [-3; 0; 1; -1; 0; 5; -2; 3] in
  let neg = List.filter (fun x -> x < 0) nums in
  let zero = List.filter (fun x -> x = 0) nums in
  let pos = List.filter (fun x -> x > 0) nums in
  Printf.printf "Neg: %s, Zero: %d, Pos: %s\n"
    (String.concat "," (List.map string_of_int neg))
    (List.length zero)
    (String.concat "," (List.map string_of_int pos))
