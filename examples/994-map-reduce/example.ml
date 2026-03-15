(* 994: Map-Reduce
   Parallel map over partitions followed by a sequential reduce.
   OCaml 5 Domains for true CPU parallelism; Thread-based variant also shown.
   The pattern: split input → map each chunk in parallel → reduce results. *)

(* --- Generic map-reduce with OCaml 5 Domains --- *)
let parallel_map_reduce
    ?(n_workers = Domain.recommended_domain_count ())
    ~map ~reduce ~init items =

  let arr = Array.of_list items in
  let n   = Array.length arr in
  if n = 0 then init
  else begin
    (* Split into n_workers chunks *)
    let chunk_size = max 1 ((n + n_workers - 1) / n_workers) in
    let chunks = Array.init n_workers (fun i ->
      let lo = i * chunk_size in
      let hi = min n ((i + 1) * chunk_size) in
      if lo >= n then [||]
      else Array.sub arr lo (hi - lo)
    ) in

    (* Map phase: each domain processes one chunk *)
    let domains = Array.map (fun chunk ->
      if Array.length chunk = 0 then Domain.spawn (fun () -> init)
      else Domain.spawn (fun () ->
        Array.fold_left (fun acc x -> reduce acc (map x)) init chunk
      )
    ) chunks in

    (* Reduce phase: combine domain results *)
    Array.fold_left (fun acc d -> reduce acc (Domain.join d)) init domains
  end

(* --- Word frequency count (classic map-reduce) --- *)
let word_frequency_mr docs =
  (* Map: tokenize each document into word counts *)
  let map doc =
    let words = String.split_on_char ' ' doc
                |> List.filter (fun s -> s <> "")
                |> List.map String.lowercase_ascii in
    let tbl = Hashtbl.create 16 in
    List.iter (fun w ->
      let c = Option.value (Hashtbl.find_opt tbl w) ~default:0 in
      Hashtbl.replace tbl w (c + 1)
    ) words;
    tbl
  in
  (* Reduce: merge two frequency tables *)
  let reduce acc tbl =
    let merged = Hashtbl.copy acc in
    Hashtbl.iter (fun w c ->
      let prev = Option.value (Hashtbl.find_opt merged w) ~default:0 in
      Hashtbl.replace merged w (prev + c)
    ) tbl;
    merged
  in
  let domains = Array.of_list (List.map (fun doc ->
    Domain.spawn (fun () -> map doc)
  ) docs) in
  let tables = Array.map Domain.join domains in
  Array.fold_left reduce (Hashtbl.create 16) tables

(* --- Parallel sum / min / max using chunks --- *)
let parallel_sum arr =
  let n = Array.length arr in
  let nw = min n (Domain.recommended_domain_count ()) in
  let chunk = max 1 ((n + nw - 1) / nw) in
  let domains = Array.init nw (fun i ->
    Domain.spawn (fun () ->
      let lo = i * chunk and hi = min n ((i+1)*chunk) in
      let s = ref 0 in
      for j = lo to hi-1 do s := !s + arr.(j) done;
      !s
    )
  ) in
  Array.fold_left (fun acc d -> acc + Domain.join d) 0 domains

let parallel_histogram arr n_buckets max_val =
  let counts = Array.make n_buckets 0 in
  let mutex  = Mutex.create () in
  let nw = min (Array.length arr) 4 in
  let chunk = max 1 ((Array.length arr + nw - 1) / nw) in
  let domains = Array.init nw (fun i ->
    Domain.spawn (fun () ->
      let local = Array.make n_buckets 0 in
      let lo = i * chunk and hi = min (Array.length arr) ((i+1)*chunk) in
      for j = lo to hi-1 do
        let b = arr.(j) * n_buckets / (max_val + 1) in
        local.(b) <- local.(b) + 1
      done;
      Mutex.lock mutex;
      Array.iteri (fun b c -> counts.(b) <- counts.(b) + c) local;
      Mutex.unlock mutex
    )
  ) in
  Array.iter Domain.join domains;
  counts

let () =
  Printf.printf "=== Parallel map-reduce: sum of squares ===\n";
  let items = List.init 100 (fun i -> i + 1) in
  let result = parallel_map_reduce
    ~map:(fun x -> x * x)
    ~reduce:( + )
    ~init:0
    items
  in
  Printf.printf "sum of squares 1..100 = %d (expected 338350)\n" result;

  Printf.printf "\n=== Parallel sum ===\n";
  let arr = Array.init 1000 (fun i -> i + 1) in
  Printf.printf "sum 1..1000 = %d (expected 500500)\n" (parallel_sum arr);

  Printf.printf "\n=== Word frequency (map-reduce) ===\n";
  let docs = [
    "the quick brown fox";
    "the fox jumped over the lazy dog";
    "the dog slept all day";
    "quick brown dog quick";
  ] in
  let freq = word_frequency_mr docs in
  let sorted = Hashtbl.fold (fun w c acc -> (w,c)::acc) freq []
               |> List.sort (fun (_,a) (_,b) -> compare b a) in
  Printf.printf "top words:\n";
  List.iter (fun (w,c) -> Printf.printf "  %-10s %d\n" w c)
    (List.filteri (fun i _ -> i < 6) sorted);

  Printf.printf "\n=== Parallel histogram ===\n";
  let data = Array.init 50 (fun i -> i mod 10) in
  let hist = parallel_histogram data 10 9 in
  Printf.printf "histogram (each bucket should be 5):\n";
  Array.iteri (fun b c -> Printf.printf "  [%d..%d]: %d\n" b b c) hist
