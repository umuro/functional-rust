(* 742: Type Witnesses — OCaml GADT approach *)

(* A 'sorted list' witness — can only be created by our sort function *)
module SortedList : sig
  type 'a t
  val sort       : 'a list -> 'a t    (* only entry point *)
  val to_list    : 'a t -> 'a list
  val merge      : 'a t -> 'a t -> 'a t  (* merging sorted lists = sorted *)
  val binary_search : int t -> int -> bool
end = struct
  type 'a t = Sorted of 'a list    (* private constructor *)

  let sort lst = Sorted (List.sort compare lst)
  let to_list (Sorted lst) = lst

  let merge (Sorted a) (Sorted b) =
    let rec merge_sorted a b = match a, b with
      | [], r | r, [] -> r
      | x :: xs, y :: ys ->
          if x <= y then x :: merge_sorted xs (y :: ys)
          else y :: merge_sorted (x :: xs) ys
    in
    Sorted (merge_sorted a b)

  let binary_search (Sorted lst) target =
    let arr = Array.of_list lst in
    let n = Array.length arr in
    let lo = ref 0 and hi = ref (n - 1) in
    let found = ref false in
    while !lo <= !hi && not !found do
      let mid = (!lo + !hi) / 2 in
      if arr.(mid) = target then found := true
      else if arr.(mid) < target then lo := mid + 1
      else hi := mid - 1
    done;
    !found
end

let () =
  let s = SortedList.sort [5; 1; 3; 2; 4] in
  Printf.printf "Sorted: [%s]\n"
    (SortedList.to_list s |> List.map string_of_int |> String.concat ";");
  Printf.printf "Binary search 3: %b\n" (SortedList.binary_search s 3);
  Printf.printf "Binary search 6: %b\n" (SortedList.binary_search s 6);
  let s2 = SortedList.sort [10; 8; 6] in
  let merged = SortedList.merge s s2 in
  Printf.printf "Merged: [%s]\n"
    (SortedList.to_list merged |> List.map string_of_int |> String.concat ";")
