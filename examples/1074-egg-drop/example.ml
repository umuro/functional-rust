(* 1074: Egg Drop — DP + Binary Search *)

(* Approach 1: Basic DP O(k*n^2) *)
let egg_drop_basic eggs floors =
  let dp = Array.init (eggs + 1) (fun _ -> Array.make (floors + 1) 0) in
  for i = 1 to eggs do
    for j = 1 to floors do
      if i = 1 then dp.(i).(j) <- j
      else begin
        dp.(i).(j) <- max_int;
        for x = 1 to j do
          let worst = 1 + max dp.(i - 1).(x - 1) dp.(i).(j - x) in
          dp.(i).(j) <- min dp.(i).(j) worst
        done
      end
    done
  done;
  dp.(eggs).(floors)

(* Approach 2: DP with binary search O(k*n*log(n)) *)
let egg_drop_bs eggs floors =
  let dp = Array.init (eggs + 1) (fun _ -> Array.make (floors + 1) 0) in
  for i = 1 to eggs do
    for j = 1 to floors do
      if i = 1 then dp.(i).(j) <- j
      else begin
        let lo = ref 1 and hi = ref j in
        while !lo < !hi do
          let mid = (!lo + !hi) / 2 in
          let broke = dp.(i - 1).(mid - 1) in
          let survived = dp.(i).(j - mid) in
          if broke < survived then lo := mid + 1
          else hi := mid
        done;
        let x = !lo in
        dp.(i).(j) <- 1 + max dp.(i - 1).(x - 1) dp.(i).(j - x)
      end
    done
  done;
  dp.(eggs).(floors)

(* Approach 3: Optimal DP — how many floors can we check with t trials and k eggs? *)
let egg_drop_optimal eggs floors =
  (* dp.(t).(k) = max floors checkable with t trials and k eggs *)
  let dp = Array.init (floors + 1) (fun _ -> Array.make (eggs + 1) 0) in
  let t = ref 0 in
  while dp.(!t).(eggs) < floors do
    incr t;
    for k = 1 to eggs do
      dp.(!t).(k) <- 1 + dp.(!t - 1).(k - 1) + dp.(!t - 1).(k)
    done
  done;
  !t

let () =
  assert (egg_drop_basic 1 10 = 10);
  assert (egg_drop_basic 2 10 = 4);
  assert (egg_drop_basic 2 6 = 3);
  assert (egg_drop_basic 3 14 = 4);

  assert (egg_drop_bs 1 10 = 10);
  assert (egg_drop_bs 2 10 = 4);
  assert (egg_drop_bs 2 6 = 3);

  assert (egg_drop_optimal 1 10 = 10);
  assert (egg_drop_optimal 2 10 = 4);
  assert (egg_drop_optimal 2 6 = 3);
  assert (egg_drop_optimal 2 100 = 14);

  Printf.printf "✓ All tests passed\n"
