(* Decode Run-Length Encoding — 99 Problems #12 *)

type 'a rle =
  | One of 'a
  | Many of int * 'a

(* ── Using List.concat_map (OCaml 4.10+) ────────────────── *)

let decode lst =
  List.concat_map (function
    | One x -> [x]
    | Many (n, x) -> List.init n (fun _ -> x))
  lst

(* ── Recursive with accumulator ──────────────────────────── *)

let decode_recursive lst =
  let rec expand = function
    | One x -> [x]
    | Many (0, _) -> []
    | Many (n, x) -> x :: expand (Many (n - 1, x))
  in
  let rec aux acc = function
    | [] -> List.rev acc
    | item :: rest -> aux (List.rev_append (expand item) acc) rest
  in aux [] lst

(* ── Tail-recursive version ──────────────────────────────── *)

let decode_tr lst =
  let rec aux acc = function
    | [] -> List.rev acc
    | One x :: rest -> aux (x :: acc) rest
    | Many (0, _) :: rest -> aux acc rest
    | Many (n, x) :: rest -> aux (x :: acc) (Many (n - 1, x) :: rest)
  in aux [] lst

(* ── Tests ────────────────────────────────────────────────── *)
let () =
  assert (decode [] = []);
  assert (decode [One 'a'; One 'b'] = ['a'; 'b']);
  assert (decode [Many (3, 'x')] = ['x'; 'x'; 'x']);
  assert (decode [Many (3,'a'); One 'b'; Many (2,'c')] = ['a';'a';'a';'b';'c';'c']);
  assert (decode_recursive [Many (2,'x'); One 'y'] = ['x';'x';'y']);
  assert (decode_tr [Many (3,'a'); One 'b'] = ['a';'a';'a';'b']);
  print_endline "✓ Decode RLE tests passed"
