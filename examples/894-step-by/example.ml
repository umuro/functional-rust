(* Example 100: Step By, Enumerate, Rev *)
(* Iterator modifiers *)

(* Approach 1: Step by — take every nth element *)
let step_by n lst =
  List.filteri (fun i _ -> i mod n = 0) lst

let range_step start stop step =
  let rec aux acc i =
    if i >= stop then List.rev acc
    else aux (i :: acc) (i + step)
  in
  aux [] start

(* Approach 2: Enumerate — pair with index *)
let enumerate lst = List.mapi (fun i x -> (i, x)) lst

let find_with_index pred lst =
  let rec aux i = function
    | [] -> None
    | x :: _ when pred x -> Some (i, x)
    | _ :: rest -> aux (i + 1) rest
  in
  aux 0 lst

let indexed_filter pred lst =
  enumerate lst
  |> List.filter (fun (_, x) -> pred x)

(* Approach 3: Rev — reverse iteration *)
let rev_map f lst = List.rev_map f lst |> List.rev
(* Note: List.rev_map reverses order, so we reverse back *)

let last_n n lst =
  let len = List.length lst in
  List.filteri (fun i _ -> i >= len - n) lst

let pairs_reversed lst =
  let rev = List.rev lst in
  List.combine lst rev

(* Practical combinations *)
let format_numbered lst =
  enumerate lst
  |> List.map (fun (i, x) -> Printf.sprintf "%d. %s" (i + 1) x)

let every_other lst = step_by 2 lst
let every_third lst = step_by 3 lst

let reverse_words sentence =
  String.split_on_char ' ' sentence
  |> List.rev
  |> String.concat " "

(* Tests *)
let () =
  assert (step_by 2 [0;1;2;3;4;5;6;7;8;9] = [0;2;4;6;8]);
  assert (step_by 3 [0;1;2;3;4;5;6;7;8;9] = [0;3;6;9]);

  assert (range_step 0 10 2 = [0;2;4;6;8]);
  assert (range_step 1 10 3 = [1;4;7]);

  assert (enumerate ["a";"b";"c"] = [(0,"a"); (1,"b"); (2,"c")]);

  assert (find_with_index (fun x -> x > 3) [1;2;3;4;5] = Some (3, 4));
  assert (find_with_index (fun x -> x > 10) [1;2;3] = None);

  let filtered = indexed_filter (fun x -> x mod 2 = 0) [10;11;12;13;14] in
  assert (filtered = [(0, 10); (2, 12); (4, 14)]);

  assert (last_n 3 [1;2;3;4;5] = [3;4;5]);

  assert (format_numbered ["apple"; "banana"; "cherry"] =
          ["1. apple"; "2. banana"; "3. cherry"]);

  assert (every_other [1;2;3;4;5;6] = [1;3;5]);

  assert (reverse_words "hello world foo" = "foo world hello");

  Printf.printf "✓ All tests passed\n"
