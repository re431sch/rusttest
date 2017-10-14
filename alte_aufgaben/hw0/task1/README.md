# Homework hw0 task 1


## task1: Hello Rust & Hello Cargo

Ziel von Task 1 ist es, den Umgang mit git, cargo und der Abgabe über einen Pull-Request (PR) zu üben.

### Auschecken Ihres geforkten Repositories im Labor

1. Loggen Sie sich im Laborraum ein.
1. Checken Sie nun Ihr auf Github geforktes Repository aus. Zu Beginn sollten Sie beide an einem Rechner in einem ausgecheckten Repository arbeiten, bis Sie sich mit der Bedienung von git angefreundet haben.

### Cargo und Rust: Hello World

Rust hat ein fantastischen Paketmanager: Cargo. Dieser beherscht alle Automatisierungs- und Dependency-Management-Details für Sie, so dass Sie keine Sorgen mehr darüber machen müssen, wenn Sie ein Projekt erstellen, bauen oder aktualisieren.

1. In Ihrem `hw0` Verzeichnis wurde bereits über cargo das Verzeichnis `task1/` angelegt. Ausserdem wurde in diesem Verzeichnis im Unterverzeichnis `tests/` ein Skript hinterlegt, mit welchem Sie später Ihr Programm testen können.
1. Wenn Sie sich nun in Ihrem Editor die Datei main.rs anschauen, so finden Sie darin bereits das fertige 'Hello World' Programm.
1. Führen Sie nun **cargo build** und/oder **cargo run** aus. Damit kann das Projekt gebaut bzw. ausgeführt werden.
1. Im Root Ihres Repositories ist bereits eine `.gitignore` Datei enthalten. Schauen Sie sich den Inhalt dieser Datei an.
    > **cargo build** erstellt die nötigen Bibliotheken und Binaries im Unterverezeichnis `target/`. Die dafür aus dem Internet geladenen Programmversionen werden in `Cargo.lock` gespeichert. Das `target/` Verzeichnis benötigen wir nicht im git Repository, da dies jederzeit wieder per **cargo build** erstellt werden kann. Mit der `.gitignore` Datei können Sie **git** anweisen, bestimmte Dateien/Verzeichnisse zu ignorieren.

1. Fügen Sie nun über **git add** und **git commit** neue Dateien bzw. geänderte Dateien in Ihr Repository.
1. Als kleine Fingerübung erstellen Sie nun Ihre 1. Rust Funktion. Die Funktion hat folgende Notation:
    ```Rust
    pub fn multiple_hello(name: &str, n: i32)
    ```
    - *name*: Der Name, der hinter Hello, ausgegeben werden soll
    - *n*: Anzahl der Ausgaben

    und gibt mit Hilfe des *println!* Makros 'Hello, \<name\>!' aus.

1. Dokumentieren Sie diese Hilfsfunktion in Englisch (Einzeiler langt aus), um über **cargo doc** die Dokumentation erstellen zu lassen. Damit Ihre Funktion auch in der Dokumentation auftaucht ist sie in obiger Notation als public (*pub*) markiert.
> Hinweis: Bitte bennenen Sie Variablen und Funktionen auf Englisch. Das gilt auch für Ihre Dokumentation der Funktionen bzw. Kommentare im Code.
1. Der Aufruf von **cargo doc** erstellt nun den Ordner `target/doc/`. Darin liegen Ihre Domunetationsdateien sowie die Dokumentation von externen Modulen (crates). Schauen Sie sich Ihre Dokumentation im Browser an. Öffnen Sie dazu die Datei `index.html` im Ordner Ihres crates `task1`.
> Hinweis: Sie sollten sowohl die Crate Bescheribung als auch Ihre Funktion
1. Lassen Sie Ihr Programm zweimal Ferris grüssen. Ferris ist der Name von Rust's 'unofficial crustacean mascot'. Rufen Sie dazu die Funktion *multiple_hello()* in Ihrer main Funktion geeignet auf.

    ```text
    Hello, Ferris!
    Hello, Ferris!
    ```
    > Achten Sie genau auf die Schreibweise Ihrer Ausgabe, also Komma und Gross- und Kleinschreibung. Sonst laufen die automatischen Tests bei Ihrer Abgabe nicht durch.

### Abgabe task1

1. Erstellen Sie den Branch 'hw0-t1'.
1. Lassen Sie alle Tests laufen, siehe dazu 'Abgabe allgemein' in der Datei OVERVIEW.md.
1. Wenn Sie mit Ihrer Bearbeitung zufrieden sind und alle Tests erfolgreich durchlaufen, pushen Sie ihren lokalen branch auf ihren Fork nach github und erstellen Sie auf github einen Pull-Request(PR) gegen das original Repository dafür.
1. Tragen Sie Ihren Tutor als Reviewer ein.
