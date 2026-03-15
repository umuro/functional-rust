(* 918: Random access with nth — List.nth and Array indexing

   OCaml: List.nth raises Not_found / Invalid_argument for out-of-bounds.
   We wrap it in a safe Option-returning version. *)

(* Safe nth: returns None instead of raising *)
let nth_opt lst n =
  if n < 0 then None
  else
    let rec go i = function
      | [] -> None
      | x :: _ when i = 0 -> Some x
      | _ :: rest -> go (i - 1) rest
    in
    go n lst

(* nth on arrays is O(1) and trivial *)
let array_nth arr n =
  if n < 0 || n >= Array.length arr then None
  else Some arr.(n)

(* drop n elements from a list — residual after nth *)
let rec drop n lst =
  if n <= 0 then lst
  else match lst with
    | [] -> []
    | _ :: rest -> drop (n - 1) rest

(* advance: like Rust's nth that mutates iterator state — return (element, rest) *)
let advance n lst =
  (* consume n elements, return (nth_element, remaining) *)
  let rest = drop n lst in
  match rest with
  | [] -> (None, [])
  | x :: remaining -> (Some x, remaining)

let () =
  let v = [10; 20; 30; 40] in

  (* basic nth *)
  assert (nth_opt v 0 = Some 10);
  assert (nth_opt v 2 = Some 30);
  assert (nth_opt v 3 = Some 40);

  (* out of bounds *)
  assert (nth_opt v 5 = None);
  assert (nth_opt [] 0 = None);

  (* nth(0) = head *)
  assert (nth_opt [99] 0 = Some 99);

  (* advance simulates mutable iterator state — nth(1) then nth(0) *)
  let lst = [1; 2; 3; 4; 5] in
  let (first, rest) = advance 1 lst in  (* skip 1, take element at index 1 *)
  assert (first = Some 2);
  let (second, _) = advance 0 rest in   (* now at position 2 *)
  assert (second = Some 3);

  (* array nth — O(1) *)
  let arr = [|10; 20; 30; 40|] in
  assert (array_nth arr 2 = Some 30);
  assert (array_nth arr 5 = None);

  (* Standard library List.nth raises, but we use nth_opt *)
  (try ignore (List.nth [] 0)
   with Invalid_argument _ -> ());  (* expected *)

  print_endline "918-iterator-nth: all tests passed"
