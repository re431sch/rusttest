# Homework hw2 task3

## Aufgabe

Schreiben Sie eine Funktion die beliebig viele Zahlen von der Kommandozeile einliest und:

- *min*, *max*, *avg* und die *standard deviation* ausgibt
- die Zahlen sortiert ausgibt

Die Daten sollen in-place sortiert werden. Bei der Auswahl des Sortieralgorithmus muss nicht auf Laufzeit geachtet werden.

Benutzen Sie dazu alles bisher zu Rust gelernte, insbesondere:

- Idiomatischen Rust Code
- Fehlerbehandlung
- Module und Crates (z.B. `src/lib.rs`)
- Tests

Benutzen Sie keine externen Crates, auser Ihren eigenen. Zur Vereinfachung gehen Sie in Ihren Berechnungen und Sortierungen von ganzen Zahlen aus. Jedoch kann der Benutzer alle möglichen fehlerhaften Eingaben machen, die Sie ohne Programmabsturz abfangen müssen.

Die Tests Ihrer Library müssen in der Datei `tests/task3.rs` implementiert werden. Unit-Tests von Funktionen in `main.rs` werden direkt in der Datei `main.rs` gepflegt.

Tests zum Aufruf Ihres Programms (`tests/output.bats`) werden nicht benötigt.

#### Dokumentation

Erstellen Sie über Ihre crate und alle Library Funktionen eine aussagekräftige Dokumentation mittels Code Kommentare. Über den Aufruf **cargo doc** muss diese Dokumentation komplett erstellt werden können.
