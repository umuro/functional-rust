(* Pipe operator in OCaml — built in as |> *)
let double x = x * 2
let add1 x = x + 1
let square x = x * x
let to_string x = string_of_int x

let () =
  let result = 3 |> double |> add1 |> square |> to_string in
  Printf.printf "3 |> double |> add1 |> square |> to_string = %s\n" result;

  (* Pipe with anonymous functions *)
  let processed =
    [1;2;3;4;5]
    |> List.filter (fun x -> x mod 2 = 0)
    |> List.map (fun x -> x * 3)
    |> List.fold_left (+) 0
  in
  Printf.printf "sum of tripled evens: %d\n" processed
