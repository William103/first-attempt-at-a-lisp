# What is it?
This is a really basic lisp interpreter. I went into this just as an exercise and out of curiosity, having no idea what I'm doing, just trying to make it work, so I am fully aware that this code is garbage and probably full of sneaky bugs and I am confident that there are much, much better ways of doing things than the way I did (or at least I hope so).

# Compilation
Just your standard `cargo build` or `cargo build --release`. 

# Usage
Upon running the executable you will be dumped into a REPL. It should say `ready> ` and be waiting for input. The input system is extremely dumb, so it won't handle any kind of `readline` shortcuts or anything. It will, however, wait until a top-level statement is complete before evaluating it, i.e. you can have line-breaks in your input. In the REPL you can enter basic lisp code. If you enter `env` you can get a peek behind the curtain and see all the currently defined symbols in ~~an extremely difficult to read~~ a slightly easier to read format. You can also enter `exit` to quit the REPL. You can also run the executable with a filename as an argument and it will evaluate all the top-level expressions in the file.

# The language itself
This is, as of right now, an extremely tiny, barely functional subset of scheme lisp. The things you can do are as follows:
 - Integers
   * Ex. `1` is just `1`
 - Floating point numbers. When an integer and a floating point number interact, the integer is promoted to a floating point value.
   * Ex. `1.0` evaluates to `1.0`
   * Ex. `5e2` evaluates to `500.0`
 - Booleans
   * Ex. `#t` evaluates to true
   * Ex. `#f` evaluates to false
 - The empty list `()`. As you'll see later, you can actually represent this like Scheme with `'()` if you want.
 - Chars, these work similarly to Scheme, except space doesn't quite work yet.
   * Ex. `#\a` evaluates to the character `'a'`
 - Strings, these work exactly how you'd expect.
   * Ex. `"hello"` evaluates to the string `hello`
   * Ex. `"hello\nworld"` has a newline in the middle like expected
   * Ex. `"hello\tworld"` has a tab character in the middle like expected
   * Ex. `"hello\\world"` evaluates to the string `hello\world`
   * Ex. `"\"hello\""` evaluates to the string `"hello"`.
 - S-expressions.
   * Ex. `(op arg1 arg2)` evaluates to the result of `op` called on `arg1` and `arg2`.
 - The built in procedure `display` which prints its arguments and evaluates to the special value `Nil` (same as defines later on).
   * Ex. `(display 3)` prints `3` to the screen and evaluates to Nil
 - The built in procedure `newline` which just prints a newline to the screen and also evaluates to `Nil`.
 - The built in procedure `string->list` which converts a string into a list of characters.
   * Ex. `(string->list "hi")` evaluates to `(#\h . (#\i . ()))`
 - The built in procedure `list->string` which converts a list of characters into a string.
   * Ex. `(list->string '(#\h #\i))` evaluates to `"hi"`.
 - The built in procedure `not` which returns true if and only if its input evaluates to false
   * Ex. `(not #t)` evaluates to false
   * Ex. `(not 3)` evaluates to false
   * Ex. `(not #f)` evaluates to true
 - The built in procedure `cons` which takes two arguments and returns a pair created from them.
   * Ex. `(cons 1 2)` evaluates to `(1 . 2)`
   * Ex. `(cons 1 (cons 2 (cons 3 (cons ()))))` evaluates to `(1 . 2 . 3 . ())`
 - The built in procedure `car` which takes a pair and returns its first element.
   * Ex. `(car (cons 1 2))` evaluates to `1`
 - The built in procedure `cdr` which takes a pair and returns its second element.
   * Ex. `(cdr (cons 1 2))` evaluates to `2`
 - The built in procedure `list` which returns a list of its arguments as nested pairs.
   * Ex. `(list 1 2 3)` evaluates to `(1 . (2 . (3 . ())))`
 - List literals with `'`. Note, this is quite different from Scheme's quote, but I don't quite have symbols implemented yet.
   * Ex. `'(1 2 3 4)` evaluates to `(1 . (2 . (3 . (4 . ()))))`
 - The built in procedure `null?` which returns true if its argument is the empty list.
   * Ex. `(null? ())` evaluates to true
   * Ex. `(null? (cons 1 2))` evaluates to false
 - The built in procedure `+`, which adds its arguments.
   * Ex. `(+ 1 2)` evaluates to `3`
   * Ex. `(+ 1 2 -3)` evaluates to `0`
 - The built in procedure `-`, which subtracts its arguments. If only one argument is given, it negates it.
   * Ex. `(- 3.2 3)` evaluates to `0.2`
   * Ex. `(- 1)` evaluates to `-1`
   * Ex. `(- 3 2 1)` evaluates to `0`
 - The built in procedure `*`, which multiplies its arguments.
   * Ex. `(* 3 2)` evaluates to `6`
   * Ex. `(* 1 2 3 4 5)` evaluates to `120`
 - The built in procedure `/`, which divides its arguments. If only one argument is given, it returns the reciprocal. It always returns a float.
   * Ex. `(/ 3 2)` evaluates to `1.5`
   * Ex. `(/ 3 1 3)` evaluates to `1.0`
   * Ex. `(/ 4 2 2)` evaluates to `1.0`
   * Ex. `(/ 5)` evaluates to `0.5`
 - The built in procedure `<`, which returns true if its arguments are strictly increasing and false otherwise
   * Ex. `(< 3 2)` returns false
   * Ex. `(< 2 3)` returns true
   * Ex. `(< 1 2 3)` returns true
   * Ex. `(< 1 3 2)` returns false
   * Ex. `(< 3)` returns true
 - The built in procedure `=`, which returns true if all of its arguments are the same (converted to same numeric type biased towards floats)
   * Ex. `(= 3)` returns true
   * Ex. `(= 1 1.0 1)` returns true
   * Ex. `(= 0.3 (+ 0.1 0.2))` returns false (gotta love floating point math)
 - The built in procedure `int`, which returns true if all of its arguments are integers and false otherwise (either integers or equal to its floor)
   * Ex. `(int 3.2)` returns false
   * Ex. `(int 2 3.0 5)` returns true
   * Ex. `(int 3.2 3 5)` returns false
 - The built in procedure `if`, which, if the first argument (the condition) evaluates to true evaluates to the second argument, otherwise the third argument.
   * Ex. `(if #f 1 2)` evaluates to `2`
   * Ex. `(if #t 1 2)` evaluates to `1`
   * Ex. `(if (+ 3 -3) 1 2)` evaluates to `2`
 - Lambda functions. These consist of the keyword `lambda` followed by a list of formal arguments and an expression in terms of those arguments.
   * Ex. `(lambda (x y) (+ x y))` evaluates to a function that sums its arguments
   * Ex. `((lambda (x y) (+ x y)) 3 2)` evaluates to `5`
   * Ex. `(((lambda (x) (lambda (y) (+ x y))) 3) 2)` also evaluates to `5`
 - Defining symbols. This consists of the keyword `define` followed by an identifier (any sequence of non-whitespace, non-parentheses characters that aren't a keyword; this does include sequences starting with numbers, so `5foo` is a valid variable name) followed by an expression to bind to that symbol. Shadowing is allowed and behaves like you would expect.
   * Ex. `(define x 3)` binds the value `3` to the symbol `x`
   * Ex. `(define fact (lambda (n) (if n (* n (fact (+ n -1))) 1)))` binds the factorial function to the symbol `fact`
   * Ex. `(define >= (lambda (a b) (not (< a b))))` binds to the symbol `>=` the `>=` function.

# Samples
See the file `collatz.scm` for an example program. This program finds the largest number of steps it takes to reach 1 along the collatz sequence for all numbers less than 100. You can run it with `cargo run --release < collatz.scm`. There is also the file `lists.scm` which gives some examples of how to program with lists.

# TODO
 1. Fix space bug: `#\ ` not being recognized as `' '`.
 2. General maintenance: i.e. cleaning up code, better error handling (actually useful debug info?), document/comment the code.
 3. Look into optimizations, some kind of tail-call optimization.
 4. Add symbols and quote.
 5. Add macros? Use macros to implement standard library in the language.
