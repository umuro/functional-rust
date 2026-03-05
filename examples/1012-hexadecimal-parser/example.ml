(* Hexadecimal Parser *)
(* Character-to-integer conversion with pattern matching *)

let digit_to_int = function
  | '0'..'9' as c -> Some (Char.code c - 48)
  | 'a'..'f' as c -> Some (Char.code c - 87)
  | 'A'..'F' as c -> Some (Char.code c - 55)
  | _ -> None

let to_int hex_str =
  let chars = List.init (String.length hex_str) (String.get hex_str) in
  let rec go acc = function
    | [] -> acc
    | c :: cs -> match digit_to_int c with
      | Some n -> go (acc * 16 + n) cs
      | None -> 0
  in
  go 0 chars
