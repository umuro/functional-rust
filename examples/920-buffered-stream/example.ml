(* OCaml: buffered concurrent processing *)

let process_with_limit n items f =
  let sem = Semaphore.counting_semaphore n in
  let results = Array.make (List.length items) None in
  let threads = List.mapi (fun i item ->
    Thread.create (fun () ->
      Semaphore.acquire sem;
      let r = f item in
      results.(i) <- Some r;
      Semaphore.release sem
    ) ()
  ) items in
  List.iter Thread.join threads;
  Array.to_list results |> List.filter_map Fun.id

(* Simpler version using Thread pool concept *)
let chunked_process chunk_size items f =
  let chunks =
    let rec go acc = function
      | [] -> List.rev acc
      | lst ->
        let chunk = List.filteri (fun i _ -> i < chunk_size) lst in
        let rest  = List.filteri (fun i _ -> i >= chunk_size) lst in
        go (chunk :: acc) rest
    in go [] items
  in
  List.concat_map (fun chunk ->
    let threads = List.map (fun x -> Thread.create (fun () -> f x) ()) chunk in
    List.map Thread.join threads
  ) chunks

let () =
  let items = List.init 10 (fun i -> i + 1) in
  let results = chunked_process 3 items (fun x ->
    Thread.delay (float_of_int (11 - x) *. 0.005);
    x * x
  ) in
  List.iter (Printf.printf "%d ") results;
  print_newline ()
