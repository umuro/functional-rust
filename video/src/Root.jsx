import React from 'react';
import { Composition } from 'remotion';
import { RustExampleVideo } from './RustExample';

// Default props for studio preview
const defaultProps = {
  title: "Last Element of a List",
  category: "Lists & Pattern Matching",
  difficulty: 1,
  code: `// Find the last element of a list

fn last<T>(list: &[T]) -> Option<&T> {
    list.last()
}

// Pattern matching (functional style)
fn last_pattern<T>(list: &[T]) -> Option<&T> {
    match list {
        [] => None,
        [.., last] => Some(last),
    }
}`,
  output: "last(&[1, 2, 3, 4])  →  Some(4)\nlast(&[])            →  None",
};

function RemotionRoot() {
  return (
    <Composition
      id="RustExample"
      component={RustExampleVideo}
      durationInFrames={450}  // 15 seconds @ 30fps
      fps={30}
      width={1080}
      height={1080}
      defaultProps={defaultProps}
    />
  );
}

export { RemotionRoot };
