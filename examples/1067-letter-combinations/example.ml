(* 1067: Phone Keypad Letter Combinations *)

let phone_map = [|
  "";     (* 0 *)
  "";     (* 1 *)
  "abc";  (* 2 *)
  "def";  (* 3 *)
  "ghi";  (* 4 *)
  "jkl";  (* 5 *)
  "mno";  (* 6 *)
  "pqrs"; (* 7 *)
  "tuv";  (* 8 *)
  "wxyz"; (* 9 *)
|]

(* Approach 1: Backtracking *)
let letter_combos digits =
  if String.length digits = 0 then []
  else begin
    let results = ref [] in
    let buf = Buffer.create (String.length digits) in
    let rec backtrack idx =
      if idx = String.length digits then
        results := Buffer.contents buf :: !results
      else begin
        let letters = phone_map.(Char.code digits.[idx] - Char.code '0') in
        String.iter (fun c ->
          Buffer.add_char buf c;
          backtrack (idx + 1);
          let len = Buffer.length buf in
          Buffer.truncate buf (len - 1)
        ) letters
      end
    in
    backtrack 0;
    List.rev !results
  end

(* Approach 2: Functional with List.concat_map *)
let letter_combos_func digits =
  if String.length digits = 0 then []
  else begin
    let chars_of_string s = List.init (String.length s) (fun i -> s.[i]) in
    let digit_chars idx =
      chars_of_string phone_map.(Char.code digits.[idx] - Char.code '0')
    in
    let rec solve idx =
      if idx = String.length digits then [""]
      else
        let letters = digit_chars idx in
        let rest = solve (idx + 1) in
        List.concat_map (fun c ->
          List.map (fun suffix -> String.make 1 c ^ suffix) rest
        ) letters
    in
    solve 0
  end

(* Approach 3: Iterative with queue *)
let letter_combos_iter digits =
  if String.length digits = 0 then []
  else begin
    let queue = Queue.create () in
    Queue.push "" queue;
    for i = 0 to String.length digits - 1 do
      let letters = phone_map.(Char.code digits.[i] - Char.code '0') in
      let size = Queue.length queue in
      for _ = 1 to size do
        let current = Queue.pop queue in
        String.iter (fun c ->
          Queue.push (current ^ String.make 1 c) queue
        ) letters
      done
    done;
    Queue.fold (fun acc x -> x :: acc) [] queue |> List.rev
  end

let () =
  let r1 = letter_combos "23" in
  assert (List.length r1 = 9);
  assert (List.mem "ad" r1);
  assert (List.mem "cf" r1);

  let r2 = letter_combos_func "23" in
  assert (List.length r2 = 9);

  let r3 = letter_combos_iter "23" in
  assert (List.length r3 = 9);

  assert (letter_combos "" = []);
  assert (List.length (letter_combos "7") = 4);

  Printf.printf "✓ All tests passed\n"
