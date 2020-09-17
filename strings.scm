(define reverse
  ((lambda (helper)
     (lambda (l) (helper l '() helper)))
   (lambda (l acc f)
     (if (null? l)
         acc
         (f (cdr l) (cons (car l) acc) f)))))

(define msg "Here is a string\nIt has a line break")
(display msg)
(newline)
(display "Here it is backwards!")
(display (list->string (reverse (string->list msg))))
(newline)
(display "This is a\ttab")
(display "What a \"cool\" programming language this is")
