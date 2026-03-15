(* 1061: Word Break — DP + Set
   Can a string be segmented into dictionary words? *)

module StringSet = Set.Make(String)

(* Approach 1: Bottom-up DP *)
let word_break s words =
  let dict = List.fold_left (fun acc w -> StringSet.add w acc) StringSet.empty words in
  let n = String.length s in
  let dp = Array.make (n + 1) false in
  dp.(0) <- true;
  for i = 1 to n do
    let j = ref 0 in
    while !j < i && not dp.(i) do
      if dp.(!j) && StringSet.mem (String.sub s !j (i - !j)) dict then
        dp.(i) <- true;
      incr j
    done
  done;
  dp.(n)

(* Approach 2: Recursive with memoization *)
let word_break_memo s words =
  let dict = List.fold_left (fun acc w -> StringSet.add w acc) StringSet.empty words in
  let n = String.length s in
  let cache = Array.make (n + 1) None in
  let rec solve start =
    if start = n then true
    else match cache.(start) with
    | Some v -> v
    | None ->
      let result = ref false in
      let i = ref (start + 1) in
      while !i <= n && not !result do
        if StringSet.mem (String.sub s start (!i - start)) dict
           && solve !i then
          result := true;
        incr i
      done;
      cache.(start) <- Some !result; !result
  in
  solve 0

(* Approach 3: BFS *)
let word_break_bfs s words =
  let dict = List.fold_left (fun acc w -> StringSet.add w acc) StringSet.empty words in
  let n = String.length s in
  let visited = Array.make (n + 1) false in
  let q = Queue.create () in
  Queue.add 0 q;
  visited.(0) <- true;
  let found = ref false in
  while not (Queue.is_empty q) && not !found do
    let start = Queue.pop q in
    for end_ = start + 1 to n do
      if not visited.(end_)
         && StringSet.mem (String.sub s start (end_ - start)) dict then begin
        if end_ = n then found := true
        else begin
          visited.(end_) <- true;
          Queue.add end_ q
        end
      end
    done
  done;
  !found

let () =
  assert (word_break      "leetcode"    ["leet"; "code"]                              = true);
  assert (word_break      "applepenapple" ["apple"; "pen"]                            = true);
  assert (word_break      "catsandog"   ["cats";"dog";"sand";"and";"cat"]            = false);
  assert (word_break_memo "leetcode"    ["leet"; "code"]                              = true);
  assert (word_break_memo "catsandog"   ["cats";"dog";"sand";"and";"cat"]            = false);
  assert (word_break_bfs  "leetcode"    ["leet"; "code"]                              = true);
  assert (word_break_bfs  "catsandog"   ["cats";"dog";"sand";"and";"cat"]            = false);
  Printf.printf "All word-break tests passed.\n"
