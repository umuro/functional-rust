(* 463: Fan-out / Fan-in
   Distribute work across N worker domains (fan-out) then
   collect all results back (fan-in). *)

(* Shared work queue protected by a mutex *)
type 'a work_queue = {
  items : 'a Queue.t;
  mu    : Mutex.t;
}

let make_wq items =
  let q = Queue.create () in
  List.iter (fun x -> Queue.push x q) items;
  { items = q; mu = Mutex.create () }

let next_work wq =
  Mutex.lock wq.mu;
  let v = if Queue.is_empty wq.items then None else Some (Queue.pop wq.items) in
  Mutex.unlock wq.mu;
  v

(* Fan-out map: apply [f] to items using [n] worker domains *)
let fan_map items n f =
  let wq = make_wq items in
  (* Result channel: a mutex-protected list *)
  let results_mu = Mutex.create () in
  let results    = ref [] in
  let workers = List.init n (fun _ ->
    Domain.spawn (fun () ->
      let local = ref [] in
      let rec loop () =
        match next_work wq with
        | None      -> ()
        | Some item -> local := f item :: !local; loop ()
      in
      loop ();
      (* Batch-append to shared results *)
      Mutex.lock results_mu;
      results := !local @ !results;
      Mutex.unlock results_mu))
  in
  List.iter Domain.join workers;
  !results

let () =
  (* fan_map: multiply each element by 2, using 4 workers *)
  let input = List.init 8 (fun i -> i + 1) in  (* [1..8] *)
  let r = fan_map input 4 (fun x -> x * 2) in
  let sorted = List.sort compare r in
  assert (sorted = [2;4;6;8;10;12;14;16]);
  Printf.printf "fan_map ×2: %s\n%!"
    (sorted |> List.map string_of_int |> String.concat ", ");

  (* All 100 items processed *)
  let r2 = fan_map (List.init 100 Fun.id) 8 Fun.id in
  assert (List.length r2 = 100);
  Printf.printf "fan_map 100 items via 8 workers: %d results\n%!" (List.length r2)
