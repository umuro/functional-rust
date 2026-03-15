(* 278. Getting the last element - OCaml *)

let last = function
  | [] -> None
  | lst -> Some (List.nth lst (List.length lst - 1))

let () =
  let nums = [1; 2; 3; 4; 5] in
  Printf.printf "Last: %s\n" (match last nums with Some n -> string_of_int n | None -> "None");
  Printf.printf "Last of []: %s\n" (match last [] with Some _ -> "Some" | None -> "None");

  let words = ["apple"; "banana"; "cherry"] in
  Printf.printf "Last word: %s\n"
    (match last words with Some w -> w | None -> "None");

  (* Last filtered element *)
  let last_even = last (List.filter (fun x -> x mod 2 = 0) nums) in
  Printf.printf "Last even: %s\n"
    (match last_even with Some n -> string_of_int n | None -> "None")
