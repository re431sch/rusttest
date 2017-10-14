# Homework hw0 task 2

## Vorbereitung task2

1. Legen Sie sich mit **cargo** das neue Projekt `task2` an. Um ein neues Projekt für die Homework zu erstellen, führen Sie folgenden Befehl im `hw0/` Verzeichnis aus:
    **cargo new --bin task2** (das **--bin** erstellt ein Binary Projekt).
1. Cargo erstellt diese Verzeichnisstruktur für Sie:

    ```text
    task2
    ├── Cargo.toml
    └── src
        └── main.rs
    ```

1. Kopieren Sie die entsprechende README.md Datei (`hw0/task2/README.md`) aus dem `htwg-syslab-bsys/bsys-ss17-homework` in das hw0 Verzeichnis. Darin enthalten ist die Aufgabe für task2.
1. Kopieren Sie die Datei `tests/output.bats` in das von Ihnen in *task2* erstellte `tests/` Verzeichnis.
1. Die Bearbeitung dieser Aufgaben soll auf einem anderen Git-Branch geschehen.
Legen Sie dazu einen Branch `hw0-t2` an, von welchem Sie später den PR öffnen.
Wichtig: Stellen Sie sicher, dass Sie wirklich auf einem neuen Branch sind und nicht noch auf `master` oder einem alten `branch`.


## Pull Request aus hw0-t1 in master gemerged
1. Sobald Ihr Pull-Request von `hw0-t1` akzeptiert wurde, aktualisieren Sie Ihr Repository. Ihr Pull-Request ist jetzt im Master vom upstream gemerged. Um nun wiederum Ihren lokalen master zu syncen, können Sie in Ihrem lokalen Repository den `upstream` mit eintragen.
1. Einen möglichen Ablauf dazu finden Sie zusammengefasst [hier][git-pr-wf] im Kapitel "Keeping Your Fork Up To Date"
1. Weitere Informationen finden Sie natürlich auch bei github selbst, z.B. [hier][github-collab].


## task2

Schreiben Sie ein Programm, das für die Zahl 27 alle Schritte des
Collatz-Algorithmus ausgibt. Die Ausgabe soll in der Form `x -> y` geschehen,
wobei `x` ein Zähler ist, welcher den aktuellen Schritt anzeigt, und `y` für
den aktuellen Wert der Eingabezahl steht.

Der Algorithmus für die Zahl 5 wäre also:

```
1 -> 16
2 -> 8
3 -> 4
4 -> 2
5 -> 1
```

Die "Eingabezahl" soll direkt im Quellcode notiert werden, also *nicht*
vom Nutzer eingelesen werden.

Die Aufgabe soll in Cargo Projekt task2 gelöst werden.

## Abgabe
1. Erstellen Sie aus dem Branch hw0-t1 einen Pull-Request und vergessen Sie nicht Ihren Tutor als Reviewer einzutragen.

[git-pr-wf]: https://gist.github.com/Chaser324/ce0505fbed06b947d962#keeping-your-fork-up-to-date
[github-collab]: https://help.github.com/categories/collaborating-with-issues-and-pull-requests/
