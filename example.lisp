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

(defun complex (a (b nil) ...rest (:d 2))
  (println "(complex a {} b {} :d {} ... {})" a b d rest))
(complex 2 "Hi" "Bye" 10 ''or 2 :d 5)


(println "{}" `(echo ,var))


(defmacro debug (var)
  `(println "{} = {}" ',var ,var))

(debug var)
