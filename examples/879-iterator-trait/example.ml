(* Example 085: Iterator Trait *)
(* OCaml Seq → Rust Iterator *)

(* Approach 1: Using Seq (lazy sequences) *)
let range start stop =
  let rec aux n () =
    if n >= stop then Seq.Nil
    else Seq.Cons (n, aux (n + 1))
  in
  aux start

let seq_to_list seq =
  Seq.fold_left (fun acc x -> x :: acc) [] seq |> List.rev

(* Approach 2: Custom iterator via closure *)
type 'a iterator = {
  next : unit -> 'a option;
}

let range_iter start stop =
  let current = ref start in
  { next = fun () ->
    if !current >= stop then None
    else begin
      let v = !current in
      incr current;
      Some v
    end
  }

let iter_to_list it =
  let rec aux acc =
    match it.next () with
    | None -> List.rev acc
    | Some v -> aux (v :: acc)
  in
  aux []

let iter_map f it =
  { next = fun () ->
    match it.next () with
    | None -> None
    | Some v -> Some (f v)
  }

let iter_filter pred it =
  let rec find () =
    match it.next () with
    | None -> None
    | Some v -> if pred v then Some v else find ()
  in
  { next = find }

(* Approach 3: Using List as iterator source *)
let counter_from n =
  let c = ref n in
  { next = fun () ->
    let v = !c in
    c := v + 1;
    Some v
  }

let take n it =
  let count = ref 0 in
  { next = fun () ->
    if !count >= n then None
    else begin
      incr count;
      it.next ()
    end
  }

(* Tests *)
let () =
  let s = range 1 6 in
  assert (seq_to_list s = [1; 2; 3; 4; 5]);

  let it = range_iter 1 6 in
  assert (iter_to_list it = [1; 2; 3; 4; 5]);

  let it2 = range_iter 1 11 in
  let doubled = iter_map (fun x -> x * 2) it2 in
  let evens = iter_filter (fun x -> x <= 10) doubled in
  assert (iter_to_list evens = [2; 4; 6; 8; 10]);

  let inf = counter_from 0 in
  let first5 = take 5 inf in
  assert (iter_to_list first5 = [0; 1; 2; 3; 4]);

  Printf.printf "✓ All tests passed\n"
