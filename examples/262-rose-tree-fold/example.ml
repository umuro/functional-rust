type 'a rose = Rose of 'a * 'a rose list

let rec fold f (Rose (x, children)) =
  f x (List.map (fold f) children)

let size = fold (fun _ sizes -> 1 + List.fold_left (+) 0 sizes)
let depth = fold (fun _ depths ->
  1 + List.fold_left max 0 depths)

let to_string = fold (fun x strs ->
  match strs with
  | [] -> x
  | _ -> x ^ "(" ^ String.concat "," strs ^ ")")

let () =
  let tree = Rose ("a", [
    Rose ("b", [Rose ("d", []); Rose ("e", [])]);
    Rose ("c", [Rose ("f", [])])
  ]) in
  assert (size tree = 6);
  assert (depth tree = 3);
  assert (to_string tree = "a(b(d,e),c(f))");
  print_endline "ok"
