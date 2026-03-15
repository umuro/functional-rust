(* 276. Custom comparison min_by() and max_by() - OCaml *)

let min_by cmp lst =
  match lst with
  | [] -> None
  | x :: xs -> Some (List.fold_left (fun a y -> if cmp y a < 0 then y else a) x xs)

let max_by cmp lst =
  match lst with
  | [] -> None
  | x :: xs -> Some (List.fold_left (fun a y -> if cmp y a > 0 then y else a) x xs)

let () =
  let floats = [3.14; 1.41; 2.71; 1.73; 0.57] in
  (match min_by Float.compare floats with Some v -> Printf.printf "Min: %.2f\n" v | None -> ());
  (match max_by Float.compare floats with Some v -> Printf.printf "Max: %.2f\n" v | None -> ());

  let words = ["banana"; "apple"; "fig"; "kiwi"; "cherry"] in
  let cmp_len a b =
    let c = compare (String.length a) (String.length b) in
    if c <> 0 then c else String.compare a b
  in
  (match min_by cmp_len words with Some w -> Printf.printf "Min by len+alpha: %s\n" w | None -> ());

  let nums = [5; 2; 8; 1; 9; 3] in
  (match min_by (fun a b -> -compare a b) nums with
  | Some n -> Printf.printf "Max via reversed min: %d\n" n
  | None -> ())
