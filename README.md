# What is it?
This is a really basic lisp interpreter. I went into this just as an exercise and out of curiosity, having no idea what I'm doing, just trying to make it work, so I am fully aware that this code is garbage and probably full of sneaky bugs and I am confident that there are much, much better ways of doing things than the way I did (or at least I hope so).

# Compilation
Just your standard `cargo build` or `cargo build --release`. 

# Usage
Upon running the executable you will be dumped into a REPL. It should say `ready> ` and be waiting for input. The input system is extremely dumb, so it won't handle any kind of `readline` shortcuts or anything. It will, however, wait until a top-level statement is complete before evaluating it, i.e. you can have line-breaks in your input. In the REPL you can enter basic lisp code. If you enter `env` you can get a peek behind the curtain and see all the currently defined symbols in ~~an extremely difficult to read~~ a slightly easier to read format. You can also enter `exit` to quit the REPL.

# The language itself
This is, as of right now, an extremely tiny, barely functional subset of scheme lisp. The things you can do are as follows:
 - Floating point numbers.
   * Ex. `1` evaluates to `1.0`
   * Ex. `-4` evaluates to `-4.0`
   * Ex. `5e2` evaluates to `500.0`
 - S-expressions.
   * Ex. `(op arg1 arg2)` evaluates to the result of `op` called on `arg1` and `arg2`.
 - The built in operator `+`, which adds its arguments.
   * Ex. `(+ 1 2)` evaluates to `3.0`
   * Ex. `(+ 1 2 -3)` evaluates to `0.0`
 - The built in operator `-`, which subtracts its arguments. If only one argument is given, it negates it.
   * Ex. `(- 3.2 3)` evaluates to `0.2`
   * Ex. `(- 1)` evaluates to `-1.0`
   * Ex. `(- 3 2 1)` evaluates to `0.0`
 - The built in operator `*`, which multiplies its arguments.
   * Ex. `(* 3 2)` evaluates to `6.0`
   * Ex. `(* 1 2 3 4 5)` evaluates to `120.0`
 - The built in operator `/`, which divides its arguments. If only one argument is given, it returns the reciprocal.
   * Ex. `(/ 3 2)` evaluates to `1.5`
   * Ex. `(/ 3 1 3)` evaluates to `1.0`
   * Ex. `(/ 4 2 2)` evaluates to `1.0`
   * Ex. `(/ 5)` evaluates to `0.5`
 - The built in operator `<`, which returns 1 if its arguments are strictly increasing and 0 otherwise
   * Ex. `(< 3 2)` returns `0.0`
   * Ex. `(< 2 3)` returns `1.0`
   * Ex. `(< 1 2 3)` returns `1.0`
   * Ex. `(< 1 3 2)` returns `0.0`
   * Ex. `(< 3)` returns `1.0`
 - The built in operator `int`, which returns 1 if all of its arguments are integers and 0 otherwise
   * Ex. `(int 3.2)` returns `0.0`
   * Ex. `(int 2 3.0 5)` returns `1.0`
   * Ex. `(int 3.2 3 5)` returns `0.0`
 - The built in operator `if`, which, if the first argument (the condition) evaluates to `0.0` evaluates to the third argument, otherwise the second argument.
   * Ex. `(if 0 1 2)` evaluates to `2`
   * Ex. `(if 1 1 2)` evaluates to `1`
   * Ex. `(if (+ 3 -3) 1 2)` evaluates to `2`
 - Lambda functions. These consist of the keyword `lambda` followed by a list of formal arguments and an expression in terms of those arguments.
   * Ex. `(lambda (x y) (+ x y))` evaluates to a function that sums its arguments
   * Ex. `((lambda (x y) (+ x y)) 3 2)` evaluates to `5`
   * Ex. `(((lambda (x) (lambda (y) (+ x y))) 3) 2)` also evaluates to `5`
 - Defining symbols. This consists of the keyword `define` followed by an identifier (any sequence of non-whitespace, non-parentheses characters that aren't a keyword; this does include sequences starting with numbers, so `5foo` is a valid variable name) followed by an expression to bind to that symbol. **Note**: shadowing isn't a thing. Symbols bound by `define` trump symbols bound by `lambda`.
   * Ex. `(define x 3)` binds the value `3` to the symbol `x`
   * Ex. `(define fact (lambda (n) (if n (* n (fact (+ n -1))) 1)))` binds the factorial function to the symbol `fact`
   * Ex. `(define not (lambda (b) (if b 0 1)))` binds to the symbol `not` the function that inverts a "boolean"
   * Ex. `(define >= (lambda (a b) (not (< a b))))` binds to the symbol `>=` the `>=` function.

# Samples
See the file `collatz.scm` for an example program. This program finds the largest number of steps it takes to reach 1 along the collatz sequence for all numbers less than 100. You can rrun it with `cargo run --release < collatz.scm`. There will be quite a few artifacts from the REPL, but the number it spits out is correct.
