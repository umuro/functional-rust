(* Rectangles *)
(* Counting rectangles in ASCII art by tracing edges *)

let count_rectangles pic =
  let h = Array.length pic in
  if h = 0 then 0 else
  let w = String.length pic.(0) in
  let at r c = if r >= 0 && r < h && c >= 0 && c < w then pic.(r).[c] else '.' in
  let count = ref 0 in
  for r1 = 0 to h - 1 do
    for c1 = 0 to w - 1 do
      if at r1 c1 = '+' then
        for r2 = r1 + 1 to h - 1 do
          for c2 = c1 + 1 to w - 1 do
            if at r1 c2 = '+' && at r2 c1 = '+' && at r2 c2 = '+' then
              let top_ok = ref true and bot_ok = ref true
              and lft_ok = ref true and rgt_ok = ref true in
              for c = c1 + 1 to c2 - 1 do
                if at r1 c <> '-' && at r1 c <> '+' then top_ok := false;
                if at r2 c <> '-' && at r2 c <> '+' then bot_ok := false
              done;
              for r = r1 + 1 to r2 - 1 do
                if at r c1 <> '|' && at r c1 <> '+' then lft_ok := false;
                if at r c2 <> '|' && at r c2 <> '+' then rgt_ok := false
              done;
              if !top_ok && !bot_ok && !lft_ok && !rgt_ok then incr count
          done
        done
    done
  done;
  !count
