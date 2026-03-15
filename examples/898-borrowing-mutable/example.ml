(* Example 104: Mutable References — OCaml Mutable Fields → Rust &mut *)

(* Approach 1: Mutable record fields *)
type counter = { mutable count : int }

let increment c = c.count <- c.count + 1
let get_count c = c.count

let approach1 () =
  let c = { count = 0 } in
  increment c;
  increment c;
  increment c;
  assert (get_count c = 3);
  Printf.printf "Counter: %d\n" (get_count c)

(* Approach 2: Ref cells — mutable references *)
let approach2 () =
  let total = ref 0 in
  List.iter (fun x -> total := !total + x) [1; 2; 3; 4; 5];
  assert (!total = 15);
  Printf.printf "Total: %d\n" !total

(* Approach 3: Mutable arrays *)
let reverse_in_place arr =
  let n = Array.length arr in
  for i = 0 to n / 2 - 1 do
    let tmp = arr.(i) in
    arr.(i) <- arr.(n - 1 - i);
    arr.(n - 1 - i) <- tmp
  done

let approach3 () =
  let arr = [| 1; 2; 3; 4; 5 |] in
  reverse_in_place arr;
  assert (arr = [| 5; 4; 3; 2; 1 |]);
  Printf.printf "Reversed: %s\n"
    (String.concat ", " (Array.to_list (Array.map string_of_int arr)))

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
