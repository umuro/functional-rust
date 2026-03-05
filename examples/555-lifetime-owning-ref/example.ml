(* Owning references pattern in OCaml *)
(* OCaml data structures always own their contents via GC *)

type 'a owned_view = {
  source: 'a;
  start: int;
  length: int;
}

let make_view source start len = { source; start; length = len }

let get_element view i =
  if i < view.length then
    Some (Array.get view.source (view.start + i))
  else None

let () =
  let data = [|10; 20; 30; 40; 50|] in
  let view = make_view data 1 3 in
  for i = 0 to view.length - 1 do
    match get_element view i with
    | Some v -> Printf.printf "view[%d] = %d\n" i v
    | None -> ()
  done
