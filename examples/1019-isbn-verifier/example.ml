(* ISBN Verifier *)
(* Checksum validation with weighted digit processing *)

let is_valid s =
  let chars = List.init (String.length s) (String.get s) in
  let rec aux chars acc count =
    match chars with
    | [] -> count = 10 && acc mod 11 = 0
    | '-' :: rest | ' ' :: rest -> aux rest acc count
    | 'X' :: rest when count = 9 -> aux rest (acc + 10 * (count + 1)) (count + 1)
    | c :: rest when c >= '0' && c <= '9' ->
      let value = Char.code c - Char.code '0' in
      aux rest (acc + value * (count + 1)) (count + 1)
    | _ -> false
  in
  aux chars 0 0
