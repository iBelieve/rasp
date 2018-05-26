(defun func ()
  (println "Hello, world!"))

(func)
(println "{} {}" func '(println "Hi there"))

(set var 2)

(defun testing (a)
  (let ((b (+ a 1)))
    (defun other ()
      (println "A = {}, B = {}" a b))
    other))

(println "{}" (testing var))
(set f (testing var))
(set var 5)
(f)
