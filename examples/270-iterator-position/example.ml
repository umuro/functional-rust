(* 270. Finding index with position() - OCaml *)

let position pred lst =
  let rec aux i = function
    | [] -> None
    | x :: xs -> if pred x then Some i else aux (i + 1) xs
  in
  aux 0 lst

let () =
  let nums = [10; 20; 30; 40; 50] in
  (match position (fun x -> x > 25) nums with
  | Some i -> Printf.printf "First >25 at index %d\n" i
  | None -> print_endline "Not found");

  let words = ["apple"; "banana"; "cherry"] in
  (match position (fun w -> w = "banana") words with
  | Some i -> Printf.printf "banana at index %d\n" i
  | None -> print_endline "Not found");

  (match position (fun x -> x = 30) nums with
  | Some i ->
    let before = List.filteri (fun j _ -> j < i) nums in
    Printf.printf "Before 30: %s\n"
      (String.concat ", " (List.map string_of_int before))
  | None -> print_endline "30 not found")
