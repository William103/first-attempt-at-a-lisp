(define even
  (lambda (n)
    (int (/ n 2))))

(define >=
  (lambda (a b)
    (not (< a b))))

(define max
  (lambda (a b)
    (if (>= a b)
        a
        b)))

(define collatz-helper
  (lambda (n sum)
    (if (= n 1) sum
        (if (even n)
            (collatz-helper (/ n 2) (+ sum 1))
            (collatz-helper (/ (+ (* 3 n) 1) 2) (+ sum 2))))))

(define collatz
  (lambda (n)
    (collatz-helper n 0)))

(define search
  (lambda (n m)
    (if (= n 100)
        m
        (search (+ n 1) (max (collatz n) m)))))

(display (search 50 0))
