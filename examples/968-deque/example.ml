(* 968: Double-Ended Queue (Deque) *)
(* OCaml: pair of lists (front, back) for amortized O(1) operations *)
(* Classic functional deque using two stacks *)

type 'a deque = {
  front: 'a list;
  back: 'a list;
}

let empty = { front = []; back = [] }

let is_empty d = d.front = [] && d.back = []

let size d = List.length d.front + List.length d.back

(* Balance: if front is empty, reverse back into front *)
let balance d =
  match d.front with
  | [] -> { front = List.rev d.back; back = [] }
  | _ -> d

let push_front x d = balance { d with front = x :: d.front }

let push_back x d = balance { d with back = x :: d.back }

let pop_front d =
  let d = balance d in
  match d.front with
  | [] -> None
  | x :: rest -> Some (x, balance { d with front = rest })

let pop_back d =
  let d = balance { front = d.back; back = d.front } in
  match d.front with
  | [] -> None
  | x :: rest -> Some (x, { front = d.back; back = rest })

let peek_front d =
  let d = balance d in
  match d.front with
  | [] -> None
  | x :: _ -> Some x

let peek_back d =
  let d = balance { front = d.back; back = d.front } in
  match d.front with
  | [] -> None
  | x :: _ -> Some x

let () =
  let d = empty in
  assert (is_empty d);

  let d = push_back 1 d in
  let d = push_back 2 d in
  let d = push_back 3 d in
  let d = push_front 0 d in

  assert (size d = 4);
  assert (peek_front d = Some 0);
  assert (peek_back d = Some 3);

  let (v, d) = Option.get (pop_front d) in
  assert (v = 0);
  assert (peek_front d = Some 1);

  let (v, d) = Option.get (pop_back d) in
  assert (v = 3);
  assert (size d = 2);

  let d = push_front 10 d in
  let d = push_back 20 d in
  assert (size d = 4);

  let (v, _) = Option.get (pop_front d) in
  assert (v = 10);

  assert (pop_front empty = None);
  assert (pop_back empty = None);

  Printf.printf "✓ All tests passed\n"
