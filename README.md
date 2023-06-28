# Whist-lang

 [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

## Description

 An interpreted programming language prototype made for tabletop games.

### Features
- [x] 16-bit integer arithmetic
- [x]  Dice throw
- [x]  Function definition
- [x]  Register management
- [x]  If statement
- [x]  While statement
- [x]  Comments
- [x]  Debugging
- [x]  Terminal arguments
- [ ]  String management
- [ ]  Input handling
- [ ]  Dice and function distribution

## Examples
### Terminal arguments

 ``whist-lang.exe read line "tell -> ...;"``
 
 ``whist-lang.exe read file script.ws`` 
 
### Exploding dice

```
define [EXPLODING_DICE]:
  let [throw] <- 1d6;
  let [sum]   <- integer [throw];

  while | integer [throw] = 6 |:
    set [throw] <- 1d6;
    set [sum]   <- integer [sum] + integer [throw];
  >>>>
  
  out -> integer [sum];
>>>>

tell -> integer [EXPLODING_DICE];
```

### Fibonacci sequence computing

```
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
```
