(* Replicate Elements N Times — 99 Problems #15 *)
(* replicate [a;b;c] 3 → [a;a;a;b;b;b;c;c;c] *)

(* ── Recursive with helper ───────────────────────────────── *)

let replicate lst n =
  let rec repeat x = function
    | 0 -> []
    | k -> x :: repeat x (k - 1)
  in
  let rec aux = function
    | [] -> []
    | h :: t -> repeat h n @ aux t
  in aux lst

(* ── Tail-recursive ──────────────────────────────────────── *)

let replicate_tr lst n =
  let rec aux acc = function
    | [] -> List.rev acc
    | h :: t ->
      let rec add_n acc = function
        | 0 -> aux acc t
        | k -> add_n (h :: acc) (k - 1)
      in add_n acc n
  in aux [] lst

(* ── Using concat_map + List.init ────────────────────────── *)

let replicate_map lst n =
  List.concat_map (fun x -> List.init n (fun _ -> x)) lst

(* ── Tests ────────────────────────────────────────────────── *)
let () =
  assert (replicate [] 5 = []);
  assert (replicate [1;2;3] 0 = []);
  assert (replicate [1;2;3] 1 = [1;2;3]);
  assert (replicate ['a';'b';'c'] 3 = ['a';'a';'a';'b';'b';'b';'c';'c';'c']);
  assert (replicate_tr [1;2] 3 = [1;1;1;2;2;2]);
  assert (replicate_map [1;2] 2 = [1;1;2;2]);
  print_endline "✓ Replicate tests passed"
