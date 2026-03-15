(* 273. Debugging iterators with inspect() - OCaml *)

let tap f x = f x; x

let () =
  let result =
    [1; 2; 3; 4; 5; 6; 7; 8; 9; 10]
    |> List.map (tap (fun x -> Printf.printf "[in:%d] " x))
    |> List.filter (fun x -> x mod 2 = 0)
    |> List.map (tap (fun x -> Printf.printf "[even:%d] " x))
    |> List.map (fun x -> x * x)
  in
  print_newline ();
  Printf.printf "Final: %s\n"
    (String.concat ", " (List.map string_of_int result));

  let log_negatives lst =
    List.filter_map (fun x ->
      if x < 0 then (Printf.printf "Warn: negative %d\n" x; None)
      else Some x
    ) lst
  in
  let cleaned = log_negatives [-1; 2; -3; 4; 5] in
  Printf.printf "Cleaned: %s\n"
    (String.concat ", " (List.map string_of_int cleaned))
