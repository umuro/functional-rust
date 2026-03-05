(* Imperative — Stack with Array *)
(* Array-backed stack with dynamic resizing *)

type 'a stack = {
  mutable data : 'a option array;
  mutable top : int;
}

let create () = { data = Array.make 8 None; top = 0 }

let push s x =
  if s.top = Array.length s.data then begin
    let new_data = Array.make (s.top * 2) None in
    Array.blit s.data 0 new_data 0 s.top;
    s.data <- new_data
  end;
  s.data.(s.top) <- Some x;
  s.top <- s.top + 1

let pop s =
  if s.top = 0 then None
  else begin
    s.top <- s.top - 1;
    let v = s.data.(s.top) in
    s.data.(s.top) <- None;
    v
  end

let s = create ()
let () = List.iter (push s) [1;2;3;4;5]
let () =
  let rec drain () = match pop s with
    | Some x -> Printf.printf "%d " x; drain ()
    | None -> print_newline ()
  in drain ()
