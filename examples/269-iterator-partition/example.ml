(* 269. Splitting by predicate with partition() - OCaml *)

let () =
  let nums = List.init 10 (fun i -> i + 1) in
  let (evens, odds) = List.partition (fun x -> x mod 2 = 0) nums in
  Printf.printf "Evens: %s\n" (String.concat ", " (List.map string_of_int evens));
  Printf.printf "Odds:  %s\n" (String.concat ", " (List.map string_of_int odds));

  let words = ["hi"; "hello"; "yo"; "world"; "hey"; "programming"] in
  let (short, long) = List.partition (fun w -> String.length w <= 3) words in
  Printf.printf "Short: %s\n" (String.concat ", " short);
  Printf.printf "Long: %s\n" (String.concat ", " long);

  let data = [-3; 1; -1; 4; -1; 5; 9; -2; 6] in
  let (pos, neg) = List.partition (fun x -> x >= 0) data in
  Printf.printf "Sum pos: %d, Sum neg: %d\n"
    (List.fold_left (+) 0 pos)
    (List.fold_left (+) 0 neg)
