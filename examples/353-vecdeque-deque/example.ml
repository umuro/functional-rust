(* OCaml: deque via doubly-linked list simulation *)

type 'a deque = { mutable data: 'a list; mutable back: 'a list }

let empty () = { data=[]; back=[] }
let push_back d x = d.back <- x :: d.back
let push_front d x = d.data <- x :: d.data
let pop_front d =
  match d.data with
  | x::xs -> d.data <- xs; Some x
  | [] -> match List.rev d.back with
    | [] -> None
    | x::xs -> d.data <- xs; d.back <- []; Some x

let () =
  let d = empty () in
  push_back d 1; push_back d 2; push_back d 3;
  push_front d 0;
  let rec drain () = match pop_front d with
    | None -> ()
    | Some x -> Printf.printf "%d " x; drain ()
  in drain (); print_newline ()
