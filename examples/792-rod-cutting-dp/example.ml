(* Rod Cutting — bottom-up DP, unbounded knapsack style *)

let rod_cut prices =
  let n = Array.length prices in
  let dp   = Array.make (n + 1) 0 in
  let cuts = Array.make (n + 1) 0 in
  for i = 1 to n do
    for j = 1 to i do
      let v = prices.(j - 1) + dp.(i - j) in
      if v > dp.(i) then begin
        dp.(i)   <- v;
        cuts.(i) <- j
      end
    done
  done;
  (* Reconstruct cut sequence *)
  let pieces = ref [] in
  let len    = ref n in
  while !len > 0 do
    pieces := cuts.(!len) :: !pieces;
    len    := !len - cuts.(!len)
  done;
  (dp.(n), List.rev !pieces)

let () =
  (* prices.(i) = price for a rod of length i+1 *)
  let prices = [| 1; 5; 8; 9; 10; 17; 17; 20 |] in
  let (revenue, pieces) = rod_cut prices in
  Printf.printf "Rod length: %d\n" (Array.length prices);
  Printf.printf "Max revenue: %d\n" revenue;
  Printf.printf "Cut into: [%s]\n"
    (String.concat "; " (List.map string_of_int pieces))
