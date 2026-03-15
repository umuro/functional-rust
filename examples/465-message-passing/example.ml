(* 465: Message Passing vs Shared Memory
   Two approaches to concurrent word counting:
   1. Message passing: each domain computes a local count, sends it back.
   2. Shared memory: domains write to a shared Hashtbl under a Mutex. *)

module StringMap = Map.Make(String)

(* Count words in a string; returns a pure functional map *)
let count_words text =
  String.split_on_char ' ' text
  |> List.filter (fun s -> s <> "")
  |> List.map (fun w -> String.lowercase_ascii w)
  |> List.fold_left (fun m w ->
       let n = try StringMap.find w m with Not_found -> 0 in
       StringMap.add w (n + 1) m)
     StringMap.empty

(* Merge two word-count maps *)
let merge a b =
  StringMap.union (fun _ x y -> Some (x + y)) a b

(* Approach 1: message passing — each domain returns its local result *)
let msg_passing texts =
  let domains = List.map (fun t ->
    Domain.spawn (fun () -> count_words t)) texts
  in
  List.map Domain.join domains
  |> List.fold_left merge StringMap.empty

(* Approach 2: shared Hashtbl under a Mutex *)
let shared_mem texts =
  let mu     = Mutex.create () in
  let shared = Hashtbl.create 16 in
  let domains = List.map (fun t ->
    Domain.spawn (fun () ->
      let local = count_words t in
      (* Merge local into shared under the lock *)
      Mutex.lock mu;
      StringMap.iter (fun w n ->
        let prev = try Hashtbl.find shared w with Not_found -> 0 in
        Hashtbl.replace shared w (prev + n)
      ) local;
      Mutex.unlock mu))
    texts
  in
  List.iter Domain.join domains;
  (* Convert Hashtbl → StringMap for comparison *)
  Hashtbl.fold (fun k v m -> StringMap.add k v m) shared StringMap.empty

let () =
  let texts = ["a b c"; "a d"] in

  let mp  = msg_passing texts in
  let sm  = shared_mem  texts in
  assert (mp = sm);
  Printf.printf "message-passing == shared-memory: %b\n%!" (mp = sm);

  (* Spot check *)
  assert (StringMap.find "a" mp = 2);
  assert (StringMap.find "b" mp = 1);
  Printf.printf "word counts: a=%d b=%d c=%d d=%d\n%!"
    (StringMap.find "a" mp)
    (StringMap.find "b" mp)
    (StringMap.find "c" mp)
    (StringMap.find "d" mp)
