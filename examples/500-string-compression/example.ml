(* Run-length encoding in OCaml *)

(* Encode: "aabbbcc" -> [(2,'a');(3,'b');(2,'c')] *)
let encode (s : string) : (int * char) list =
  let chars = List.of_seq (String.to_seq s) in
  match chars with
  | [] -> []
  | first :: rest ->
    let (cur, count, acc) =
      List.fold_left
        (fun (cur, count, acc) c ->
          if c = cur then (cur, count + 1, acc)
          else (c, 1, (count, cur) :: acc))
        (first, 1, [])
        rest
    in
    List.rev ((count, cur) :: acc)

(* Decode: [(2,'a');(3,'b')] -> "aabbb" *)
let decode (pairs : (int * char) list) : string =
  List.fold_left
    (fun acc (n, c) -> acc ^ String.make n c)
    ""
    pairs

(* Pretty print encoded form *)
let show_encoded pairs =
  let parts = List.map (fun (n, c) -> Printf.sprintf "%d%c" n c) pairs in
  String.concat "" parts

let () =
  let tests = ["aabbbcc"; "aaaa"; "abcde"; ""; "aabbcc"] in
  List.iter (fun s ->
    let enc = encode s in
    let dec = decode enc in
    Printf.printf "Input:   %S\n" s;
    Printf.printf "Encoded: %s\n" (show_encoded enc);
    Printf.printf "Decoded: %S\n" dec;
    Printf.printf "Match:   %b\n\n" (s = dec)
  ) tests
