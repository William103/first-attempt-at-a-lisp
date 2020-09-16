(define length
  (lambda (l)
    (if (null? l)
        0
        (+ 1 (length (cdr l))))))

(define map
  (lambda (f l)
    (if (null? l)
        ()
        (cons (f (car l)) (map f (cdr l))))))

(define filter
  (lambda (pred l)
    (if (null? l) ()
        (if (pred (car l))
            (cons (car l) (filter pred (cdr l)))
            (filter pred (cdr l))))))

(define even?
  (lambda (n)
    (int (/ n 2))))

(display (length '(1 2 3 4 5)))
(display (map (lambda (x) (+ x 1)) (list 1 2 3)))
(display (filter even? '(0 1 2 3 4 5 6)))
