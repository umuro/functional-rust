(* 268. Splitting pairs with unzip() - OCaml *)

let () =
  let pairs = [(1, "one"); (2, "two"); (3, "three")] in
  let (nums, words) = List.split pairs in
  Printf.printf "Nums: %s\n" (String.concat ", " (List.map string_of_int nums));
  Printf.printf "Words: %s\n" (String.concat ", " words);

  let xs = [1; 2; 3] in
  let ys = ["a"; "b"; "c"] in
  let (xs2, ys2) = List.split (List.combine xs ys) in
  Printf.printf "Roundtrip OK: %b\n" (xs = xs2 && ys = ys2);

  let scores = [("Alice", 95); ("Bob", 87); ("Carol", 92)] in
  let (names, vals) = List.split scores in
  let avg = List.fold_left (+) 0 vals / List.length vals in
  Printf.printf "Students: %s, Average: %d\n"
    (String.concat ", " names) avg
