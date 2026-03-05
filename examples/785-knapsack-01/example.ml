(* 0/1 Knapsack in OCaml — recursive + memoisation + tabulation *)

type item = { weight: int; value: int; name: string }

(* ── Recursive with memoisation ───────────────────────────────────────────────── *)
let solve_memo items capacity =
  let n = Array.length items in
  let memo = Hashtbl.create 64 in
  let rec dp i w =
    if i = n || w = 0 then 0
    else match Hashtbl.find_opt memo (i, w) with
    | Some v -> v
    | None ->
      let item = items.(i) in
      let best =
        if item.weight > w then dp (i+1) w
        else max (dp (i+1) w) (item.value + dp (i+1) (w - item.weight))
      in
      Hashtbl.replace memo (i, w) best;
      best
  in
  dp 0 capacity

(* ── Tabulation ────────────────────────────────────────────────────────────────── *)
let solve_tab items capacity =
  let n = Array.length items in
  (* dp.(i).(w) = max value using first i items with capacity w *)
  let dp = Array.make_matrix (n+1) (capacity+1) 0 in
  for i = 1 to n do
    let item = items.(i-1) in
    for w = 0 to capacity do
      dp.(i).(w) <-
        if item.weight > w then dp.(i-1).(w)
        else max dp.(i-1).(w) (item.value + dp.(i-1).(w - item.weight))
    done
  done;
  (* Traceback *)
  let selected = ref [] in
  let w = ref capacity in
  for i = n downto 1 do
    if dp.(i).(!w) <> dp.(i-1).(!w) then begin
      selected := items.(i-1) :: !selected;
      w := !w - items.(i-1).weight
    end
  done;
  dp.(n).(capacity), !selected

let () =
  let items = [|
    { weight=2; value=6;  name="camera"  };
    { weight=2; value=10; name="laptop"  };
    { weight=3; value=12; name="guitar"  };
    { weight=1; value=4;  name="book"    };
    { weight=4; value=15; name="drone"   };
  |] in
  let capacity = 7 in
  let best_memo = solve_memo items capacity in
  Printf.printf "Memo  best value: %d\n" best_memo;
  let (best_tab, selected) = solve_tab items capacity in
  Printf.printf "Table best value: %d\n" best_tab;
  Printf.printf "Selected items:\n";
  List.iter (fun item ->
    Printf.printf "  %s (w=%d, v=%d)\n" item.name item.weight item.value
  ) selected
