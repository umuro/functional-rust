(* OCaml: concurrent patterns with Thread *)

let fetch_user id =
  Thread.delay 0.05;
  Printf.sprintf "User(%d)" id

let fetch_posts user_id =
  Thread.delay 0.03;
  [Printf.sprintf "Post1 by %d" user_id; Printf.sprintf "Post2 by %d" user_id]

let () =
  let user = fetch_user 42 in
  let posts = fetch_posts 42 in
  Printf.printf "User: %s\n" user;
  List.iter (Printf.printf "Post: %s\n") posts
