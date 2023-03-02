(def fact (lambda (n) (if (<= n 1) 1 (* n (fact (- n 1))))))
(println (fact 20))
(exit 0)
