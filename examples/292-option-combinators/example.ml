(* 292. map(), filter(), and_then() on Option - OCaml *)

let () =
  let some_5 = Some 5 in
  let none : int option = None in

  (* map: transforms Some value, passes through None *)
  Printf.printf "map Some(5)*2: %s\n"
    (match Option.map (fun x -> x * 2) some_5 with Some n -> string_of_int n | None -> "None");
  Printf.printf "map None*2: %s\n"
    (match Option.map (fun x -> x * 2) none with Some n -> string_of_int n | None -> "None");

  (* bind/and_then: chain optional computations *)
  let safe_div x y = if y = 0 then None else Some (x / y) in
  let result = Option.bind some_5 (fun n -> safe_div 10 n) in
  Printf.printf "10/5 chained: %s\n"
    (match result with Some n -> string_of_int n | None -> "None");

  (* filter *)
  let even = Option.filter (fun x -> x mod 2 = 0) some_5 in
  Printf.printf "filter even Some(5): %s\n"
    (match even with Some n -> string_of_int n | None -> "None");
  let even6 = Option.filter (fun x -> x mod 2 = 0) (Some 6) in
  Printf.printf "filter even Some(6): %s\n"
    (match even6 with Some n -> string_of_int n | None -> "None")
