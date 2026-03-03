(* Modified Run-Length Encoding — 99 Problems #11 *)
(* Single elements stay unwrapped, runs get (count, elem) *)

type 'a rle =
  | One of 'a
  | Many of int * 'a

(* ── Pack consecutive duplicates into sublists ───────────── *)

let pack lst =
  let rec aux current acc = function
    | [] -> List.rev (current :: acc)
    | x :: rest ->
      match current with
      | [] -> aux [x] acc rest
      | y :: _ when x = y -> aux (x :: current) acc rest
      | _ -> aux [x] (current :: acc) rest
  in match lst with
  | [] -> []
  | h :: t -> aux [h] [] t

(* ── Modified RLE using pack ─────────────────────────────── *)

let encode lst =
  pack lst
  |> List.map (fun group ->
       match group with
       | [x] -> One x
       | x :: _ -> Many (List.length group, x)
       | [] -> failwith "impossible")

(* ── Direct recursive version ────────────────────────────── *)

let encode_direct lst =
  let rec aux count acc = function
    | [] -> List.rev acc
    | [x] -> List.rev ((if count = 0 then One x else Many (count + 1, x)) :: acc)
    | x :: (y :: _ as rest) ->
      if x = y then aux (count + 1) acc rest
      else
        let item = if count = 0 then One x else Many (count + 1, x) in
        aux 0 (item :: acc) rest
  in aux 0 [] lst

(* ── Tests ────────────────────────────────────────────────── *)
let () =
  assert (encode [] = []);
  assert (encode ['a';'b';'c'] = [One 'a'; One 'b'; One 'c']);
  assert (encode ['x';'x';'x'] = [Many (3, 'x')]);
  assert (encode ['a';'a';'a';'b';'c';'c'] = [Many (3,'a'); One 'b'; Many (2,'c')]);
  assert (encode_direct ['a';'a';'b';'c';'c';'c'] = [Many (2,'a'); One 'b'; Many (3,'c')]);
  print_endline "✓ Modified RLE tests passed"
