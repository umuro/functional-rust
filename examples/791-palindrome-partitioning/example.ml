(* Palindrome Partitioning — minimum cuts DP O(n²) *)

let palindrome_partition s =
  let n = String.length s in
  if n = 0 then (0, [])
  else begin
    (* is_pal.(i).(j) = true if s[i..j] is palindrome *)
    let is_pal = Array.make_matrix n n false in
    (* Single chars *)
    for i = 0 to n - 1 do is_pal.(i).(i) <- true done;
    (* Length 2 *)
    for i = 0 to n - 2 do
      is_pal.(i).(i+1) <- s.[i] = s.[i+1]
    done;
    (* Length 3+ *)
    for len = 3 to n do
      for i = 0 to n - len do
        let j = i + len - 1 in
        is_pal.(i).(j) <- s.[i] = s.[j] && is_pal.(i+1).(j-1)
      done
    done;

    (* cuts.(i) = min cuts for s[0..i] *)
    let cuts = Array.make n max_int in
    let prev = Array.make n (-1) in  (* where the last partition starts *)
    for i = 0 to n - 1 do
      if is_pal.(0).(i) then begin
        cuts.(i) <- 0;
        prev.(i) <- 0
      end else begin
        for j = 1 to i do
          if is_pal.(j).(i) then begin
            let c = cuts.(j-1) + 1 in
            if c < cuts.(i) then begin
              cuts.(i) <- c;
              prev.(i) <- j
            end
          end
        done
      end
    done;

    (* Reconstruct partitions *)
    let parts = ref [] in
    let j = ref (n - 1) in
    while !j >= 0 do
      let start = prev.(!j) in
      parts := String.sub s start (!j - start + 1) :: !parts;
      j := start - 1
    done;
    (cuts.(n-1), !parts)
  end

let () =
  let test s =
    let (cuts, parts) = palindrome_partition s in
    Printf.printf "s = %S -> cuts = %d, parts = [%s]\n"
      s cuts (String.concat "; " (List.map (Printf.sprintf "%S") parts))
  in
  test "aab";
  test "a";
  test "ab";
  test "aabb";
  test "racecaranana"
