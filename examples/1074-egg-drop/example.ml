(* 1074: Egg Drop — DP + Binary Search and Optimal approach *)

(* Approach 1: Basic DP O(k * n^2) *)
let egg_drop_basic eggs floors =
  let dp = Array.init (eggs + 1) (fun _ -> Array.make (floors + 1) 0) in
  for i = 1 to eggs do
    for j = 1 to floors do
      if i = 1 then dp.(i).(j) <- j
      else begin
        let best = ref max_int in
        for x = 1 to j do
          let worst = 1 + max dp.(i - 1).(x - 1) dp.(i).(j - x) in
          if worst < !best then best := worst
        done;
        dp.(i).(j) <- !best
      end
    done
  done;
  dp.(eggs).(floors)

(* Approach 2: DP with binary search O(k * n * log n) *)
let egg_drop_bs eggs floors =
  let dp = Array.init (eggs + 1) (fun _ -> Array.make (floors + 1) 0) in
  for i = 1 to eggs do
    for j = 1 to floors do
      if i = 1 then dp.(i).(j) <- j
      else begin
        let lo = ref 1 and hi = ref j in
        while !lo < !hi do
          let mid = (!lo + !hi) / 2 in
          if dp.(i - 1).(mid - 1) < dp.(i).(j - mid) then lo := mid + 1
          else hi := mid
        done;
        dp.(i).(j) <- 1 + max dp.(i - 1).(!lo - 1) dp.(i).(j - !lo)
      end
    done
  done;
  dp.(eggs).(floors)

(* Approach 3: Optimal — how many floors can we cover with t trials and k eggs?
   dp.(t).(k) = max floors checkable with t trials and k eggs
   Recurrence: dp.(t).(k) = 1 + dp.(t-1).(k-1) + dp.(t-1).(k)
   Find minimal t such that dp.(t).(eggs) >= floors *)
let egg_drop_optimal eggs floors =
  let dp = Array.init (floors + 1) (fun _ -> Array.make (eggs + 1) 0) in
  let result = ref floors in (* worst case: 1 egg needs floors trials *)
  let t = ref 1 in
  let found = ref false in
  while not !found && !t <= floors do
    for k = 1 to eggs do
      dp.(!t).(k) <- 1 + dp.(!t - 1).(k - 1) + dp.(!t - 1).(k);
      if dp.(!t).(k) >= floors && k = eggs && not !found then begin
        result := !t;
        found := true
      end
    done;
    incr t
  done;
  !result

let () =
  let cases = [(1, 10, 10); (2, 10, 4); (2, 6, 3); (2, 100, 14)] in
  List.iter (fun (eggs, floors, expected) ->
    let r1 = egg_drop_basic eggs floors in
    let r2 = egg_drop_bs eggs floors in
    let r3 = egg_drop_optimal eggs floors in
    Printf.printf "egg_drop(%d eggs, %d floors) = basic:%d bs:%d optimal:%d (expected:%d)\n"
      eggs floors r1 r2 r3 expected
  ) cases
