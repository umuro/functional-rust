(* Direct Run-Length Encoding — 99 Problems #13 *)
(* Encode directly without creating intermediate sublists *)

type 'a rle =
  | One of 'a
  | Many of int * 'a

(* ── Direct recursive with counting ──────────────────────── *)

let encode lst =
  let rec aux count acc = function
    | [] -> List.rev acc
    | [x] ->
      let item = if count = 0 then One x else Many (count + 1, x) in
      List.rev (item :: acc)
    | x :: (y :: _ as rest) ->
      if x = y then aux (count + 1) acc rest
      else
        let item = if count = 0 then One x else Many (count + 1, x) in
        aux 0 (item :: acc) rest
  in aux 0 [] lst

(* ── Fold-based approach ─────────────────────────────────── *)

let encode_fold lst =
  let update acc x =
    match acc with
    | Many (n, y) :: rest when y = x -> Many (n + 1, y) :: rest
    | One y :: rest when y = x -> Many (2, y) :: rest
    | _ -> One x :: acc
  in
  List.fold_left update [] lst |> List.rev

(* ── Tests ────────────────────────────────────────────────── *)
let () =
  assert (encode [] = []);
  assert (encode ['a';'b';'c'] = [One 'a'; One 'b'; One 'c']);
  assert (encode ['a';'a';'a';'a';'b';'c';'c';'a';'a';'d';'e';'e';'e';'e']
    = [Many (4,'a'); One 'b'; Many (2,'c'); Many (2,'a'); One 'd'; Many (4,'e')]);
  assert (encode_fold ['a';'a';'b'] = [Many (2,'a'); One 'b']);
  print_endline "✓ Direct RLE tests passed"
