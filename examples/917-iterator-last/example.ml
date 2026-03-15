(* 917: Getting the last element of a list

   OCaml lists are singly-linked; getting the last element is O(n).
   We fold the whole list keeping only the final value. *)

(* last: returns Some of the last element, or None for empty list *)
let last lst =
  match lst with
  | [] -> None
  | _ -> Some (List.fold_left (fun _ x -> x) (List.hd lst) lst)

(* last_opt: idiomatic name — mirrors Option convention *)
let last_opt = last

(* For arrays, last is O(1) *)
let array_last arr =
  let n = Array.length arr in
  if n = 0 then None else Some arr.(n - 1)

(* last_after_filter: find last element satisfying a predicate *)
let last_where pred lst =
  List.fold_left
    (fun acc x -> if pred x then Some x else acc)
    None lst

let () =
  (* basic *)
  assert (last [1; 2; 3; 4; 5] = Some 5);

  (* empty *)
  assert (last [] = None);

  (* single *)
  assert (last [42] = Some 42);

  (* after filter — last even in 1..10 *)
  let last_even =
    let evens = List.filter (fun x -> x mod 2 = 0) (List.init 10 (fun i -> i + 1)) in
    last evens
  in
  assert (last_even = Some 10);

  (* using last_where directly *)
  assert (last_where (fun x -> x mod 2 = 0) [1;2;3;4;5;6;7] = Some 6);

  (* array last *)
  assert (array_last [|1;2;3|] = Some 3);
  assert (array_last [||] = None);

  (* strings: last char *)
  let last_char s =
    if s = "" then None
    else Some s.[String.length s - 1]
  in
  assert (last_char "hello" = Some 'o');
  assert (last_char "" = None);

  print_endline "917-iterator-last: all tests passed"
