(* Z-Algorithm in OCaml — functional style with Array *)

(* Build Z-array: Z.(i) = length of longest match with prefix of s starting at i *)
let z_array (s : string) : int array =
  let n = String.length s in
  let z = Array.make n 0 in
  let chars = Array.init n (String.get s) in
  z.(0) <- n;
  let l = ref 0 and r = ref 0 in
  for i = 1 to n - 1 do
    if i < !r then
      z.(i) <- min (z.(i - !l)) (!r - i);
    while i + z.(i) < n && chars.(z.(i)) = chars.(i + z.(i)) do
      z.(i) <- z.(i) + 1
    done;
    if i + z.(i) > !r then begin
      l := i;
      r := i + z.(i)
    end
  done;
  z

(* Pattern search: returns list of 0-based match positions in text *)
let z_search (pattern : string) (text : string) : int list =
  let m = String.length pattern in
  let combined = pattern ^ "$" ^ text in
  let z = z_array combined in
  let n = String.length combined in
  (* Positions in combined where Z = m correspond to match starts in text *)
  let results = ref [] in
  for i = m + 1 to n - 1 do
    if z.(i) = m then
      results := (i - m - 1) :: !results
  done;
  List.rev !results

let () =
  let text = "aabxaabxaab" in
  let pattern = "aab" in
  let matches = z_search pattern text in
  Printf.printf "Pattern '%s' found at positions: " pattern;
  List.iter (fun p -> Printf.printf "%d " p) matches;
  print_newline ();

  (* Demonstrate Z-array directly *)
  let s = "aabxaa" in
  let z = z_array s in
  Printf.printf "Z-array of '%s': " s;
  Array.iter (fun v -> Printf.printf "%d " v) z;
  print_newline ()
