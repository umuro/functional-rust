(* 478: String Searching
   OCaml stdlib functions for contains, find, starts_with, ends_with,
   and counting matches. *)

(* contains: check if needle is a substring of haystack *)
let contains haystack needle =
  let hn = String.length haystack in
  let nn = String.length needle in
  if nn = 0 then true
  else if nn > hn then false
  else begin
    let found = ref false in
    let i = ref 0 in
    while !i <= hn - nn && not !found do
      if String.sub haystack !i nn = needle
      then found := true
      else incr i
    done;
    !found
  end

(* First byte index of a character — like find(char) *)
let find_char s c = String.index_opt s c

(* Last byte index of a character — like rfind(char) *)
let rfind_char s c =
  let n = String.length s in
  let i = ref (n - 1) in
  let result = ref None in
  while !i >= 0 && !result = None do
    if s.[!i] = c then result := Some !i;
    decr i
  done;
  !result

(* Count non-overlapping occurrences of a character *)
let count_char s c =
  String.fold_left (fun acc ch -> if ch = c then acc + 1 else acc) 0 s

let starts_with s prefix =
  String.length s >= String.length prefix &&
  String.sub s 0 (String.length prefix) = prefix

let ends_with s suffix =
  let sn = String.length s and en = String.length suffix in
  sn >= en && String.sub s (sn - en) en = suffix

let () =
  (* contains *)
  assert (contains "hello world" "world");
  assert (not (contains "hello" "xyz"));
  Printf.printf "contains \"world\": %b / contains \"xyz\": %b\n%!"
    (contains "hello world" "world") (contains "hello" "xyz");

  (* starts_with / ends_with *)
  assert (starts_with "hello" "hel");
  assert (not (ends_with "hello" "hel"));
  Printf.printf "starts_with \"hel\": %b / ends_with \"hel\": %b\n%!"
    (starts_with "hello" "hel") (ends_with "hello" "hel");

  (* find_char *)
  assert (find_char "hello" 'l' = Some 2);
  assert (find_char "hello" 'z' = None);
  Printf.printf "find 'l': %s / find 'z': %s\n%!"
    (find_char "hello" 'l' |> Option.map string_of_int |> Option.value ~default:"None")
    (find_char "hello" 'z' |> Option.map string_of_int |> Option.value ~default:"None");

  (* rfind_char *)
  assert (rfind_char "hello" 'l' = Some 3);
  Printf.printf "rfind 'l': %s\n%!"
    (rfind_char "hello" 'l' |> Option.map string_of_int |> Option.value ~default:"None");

  (* count_char — like matches('a').count() *)
  assert (count_char "aaabaa" 'a' = 5);
  Printf.printf "count 'a' in \"aaabaa\": %d\n%!" (count_char "aaabaa" 'a')
