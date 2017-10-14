# Homework hw2 task2

## Restructuring

Bisher ist alles in unserm `src/main.rs` Modul, wodurch dieses im Laufe der Zeit immer unübersichtlicher werden würde. Ziel ist es nun, das meiste aus der `main.rs` auszulagern, sodass nur noch die *main()* Funktion in `main.rs` steht.

Ausgelagert wird der Code in eine Bibliothek, genauer gesagt in die Datei `src/lib.rs`.

>Um den Code aus `lib.rs` in `main.rs` verwenden zu können, müssen Sie ihre derzeite 'Create' mit den entsprechenden 'extern' und 'use' Anweisungen in `main.rs`, erreichbar machen.


### Parsen der Config als Methode

Die *parse_arguments()* Methode steht noch in keinem Zusammenhang mit der Struktur, die die Methode mit Daten füllt.

Schreiben Sie einen Konstruktor *new()* für die Datenstruktur *Config* und ersetzten Sie damit die *parse_arguments()* Funktion. Verwenden Sie nun Config geeignet in Ihrer *main()* Funktion.

### Tests

Die Test sind ausgelagert in die Datei `tests/task2.rs`
