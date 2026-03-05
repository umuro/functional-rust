(* Word Break — dictionary DP O(n²) *)
module SSet = Set.Make(String)

let word_break s dict =
  let dict_set = List.fold_left (fun acc w -> SSet.add w acc) SSet.empty dict in
  let n  = String.length s in
  let dp = Array.make (n + 1) false in
  dp.(0) <- true;
  for i = 1 to n do
    let j = ref 0 in
    while !j < i && not dp.(i) do
      if dp.(!j) && SSet.mem (String.sub s !j (i - !j)) dict_set then
        dp.(i) <- true;
      incr j
    done
  done;
  dp.(n)

let word_break_all s dict =
  let dict_set = List.fold_left (fun acc w -> SSet.add w acc) SSet.empty dict in
  let n  = String.length s in
  (* prev.(i) = list of j such that dp[j]=true and s[j..i] in dict *)
  let dp   = Array.make (n + 1) false in
  let prev = Array.make (n + 1) [] in
  dp.(0) <- true;
  for i = 1 to n do
    for j = 0 to i - 1 do
      if dp.(j) && SSet.mem (String.sub s j (i - j)) dict_set then begin
        dp.(i) <- true;
        prev.(i) <- j :: prev.(i)
      end
    done
  done;
  (* Collect all sentences *)
  let rec collect i =
    if i = 0 then [[]]
    else
      List.concat_map (fun j ->
        let word = String.sub s j (i - j) in
        List.map (fun rest -> rest @ [word]) (collect j)
      ) prev.(i)
  in
  if dp.(n) then collect n else []

let () =
  let dict = ["leet"; "code"; "leets"; "code"; "cats"; "and"; "sand"; "dog"; "cat"] in
  Printf.printf "'leetcode'    -> %b\n" (word_break "leetcode" dict);
  Printf.printf "'catsanddog' -> %b\n" (word_break "catsanddog" dict);
  Printf.printf "'catsanddogx'-> %b\n" (word_break "catsanddogx" dict);
  let sentences = word_break_all "catsanddog" ["cat";"cats";"and";"sand";"dog"] in
  Printf.printf "All parses of 'catsanddog':\n";
  List.iter (fun ws -> Printf.printf "  %s\n" (String.concat " " ws)) sentences
