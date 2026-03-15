(* 277. Counting with count() - OCaml *)

let () =
  let nums = List.init 10 (fun i -> i + 1) in
  Printf.printf "Count: %d\n" (List.length nums);

  let evens = List.length (List.filter (fun x -> x mod 2 = 0) nums) in
  Printf.printf "Evens: %d\n" evens;

  let s = "hello world" in
  let vowels = String.fold_left
    (fun acc c -> if String.contains "aeiou" c then acc + 1 else acc) 0 s in
  Printf.printf "Vowels in '%s': %d\n" s vowels;

  let text = "the quick brown fox jumps over the lazy dog" in
  Printf.printf "Word count: %d\n"
    (List.length (String.split_on_char ' ' text))
