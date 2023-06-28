
let [n] <- 16;

# A function that computes the fibonacci sequence up to the `n`th value.
define [FIBONACCI]:

  let [a] <- 0;
  let [b] <- 1;
  let [i] <- 0;

  while | integer [i] < integer [n] |:
    tell -> integer [a];
    let [c] <- integer [a] + integer [b];
    set [a] <- integer [b];
    set [b] <- integer [c];
    set [i] <- integer [i] + 1;
  >>>>

  out -> ...;
>>>>


tell -> void [FIBONACCI];
