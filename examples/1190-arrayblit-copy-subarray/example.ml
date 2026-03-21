let src = [| 10; 20; 30; 40; 50 |]
let dst = Array.make 8 0
let () = Array.blit src 1 dst 2 3
(* dst is now [| 0; 0; 20; 30; 40; 0; 0; 0 |] *)
let () = Array.iter (fun x -> Printf.printf "%d " x) dst
