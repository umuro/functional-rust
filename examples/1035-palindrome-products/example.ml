(* Palindrome Products *)
(* Finding smallest/largest palindrome products with factor tracking *)

let is_palindrome n =
  let s = string_of_int n in
  let len = String.length s in
  let rec check i = i >= len / 2 || (s.[i] = s.[len - 1 - i] && check (i + 1)) in
  check 0

let smallest ~min ~max =
  if min > max then Error "min must be <= max" else
  let best = ref None in
  for x = min to max do for y = x to max do
    let p = x * y in
    if is_palindrome p then
      match !best with
      | None -> best := Some (p, [(x, y)])
      | Some (b, fs) when p < b -> best := Some (p, [(x, y)])
      | Some (b, fs) when p = b -> best := Some (b, (x, y) :: fs)
      | _ -> ()
  done done;
  Ok !best

let largest ~min ~max =
  if min > max then Error "min must be <= max" else
  let best = ref None in
  for x = min to max do for y = x to max do
    let p = x * y in
    if is_palindrome p then
      match !best with
      | None -> best := Some (p, [(x, y)])
      | Some (b, fs) when p > b -> best := Some (p, [(x, y)])
      | Some (b, fs) when p = b -> best := Some (b, (x, y) :: fs)
      | _ -> ()
  done done;
  Ok !best
