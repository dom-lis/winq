(ns test1
  (:require [cmn :refer [flush-lines on-read-line]])
  (:import [java.time Instant Duration]))

(def lines (atom []))

(defn -main
  []
  (on-read-line #(swap! lines conj [% (Instant/now)]))
  (loop []
    (flush-lines (map prn @lines))
    (recur)))
