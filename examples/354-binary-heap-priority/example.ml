(* OCaml: priority queue via sorted list (simple demo) *)

module PQ = struct
  type 'a t = { mutable heap: 'a list }
  let empty () = { heap=[] }
  let push pq x = pq.heap <- List.sort compare (x :: pq.heap)
  let pop pq = match List.rev pq.heap with
    | [] -> None
    | x::xs -> pq.heap <- List.rev xs; Some x
end

let () =
  let pq = PQ.empty () in
  List.iter (PQ.push pq) [3;1;4;1;5;9;2;6];
  let rec drain () = match PQ.pop pq with
    | None -> () | Some x -> Printf.printf "%d " x; drain ()
  in drain (); print_newline ()
