(* 1061: Word Break — DP + HashSet *)

module StringSet = Set.Make(String)

(* Approach 1: Bottom-up DP *)
let word_break s words =
  let dict = List.fold_left (fun acc w -> StringSet.add w acc) StringSet.empty words in
  let n = String.length s in
  let dp = Array.make (n + 1) false in
  dp.(0) <- true;
  for i = 1 to n do
    for j = 0 to i - 1 do
      if dp.(j) && StringSet.mem (String.sub s j (i - j)) dict then
        dp.(i) <- true
    done
  done;
  dp.(n)

(* Approach 2: Recursive with memoization *)
let word_break_memo s words =
  let dict = List.fold_left (fun acc w -> StringSet.add w acc) StringSet.empty words in
  let n = String.length s in
  let cache = Hashtbl.create 32 in
  let rec solve start =
    if start = n then true
    else
      match Hashtbl.find_opt cache start with
      | Some v -> v
      | None ->
        let v = ref false in
        for end_ = start + 1 to n do
          if not !v && StringSet.mem (String.sub s start (end_ - start)) dict then
            v := solve end_
        done;
        Hashtbl.add cache start !v;
        !v
  in
  solve 0

(* Approach 3: BFS approach *)
let word_break_bfs s words =
  let dict = List.fold_left (fun acc w -> StringSet.add w acc) StringSet.empty words in
  let n = String.length s in
  let visited = Array.make (n + 1) false in
  let queue = Queue.create () in
  Queue.push 0 queue;
  visited.(0) <- true;
  while not (Queue.is_empty queue) do
    let start = Queue.pop queue in
    for end_ = start + 1 to n do
      if not visited.(end_) && StringSet.mem (String.sub s start (end_ - start)) dict then begin
        if end_ = n then (Queue.clear queue; visited.(n) <- true)
        else begin
          visited.(end_) <- true;
          Queue.push end_ queue
        end
      end
    done
  done;
  visited.(n)

let () =
  assert (word_break "leetcode" ["leet"; "code"] = true);
  assert (word_break "applepenapple" ["apple"; "pen"] = true);
  assert (word_break "catsandog" ["cats"; "dog"; "sand"; "and"; "cat"] = false);

  assert (word_break_memo "leetcode" ["leet"; "code"] = true);
  assert (word_break_memo "catsandog" ["cats"; "dog"; "sand"; "and"; "cat"] = false);

  assert (word_break_bfs "leetcode" ["leet"; "code"] = true);
  assert (word_break_bfs "catsandog" ["cats"; "dog"; "sand"; "and"; "cat"] = false);

  Printf.printf "✓ All tests passed\n"
