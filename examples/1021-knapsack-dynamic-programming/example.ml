(* Knapsack (Dynamic Programming) *)
(* 0/1 knapsack with bottom-up DP using arrays *)

type item = { weight : int; value : int }

let maximum_value items capacity =
  let dp = Array.make (capacity + 1) 0 in
  List.iter (fun item ->
    for c = capacity downto item.weight do
      let with_item = dp.(c - item.weight) + item.value in
      if with_item > dp.(c) then dp.(c) <- with_item
    done
  ) items;
  dp.(capacity)
