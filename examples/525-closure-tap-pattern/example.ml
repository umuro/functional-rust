(* Tap pattern in OCaml *)
let tap f x = f x; x

(* Debugging pipeline *)
let debug label x =
  Printf.printf "[%s]: %s\n" label (string_of_int x);
  x

let () =
  let result =
    5
    |> (fun x -> x * 2)
    |> tap (fun x -> Printf.printf "after double: %d\n" x)
    |> (fun x -> x + 1)
    |> tap (fun x -> Printf.printf "after inc: %d\n" x)
    |> (fun x -> x * x)
  in
  Printf.printf "result: %d\n" result;

  let nums = [1;2;3;4;5] in
  let sum = List.fold_left (fun acc x ->
    tap (fun s -> Printf.printf "running sum: %d\n" s) (acc + x)
  ) 0 nums in
  Printf.printf "final sum: %d\n" sum
