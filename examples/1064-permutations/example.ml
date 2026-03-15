(* 1064: Generate All Permutations via Backtracking *)

(* Approach 1: Swap-based backtracking *)
let permutations_swap arr =
  let n = Array.length arr in
  let results = ref [] in
  let rec permute start =
    if start = n then
      results := Array.to_list arr :: !results
    else
      for i = start to n - 1 do
        let tmp = arr.(start) in
        arr.(start) <- arr.(i);
        arr.(i) <- tmp;
        permute (start + 1);
        let tmp = arr.(start) in
        arr.(start) <- arr.(i);
        arr.(i) <- tmp
      done
  in
  permute 0;
  List.rev !results

(* Approach 2: Functional with list insertion *)
let permutations_insert lst =
  let rec insert x = function
    | [] -> [[x]]
    | hd :: tl as l ->
      (x :: l) :: List.map (fun rest -> hd :: rest) (insert x tl)
  in
  let rec perms = function
    | [] -> [[]]
    | hd :: tl ->
      List.concat_map (insert hd) (perms tl)
  in
  perms lst

(* Approach 3: Using used-flags *)
let permutations_flags lst =
  let arr = Array.of_list lst in
  let n = Array.length arr in
  let used = Array.make n false in
  let results = ref [] in
  let current = Array.make n 0 in
  let rec build pos =
    if pos = n then
      results := Array.to_list current :: !results
    else
      for i = 0 to n - 1 do
        if not used.(i) then begin
          used.(i) <- true;
          current.(pos) <- arr.(i);
          build (pos + 1);
          used.(i) <- false
        end
      done
  in
  build 0;
  List.rev !results

let () =
  let perms1 = permutations_swap [|1; 2; 3|] in
  assert (List.length perms1 = 6);
  assert (List.mem [1; 2; 3] perms1);
  assert (List.mem [3; 2; 1] perms1);

  let perms2 = permutations_insert [1; 2; 3] in
  assert (List.length perms2 = 6);

  let perms3 = permutations_flags [1; 2; 3] in
  assert (List.length perms3 = 6);

  (* Single element *)
  assert (List.length (permutations_insert [42]) = 1);

  Printf.printf "✓ All tests passed\n"
