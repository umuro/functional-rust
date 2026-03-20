(* Idiomatic OCaml — Array.blit copies a subarray in place *)
let src = [| 10; 20; 30; 40; 50 |]
let dst = Array.make 8 0
let () = Array.blit src 1 dst 2 3
(* Array.blit src src_pos dst dst_pos len *)

(* Recursive OCaml — manual copy without mutation *)
let rec blit_rec src src_pos dst dst_pos len =
  if len = 0 then dst
  else begin
    dst.(dst_pos) <- src.(src_pos);
    blit_rec src (src_pos + 1) dst (dst_pos + 1) (len - 1)
  end

let () =
  assert (dst = [| 0; 0; 20; 30; 40; 0; 0; 0 |]);
  let dst2 = Array.make 8 0 in
  let _ = blit_rec src 1 dst2 2 3 in
  assert (dst2 = [| 0; 0; 20; 30; 40; 0; 0; 0 |]);
  Array.iter (fun x -> Printf.printf "%d " x) dst;
  print_newline ();
  print_endline "ok"
