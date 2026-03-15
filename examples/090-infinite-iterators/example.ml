(* 090: Infinite Iterators *)

(* Approach 1: cycle *)
let cycle lst =
  let rec aux () =
    List.to_seq lst |> Seq.flat_map (fun x ->
      fun () -> Seq.Cons (x, fun () -> Seq.Nil))
  in
  let rec make original current () =
    match current () with
    | Seq.Nil -> make original (List.to_seq original) ()
    | Seq.Cons (x, rest) -> Seq.Cons (x, make original rest)
  in
  make lst (List.to_seq lst)

(* Approach 2: repeat *)
let repeat x =
  let rec aux () = Seq.Cons (x, aux) in
  aux

(* Approach 3: from_fn *)
let counter_from n =
  let c = ref n in
  fun () -> let v = !c in c := !c + 1; Seq.Cons (v, fun () -> Seq.Nil)

let take n s = List.of_seq (Seq.take n s)

(* Tests *)
let () =
  assert (take 7 (cycle [1; 2; 3]) = [1; 2; 3; 1; 2; 3; 1]);
  assert (take 4 (repeat 42) = [42; 42; 42; 42]);
  Printf.printf "✓ All tests passed\n"
