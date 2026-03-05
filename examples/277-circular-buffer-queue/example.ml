type 'a queue = { front: 'a list; back: 'a list }

let empty = { front = []; back = [] }
let is_empty q = q.front = [] && q.back = []

let enqueue x q = { q with back = x :: q.back }

let dequeue q = match q.front with
  | h :: t -> Some (h, { q with front = t })
  | [] -> match List.rev q.back with
    | [] -> None
    | h :: t -> Some (h, { front = t; back = [] })

let to_list q = q.front @ List.rev q.back

let () =
  let q = empty |> enqueue 1 |> enqueue 2 |> enqueue 3 in
  assert (to_list q = [1; 2; 3]);
  let rec drain q = match dequeue q with
    | None -> []
    | Some (x, q') -> x :: drain q'
  in
  assert (drain q = [1; 2; 3]);
  (* Test interleaved operations *)
  let q2 = empty |> enqueue 1 |> enqueue 2 in
  let (v, q2) = match dequeue q2 with Some p -> p | None -> assert false in
  assert (v = 1);
  let q2 = enqueue 3 q2 in
  assert (drain q2 = [2; 3]);
  print_endline "ok"
