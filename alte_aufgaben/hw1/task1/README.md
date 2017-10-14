# Homework hw1 task 1


## task
Schreiben Sie ein Programm, welches in einer Funktion

```rust
fn is_leap_year(year: i32) -> bool
```

überprüft, ob das übergebene Jahr ein Schaltjahr ist.

Das trickreiche an der Überprüfung ist, dass folgende Bedinungen für das Jahr gelten müssen:

```plain
on every year that is evenly divisible by 4
  except every year that is evenly divisible by 100
    unless the year is also evenly divisible by 400
```

Zum Beispiel ist 1997 kein Schaltjahr, aber 1996. 1900 ist kein Schaltjahr aber 2000.

Die *main()* Funktion soll in einer Schleife die Zahlen aus folgendem Array überprüfen:
```test
let years = [1900, 2000, 1997, 1004, 1833, 2016];
```

>Tip: Mit years.len() bekommen Sie die Länge des Arrays geliefert, falls Sie beim Arbeiten mittels Iteratoren Schwierigkeiten bekommen.

Gefundene Schaltjahre werden wieder mit einem '*' zusätzlich ausgegeben. Die Ausgabe des Programms:

```text
1900
2000*
1997
1004*
1833
2016*
```

Nutzen Sie die schon vorgegebene Datei `main.rs` sowie das `tests/` Verzeichnis mit der Datei `output.bats`.

Verwenden Sie keine Funktionen aus Bibliotheken dafür, sondern implementieren Sie die Funktion selbst.
