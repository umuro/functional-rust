(* 945: Word Count with Map

   Building a word-frequency map from text. Demonstrates string
   normalisation, splitting, and folding into a map. *)

module StringMap = Map.Make(String)

(* ── Tokenise: lowercase and extract alphanumeric words ─────────────────── *)

let tokenize s =
  let s = String.lowercase_ascii s in
  (* Split on any non-alphanumeric character *)
  let words = ref [] and buf = Buffer.create 16 in
  String.iter (fun c ->
    if (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') then
      Buffer.add_char buf c
    else begin
      let w = Buffer.contents buf in
      if w <> "" then words := w :: !words;
      Buffer.clear buf
    end
  ) s;
  let w = Buffer.contents buf in
  if w <> "" then words := w :: !words;
  List.rev !words

(* ── Word count using StringMap (sorted, like OCaml's natural Map) ─────── *)

let word_count sentence =
  List.fold_left (fun m word ->
    let count = match StringMap.find_opt word m with
      | None   -> 0
      | Some n -> n
    in
    StringMap.add word (count + 1) m
  ) StringMap.empty (tokenize sentence)

(* ── Word count using Hashtbl (O(1) average, unordered) ─────────────────── *)

let word_count_hash sentence =
  let tbl = Hashtbl.create 16 in
  List.iter (fun word ->
    let count = try Hashtbl.find tbl word with Not_found -> 0 in
    Hashtbl.replace tbl word (count + 1)
  ) (tokenize sentence);
  tbl

(* ── Top N most frequent words ────────────────────────────────────────────── *)

let top_n n m =
  let pairs = StringMap.bindings m in
  let sorted = List.sort (fun (_, a) (_, b) -> compare b a) pairs in
  let rec take k = function
    | [] -> []
    | _ when k = 0 -> []
    | x :: rest -> x :: take (k - 1) rest
  in
  take n sorted

let () =
  let m = word_count "the cat sat on the mat" in
  assert (StringMap.find "the" m = 2);
  assert (StringMap.find "cat" m = 1);

  let m2 = word_count "the cat sat on the mat, the cat sat" in
  assert (StringMap.find "the" m2 = 3);
  assert (StringMap.find "cat" m2 = 2);
  assert (StringMap.find "sat" m2 = 2);

  (* case insensitive *)
  let m3 = word_count "The THE the" in
  assert (StringMap.find "the" m3 = 3);

  (* empty *)
  assert (StringMap.is_empty (word_count ""));

  (* single word *)
  let m4 = word_count "hello" in
  assert (StringMap.find "hello" m4 = 1);
  assert (StringMap.cardinal m4 = 1);

  (* sorted keys — Map.Make gives lexicographic order *)
  let m5 = word_count "banana apple cherry apple" in
  let keys = List.map fst (StringMap.bindings m5) in
  assert (keys = ["apple"; "banana"; "cherry"]);
  assert (StringMap.find "apple" m5 = 2);

  (* hashtbl version *)
  let h = word_count_hash "the cat sat on the mat" in
  assert (Hashtbl.find h "the" = 2);
  assert (Hashtbl.find h "cat" = 1);

  (* top N *)
  let m6 = word_count "a a a b b c d" in
  let top = top_n 2 m6 in
  assert (List.length top = 2);
  assert (fst (List.hd top) = "a");
  assert (snd (List.hd top) = 3);

  print_endline "945-word-count: all tests passed"
