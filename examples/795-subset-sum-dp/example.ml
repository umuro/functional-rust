(* Subset Sum — boolean DP, O(n×T), plus all-subsets reconstruction *)

let subset_sum nums target =
  (* dp.(s) = true if sum s is achievable *)
  let dp = Array.make (target + 1) false in
  dp.(0) <- true;
  List.iter (fun x ->
    (* Iterate in reverse to avoid using x twice (0/1 knapsack) *)
    for s = target downto x do
      if dp.(s - x) then dp.(s) <- true
    done
  ) nums;
  dp.(target)

(* Find the actual subset *)
let subset_find nums target =
  let n      = List.length nums in
  let arr    = Array.of_list nums in
  let dp     = Array.make_matrix (n + 1) (target + 1) false in
  dp.(0).(0) <- true;
  for i = 1 to n do
    let x = arr.(i - 1) in
    for s = 0 to target do
      dp.(i).(s) <- dp.(i-1).(s) || (s >= x && dp.(i-1).(s-x))
    done
  done;
  if not dp.(n).(target) then None
  else begin
    let subset = ref [] in
    let s = ref target in
    for i = n downto 1 do
      if not dp.(i-1).(!s) then begin
        subset := arr.(i-1) :: !subset;
        s      := !s - arr.(i-1)
      end
    done;
    Some !subset
  end

let () =
  let nums   = [3; 34; 4; 12; 5; 2] in
  let target = 9 in
  Printf.printf "nums=%s target=%d\n"
    ("[" ^ String.concat ";" (List.map string_of_int nums) ^ "]") target;
  Printf.printf "Has subset summing to %d: %b\n" target (subset_sum nums target);
  (match subset_find nums target with
   | None   -> Printf.printf "Subset: none\n"
   | Some s -> Printf.printf "Subset: [%s]\n"
       (String.concat "; " (List.map string_of_int s)));
  Printf.printf "Has subset summing to 30: %b\n" (subset_sum nums 30)
