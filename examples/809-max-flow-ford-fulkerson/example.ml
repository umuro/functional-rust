(* Edmonds-Karp Max Flow — BFS augmenting paths, O(VE²) *)
open Queue

let max_flow_bfs cap n src snk =
  let cap = Array.init n (fun i -> Array.copy cap.(i)) in  (* mutable copy *)
  let total = ref 0 in

  let rec augment () =
    (* BFS to find augmenting path *)
    let parent = Array.make n (-1) in
    parent.(src) <- src;
    let q = Queue.create () in
    Queue.push src q;
    let found = ref false in
    while not (Queue.is_empty q) && not !found do
      let u = Queue.pop q in
      for v = 0 to n - 1 do
        if parent.(v) = -1 && cap.(u).(v) > 0 then begin
          parent.(v) <- u;
          if v = snk then found := true
          else Queue.push v q
        end
      done
    done;
    if !found then begin
      (* Find bottleneck *)
      let flow = ref max_int in
      let v = ref snk in
      while !v <> src do
        let u = parent.(!v) in
        flow := min !flow cap.(u).(!v);
        v    := u
      done;
      (* Update residual capacities *)
      let v = ref snk in
      while !v <> src do
        let u = parent.(!v) in
        cap.(u).(!v) <- cap.(u).(!v) - !flow;
        cap.(!v).(u) <- cap.(!v).(u) + !flow;
        v := u
      done;
      total := !total + !flow;
      augment ()
    end
  in
  augment ();
  !total

let () =
  let n = 6 in
  let cap = Array.make_matrix n n 0 in
  let set u v c = cap.(u).(v) <- c in
  set 0 1 16; set 0 2 13;
  set 1 2 10; set 1 3 12;
  set 2 1  4; set 2 4 14;
  set 3 2  9; set 3 5 20;
  set 4 3  7; set 4 5  4;
  Printf.printf "Max flow (0->5): %d  (expected 23)\n" (max_flow_bfs cap n 0 5)
