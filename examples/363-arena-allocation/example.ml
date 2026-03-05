(* OCaml: region-like pattern with a pool *)

type 'a arena = {
  mutable items: 'a list;
  mutable count: int;
}

let make_arena () = { items=[]; count=0 }

let arena_alloc a v =
  a.items <- v :: a.items;
  a.count <- a.count + 1;
  v

let arena_reset a =
  Printf.printf "Freeing %d items\n" a.count;
  a.items <- []; a.count <- 0

let () =
  let arena = make_arena () in
  let _ = List.init 5 (fun i -> arena_alloc arena (i*i)) in
  Printf.printf "Items: %d\n" arena.count;
  arena_reset arena;
  Printf.printf "After reset: %d\n" arena.count
