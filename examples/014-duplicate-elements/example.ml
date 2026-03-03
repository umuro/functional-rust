(* Duplicate Elements — 99 Problems #14 *)
(* Duplicate every element: [a;b;c] → [a;a;b;b;c;c] *)

(* ── Recursive ───────────────────────────────────────────── *)

let rec duplicate = function
  | [] -> []
  | h :: t -> h :: h :: duplicate t

(* ── Tail-recursive with accumulator ─────────────────────── *)

let duplicate_tr lst =
  let rec aux acc = function
    | [] -> List.rev acc
    | h :: t -> aux (h :: h :: acc) t
  in aux [] lst

(* ── Using concat_map ────────────────────────────────────── *)

let duplicate_map lst =
  List.concat_map (fun x -> [x; x]) lst

(* ── Tests ────────────────────────────────────────────────── *)
let () =
  assert (duplicate [] = []);
  assert (duplicate [1] = [1; 1]);
  assert (duplicate [1; 2; 3] = [1; 1; 2; 2; 3; 3]);
  assert (duplicate_tr ['a'; 'b'; 'c'] = ['a'; 'a'; 'b'; 'b'; 'c'; 'c']);
  assert (duplicate_map [1; 2] = [1; 1; 2; 2]);
  print_endline "✓ Duplicate elements tests passed"
