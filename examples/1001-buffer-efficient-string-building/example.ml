(* Buffer — Efficient String Building *)
(* Build strings incrementally with Buffer *)

let build_csv rows =
  let buf = Buffer.create 256 in
  List.iter (fun row ->
    Buffer.add_string buf (String.concat "," row);
    Buffer.add_char buf '\n'
  ) rows;
  Buffer.contents buf

let data = [
  ["name"; "age"; "city"];
  ["Alice"; "30"; "Amsterdam"];
  ["Bob"; "25"; "Berlin"]
]
let csv = build_csv data
let () = print_string csv
