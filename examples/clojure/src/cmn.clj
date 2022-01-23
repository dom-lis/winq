(ns cmn
  (:require [clojure.string :as s]
            [clojure.core.async :as async :refer [chan offer!]]))

(def size-pat #"size:(?<width>\d+),(?<height>\d+)")

(defn parse-size
  [s]
  (if-let [[_ & xs] (re-find size-pat s)]
    (map #(Integer/parseInt %) xs)))

(defn send'
  [x ls]
  (printf "\t%s\n%s\n" x (s/join "\n" ls)))

(defn flush-lines
  [ls]
  (send' "text" ls)
  (println "\tflush"))

(defn flush-line
  [l]
  (flush-lines [l]))

(defn flush-screen
  [ls bs fs ss]
  (send' "text" ls)
  (send' "fg" fs)
  (send' "bg" bs)
  (send' "style" ss)
  (println "\tflush"))

(defn on-read-line
  [f!]
  (async/thread
    (loop []
      (when-let [line (read-line)]
        (f! line)
        (recur)))))
