(* Higher-order functions in OCaml *)

(* Custom HOFs *)
let my_map f = List.map f
let my_filter pred = List.filter pred
let my_fold_left f init = List.fold_left f init
let my_for_all pred = List.for_all pred

(* Pipeline operator *)
let ( |> ) x f = f x

let () =
  let nums = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10] in

  (* Map *)
  let squares = List.map (fun x -> x * x) nums in
  Printf.printf "squares: [%s]\n" (String.concat ";" (List.map string_of_int squares));

  (* Filter *)
  let evens = List.filter (fun x -> x mod 2 = 0) nums in
  Printf.printf "evens: [%s]\n" (String.concat ";" (List.map string_of_int evens));

  (* Fold *)
  let sum = List.fold_left (+) 0 nums in
  Printf.printf "sum: %d\n" sum;

  (* Pipeline *)
  let result =
    nums
    |> List.filter (fun x -> x mod 2 = 0)
    |> List.map (fun x -> x * x)
    |> List.fold_left (+) 0
  in
  Printf.printf "sum of even squares: %d\n" result;

  (* any / all *)
  Printf.printf "any >5: %b\n" (List.exists (fun x -> x > 5) nums);
  Printf.printf "all >0: %b\n" (List.for_all (fun x -> x > 0) nums);

  (* flat_map *)
  let expanded = List.concat_map (fun x -> [x; x*10]) [1;2;3] in
  Printf.printf "flat_map: [%s]\n" (String.concat ";" (List.map string_of_int expanded))
