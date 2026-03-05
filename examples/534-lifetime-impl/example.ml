(* Methods on types with references in OCaml *)
type 'a view = {
  data: 'a array;
  start: int;
  length: int;
}

let make_view data start len =
  { data; start; length = min len (Array.length data - start) }

let get view i =
  if i < view.length then Some view.data.(view.start + i)
  else None

let slice view start len =
  make_view view.data (view.start + start) len

let iter view f =
  for i = 0 to view.length - 1 do
    f view.data.(view.start + i)
  done

let () =
  let data = [| 1; 2; 3; 4; 5; 6; 7; 8; 9; 10 |] in
  let view = make_view data 2 5 in
  Printf.printf "view[0] = %s\n" (match get view 0 with Some v -> string_of_int v | None -> "None");
  Printf.printf "view[4] = %s\n" (match get view 4 with Some v -> string_of_int v | None -> "None");
  let sub = slice view 1 3 in
  iter sub (fun x -> Printf.printf "%d " x);
  print_newline ()
