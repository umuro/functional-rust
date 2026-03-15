(* 095: Double-Ended Iterator
   OCaml: arrays support both-ends traversal; lists support rev *)

(* --- Approach 1: Palindrome check using both-ends comparison --- *)

(* Compare list with its reverse — O(n) but clear *)
let is_palindrome xs = xs = List.rev xs

(* Array-based: compare from both ends without allocation *)
let is_palindrome_arr xs =
  let arr = Array.of_list xs in
  let n = Array.length arr in
  let rec check i =
    if i >= n / 2 then true
    else if arr.(i) <> arr.(n - 1 - i) then false
    else check (i + 1)
  in
  check 0

(* --- Approach 2: Take from both ends of a list --- *)

let take_front n xs =
  let rec aux acc count = function
    | [] -> List.rev acc
    | _ when count = 0 -> List.rev acc
    | x :: rest -> aux (x :: acc) (count - 1) rest
  in
  aux [] n xs

let take_back n xs =
  (* take last n elements *)
  let rev = List.rev xs in
  List.rev (take_front n rev)

(* --- Approach 3: Array bidirectional iteration --- *)

let iter_from_both xs =
  let arr = Array.of_list xs in
  let n = Array.length arr in
  let front = Array.to_list (Array.sub arr 0 (min 2 n)) in
  let back  = Array.to_list (Array.sub arr (max 0 (n - 2)) (min 2 n)) in
  (front, List.rev back)   (* back in reversed order, like next_back *)

let () =
  Printf.printf "is_palindrome [1;2;3;2;1] = %b\n" (is_palindrome [1;2;3;2;1]);
  Printf.printf "is_palindrome [1;2;3]     = %b\n" (is_palindrome [1;2;3]);
  Printf.printf "is_palindrome_arr [1;2;1] = %b\n" (is_palindrome_arr [1;2;1]);

  let (f, b) = iter_from_both [1;2;3;4;5] in
  Printf.printf "front 2 = [%s]\n" (String.concat "; " (List.map string_of_int f));
  Printf.printf "back  2 = [%s]\n" (String.concat "; " (List.map string_of_int b));

  (* bidirectional traversal with refs *)
  let v = [|1;2;3|] in
  let i = ref 0 and j = ref (Array.length v - 1) in
  Printf.printf "next_back = %d\n" (let x = v.(!j) in decr j; x);
  Printf.printf "next      = %d\n" (let x = v.(!i) in incr i; x);
  Printf.printf "next      = %d\n" (let x = v.(!i) in incr i; x);
  Printf.printf "next (exhausted) = %s\n"
    (if !i > !j then "None" else string_of_int v.(!i))
