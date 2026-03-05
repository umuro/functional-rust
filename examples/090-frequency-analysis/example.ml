module CMap = Map.Make(Char)

(* Idiomatic OCaml — fold over string, update map with lowercase letters *)
let frequency s =
  String.fold_left (fun m c ->
    let c = Char.lowercase_ascii c in
    if c >= 'a' && c <= 'z' then
      CMap.update c (function None -> Some 1 | Some n -> Some (n+1)) m
    else m
  ) CMap.empty s

(* Sorted by frequency descending — bindings gives (key, value) list *)
let sorted_freq s =
  frequency s |> CMap.bindings
  |> List.sort (fun (_, a) (_, b) -> compare b a)

(* Recursive variant — explicit pattern matching on char list *)
let frequency_rec s =
  let rec go m = function
    | [] -> m
    | c :: rest ->
      let c = Char.lowercase_ascii c in
      let m' =
        if c >= 'a' && c <= 'z' then
          CMap.update c (function None -> Some 1 | Some n -> Some (n+1)) m
        else m
      in
      go m' rest
  in
  go CMap.empty (List.of_seq (String.to_seq s))

let () =
  (* Basic frequency tests *)
  let f = frequency "aabb" in
  assert (CMap.find 'a' f = 2);
  assert (CMap.find 'b' f = 2);

  (* Case insensitive *)
  let f2 = frequency "AaAa" in
  assert (CMap.find 'a' f2 = 4);

  (* Non-alpha ignored *)
  let f3 = frequency "a1b!" in
  assert (CMap.find 'a' f3 = 1);
  assert (CMap.find 'b' f3 = 1);
  assert (not (CMap.mem '1' f3));

  (* Sorted descending *)
  let s = sorted_freq "aaabbc" in
  assert (s = [('a', 3); ('b', 2); ('c', 1)]);

  (* Recursive variant matches fold variant *)
  let text = "Hello World" in
  assert (frequency text = frequency_rec text);

  (* Pangram has all 26 letters *)
  let pangram = "The quick brown fox jumps over the lazy dog" in
  assert (CMap.cardinal (frequency pangram) = 26);

  print_endline "ok"
