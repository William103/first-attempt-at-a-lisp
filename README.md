This is just a super simple s-expression evaluator written in rust. Still massive WIP, but functional for what it does now. Currently, it supports addition, multiplication, and lambda calculus.

Basically, this is a lambda calculus interpreter with lisp syntax. For example, the expression `(+ 3 2)` evaluates to `5`, and `(lambda (f) ((lambda (g) (g g)) (lambda (g) (f (lambda (x) ((g g) x))))))` is the famous Y combinator.
