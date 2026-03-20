# Comparison\n\n## OCaml\n\n```ocaml\nlet zeros = Array.make 5 0
let matrix = Array.make_matrix 3 4 0.0
let () = matrix.(1).(2) <- 42.0
let () =
  Array.iter (fun row ->
    Array.iter (fun x -> Printf.printf "%.0f " x) row;
    print_newline ()
  ) matrix\n```\n\n## Rust\n\n*TODO*\n