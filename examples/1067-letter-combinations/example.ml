(* 1067: Phone Keypad Letter Combinations
   Backtracking, iterative queue, and functional fold. *)

let phone_map = [|""; ""; "abc"; "def"; "ghi"; "jkl"; "mno"; "pqrs"; "tuv"; "wxyz"|]

let digit_letters d = phone_map.(Char.code d - Char.code '0')

(* Approach 1: Backtracking *)
let letter_combos digits =
  if String.length digits = 0 then []
  else begin
    let results = ref [] in
    let buf = Buffer.create 8 in
    let len = String.length digits in
    let rec backtrack idx =
      if idx = len then results := Buffer.contents buf :: !results
      else
        String.iter (fun c ->
          Buffer.add_char buf c;
          backtrack (idx + 1);
          Buffer.truncate buf (Buffer.length buf - 1)
        ) (digit_letters digits.[idx])
    in
    backtrack 0;
    !results
  end

(* Helper: fold over characters of a string (not available in OCaml 4.10) *)
let string_fold_left f init s =
  let acc = ref init in
  String.iter (fun c -> acc := f !acc c) s;
  !acc

(* Approach 2: Iterative with queue (level-by-level expansion) *)
let letter_combos_iter digits =
  if String.length digits = 0 then []
  else
    string_fold_left (fun acc d ->
      let letters = digit_letters d in
      List.concat_map (fun prefix ->
        let result = ref [] in
        String.iter (fun c -> result := (prefix ^ String.make 1 c) :: !result) letters;
        List.rev !result
      ) acc
    ) [""] digits

(* Approach 3: Functional fold — most idiomatic *)
let letter_combos_fold digits =
  if String.length digits = 0 then []
  else
    string_fold_left
      (fun acc d ->
        let letters = digit_letters d in
        List.concat_map (fun prefix ->
          List.map (fun c -> prefix ^ String.make 1 c)
            (List.init (String.length letters) (String.get letters))
        ) acc)
      [""]
      digits

let () =
  let r1 = letter_combos "23" in
  assert (List.length r1 = 9);
  assert (List.mem "ad" r1);
  assert (List.mem "cf" r1);

  let r2 = letter_combos_iter "23" in
  assert (List.length r2 = 9);

  let r3 = letter_combos_fold "23" in
  assert (List.length r3 = 9);

  assert (letter_combos "" = []);

  (* "7" = pqrs, 4 letters *)
  assert (List.length (letter_combos "7") = 4);

  Printf.printf "All letter-combination tests passed.\n"
