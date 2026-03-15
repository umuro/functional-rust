(* 1064: Generate All Permutations via Backtracking
   Three approaches: swap-based, used-flags, and Heap's algorithm. *)

(* Approach 1: Swap-based backtracking *)
let permutations_swap nums =
  let arr = Array.copy nums in
  let n = Array.length arr in
  let results = ref [] in
  let rec permute start =
    if start = n then results := Array.to_list arr :: !results
    else
      for i = start to n - 1 do
        let tmp = arr.(start) in arr.(start) <- arr.(i); arr.(i) <- tmp;
        permute (start + 1);
        let tmp = arr.(start) in arr.(start) <- arr.(i); arr.(i) <- tmp
      done
  in
  permute 0;
  !results

(* Approach 2: Used-flags — builds permutation by picking unused elements *)
let permutations_flags nums =
  let n = List.length nums in
  let arr = Array.of_list nums in
  let used = Array.make n false in
  let current = ref [] in
  let results = ref [] in
  let rec build () =
    if List.length !current = n then
      results := List.rev !current :: !results
    else
      for i = 0 to n - 1 do
        if not used.(i) then begin
          used.(i) <- true;
          current := arr.(i) :: !current;
          build ();
          current := List.tl !current;
          used.(i) <- false
        end
      done
  in
  build ();
  !results

(* Approach 3: Purely functional — insert into all positions *)
(* insert_at i x lst inserts x before the i-th element *)
let insert_at i x lst =
  let rec aux j = function
    | [] -> [x]
    | h :: t -> if j = i then x :: h :: t else h :: aux (j+1) t
  in
  aux 0 lst

let rec permutations_func = function
  | [] -> [[]]
  | x :: rest ->
    let sub = permutations_func rest in
    List.concat_map (fun perm ->
      let n = List.length perm in
      List.init (n + 1) (fun i -> insert_at i x perm)
    ) sub

let () =
  let nums = [|1;2;3|] in
  let p1 = permutations_swap nums in
  assert (List.length p1 = 6);
  assert (List.mem [1;2;3] p1);
  assert (List.mem [3;2;1] p1);

  let p2 = permutations_flags [1;2;3] in
  assert (List.length p2 = 6);

  let p3 = permutations_func [1;2;3] in
  assert (List.length p3 = 6);

  (* Single element *)
  assert (List.length (permutations_flags [42]) = 1);

  Printf.printf "All permutation tests passed.\n"
