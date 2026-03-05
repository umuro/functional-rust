(* Manacher's Algorithm in OCaml *)

(* Transform "abc" -> "#a#b#c#" so all palindromes are odd-length *)
let transform (s : string) : string =
  let n = String.length s in
  let buf = Buffer.create (2 * n + 1) in
  Buffer.add_char buf '#';
  String.iter (fun c -> Buffer.add_char buf c; Buffer.add_char buf '#') s;
  Buffer.contents buf

(* Compute palindrome radius array for transformed string t *)
let manacher (t : string) : int array =
  let n = String.length t in
  let p = Array.make n 0 in
  let c = ref 0 and r = ref 0 in
  for i = 0 to n - 1 do
    let mirror = 2 * !c - i in
    if i < !r then
      p.(i) <- min (p.(mirror)) (!r - i);
    (* Attempt to expand *)
    let a = ref (i - (p.(i) + 1))
    and b = ref (i + (p.(i) + 1)) in
    while !a >= 0 && !b < n && t.[!a] = t.[!b] do
      p.(i) <- p.(i) + 1;
      decr a; incr b
    done;
    if i + p.(i) > !r then begin
      c := i;
      r := i + p.(i)
    end
  done;
  p

(* Return the longest palindromic substring of s *)
let longest_palindrome (s : string) : string =
  if String.length s = 0 then ""
  else begin
    let t = transform s in
    let p = manacher t in
    (* Find centre with maximum radius *)
    let best_c = ref 0 and best_r = ref 0 in
    Array.iteri (fun i r ->
      if r > !best_r then begin best_c := i; best_r := r end
    ) p;
    (* Map back to original string *)
    let start = (!best_c - !best_r) / 2 in
    String.sub s start !best_r
  end

let () =
  let tests = [
    ("babad", "bab");
    ("cbbd",  "bb");
    ("racecar", "racecar");
    ("abacaba", "abacaba");
    ("a", "a");
    ("aabbaa", "aabbaa");
  ] in
  List.iter (fun (input, _expected) ->
    let result = longest_palindrome input in
    Printf.printf "longest_palindrome(%S) = %S\n" input result
  ) tests
