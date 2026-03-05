(* 281. Implementing Iterator trait from scratch - OCaml *)
(* OCaml uses Seq.t for lazy sequences *)

type 'a counter = {
  mutable current: int;
  max: int;
  step: int;
  f: int -> 'a;
}

let make_counter ?(step=1) max f =
  { current = 0; max; step; f }

let counter_next c =
  if c.current >= c.max then None
  else begin
    let v = c.f c.current in
    c.current <- c.current + c.step;
    Some v
  end

let counter_to_seq c =
  Seq.unfold (fun () ->
    counter_next c |> Option.map (fun v -> (v, ()))
  ) ()

let () =
  let c = make_counter 5 (fun i -> i * i) in
  let squares = counter_to_seq c |> List.of_seq in
  Printf.printf "Squares: %s\n"
    (String.concat ", " (List.map string_of_int squares));

  (* Fibonacci sequence as lazy iterator *)
  let fib_seq =
    Seq.unfold (fun (a, b) -> Some (a, (b, a + b))) (0, 1)
  in
  let first10 = Seq.take 10 fib_seq |> List.of_seq in
  Printf.printf "First 10 Fibonacci: %s\n"
    (String.concat ", " (List.map string_of_int first10))
