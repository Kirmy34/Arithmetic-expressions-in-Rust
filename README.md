# Arithmetische Ausdrücke in Rust

## Team Members
- Jonas Heck, hejo1042,
- Michael Kirmizakis, kimi1020,
- Mario Occhinegro, ocma1013,

## Nutzung
In diesem Projekt ist es möglich beliebig viele eigene Ausdrücke einzugeben welche dann zunächst geparsed, typ-gecheckt und ausgewertet werden.
Es werden nur natürliche Zahlen (`0-9`) sowie Boolische Werte (`true / false`) unterstüzt. Verknüpft können diese Werte durch folgende Operationen:
`Plus (+)`, `Mal (*)`, `Or (| oder ||)` und `And (& oder &&)`.
Um Expression einzulesen reicht es `cargo run` auszuführen. Zusätzlich werden hierbei zunächst einige Beispiel Expressions
angezeigt und ausgewertet.

Um sich die Tests anzuschauen führt man `cargo test` aus.

## Datatype 
Einfaches Enum zur Bestimmung der Art des Datentyps. 
Wir haben Integer oder boolsche Werte
```rs
pub enum DataTypes {
    TInt,
    TBool,
}
```
## Expression Enum
Einfaches Enum zur Bestimmung und Strukturierung von Ausdrücken. 

```rs
pub enum Expression {
    One,
    Zero,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    ETrue,
    EFalse,
    Plus(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    EOr(Box<Expression>, Box<Expression>),
    EAnd(Box<Expression>, Box<Expression>),
}
```
## Evaluieren von Expressions
Das Expression Enum beschreibt alle möglichen Ausdrücke, die wir verarbeiten
können. Eine Variante jeweils für die Zahlen von 1 bis 9. Sowie jeweils eine Variante
für true und false. Für die Operatoren Plus, Mult, Or und And gibt auch 
jeweils eine Variante. Interessant ist hier, dass diese hier wieder Unterexpressionen
verwalten. Ein mal für Rechts und ein mal für Links.

### Verwendung des Pattern-Matching zur Evaluation von Ausdrücken
Um eine Expression zu evaluieren, wird das Pattern-Matching von Rust verwendet.
Bezüglich des Rückgabetyps, ist zu beachten, dass aufgrund der Tatsache,
dass in den Ausdrücken sowohl logische Ausdrücke als auch Ausdrücke mit 
Zahlen vorkommen, hier eine Unterscheidung benötigt wird. Weiterhin besteht die
Möglichkeit, dass ein Ausdruck gar nicht evaluiert werden kann, weil eventuell 
eine nicht auflösbare Vermischung von logischen und mathematischen Ausdrücken
vorkommt. Manche dieser vermischten Ausdrücke können jedoch aufgrund der
Short-Circuit-Evaluation trotzdem ausgewertet werden.

### Rückgabetyp und Unterscheidung von Zahlen und logischen Werten
Um nun den Rückgabewert der Evaluate-Funktion zu definieren, wir `Option <Result <i32, bool >> `gewählt, da mittels der Option, angegeben werden kann, ob überhaupt ein Wert zurückgegeben wird `(Some)`, oder ob die Auswertung gescheitert ist und kein Wert zurückgegeben werden kann (None). Um
zwischen einer Zahl und einem logischen Wert zu unterscheiden, wird
`Result <i32, bool >` verwendet, hierdurch, kann mittels `Ok(number)` eine Zahl
und mittels `Err(logic)` ein logischer Wert zurückgegeben werden, dies wird für
das Pattern-Matching benötigt, da es hierdurch einfacher wird, den Typ einer
Evaluation zu matchen und zu erkennen.

## Evaluation von verschiedenen Ausdruckstypen

### Evaluation von Zahl-Expressions
Um die eigentliche Evaluation durchzuführen, werden zunächst die Patterns für
Expression angegeben, die nur eine Zahl enthalten. Das bedeutet, es werden
die Expression-Patterns Zero bis Nine im Pattern-Matching angegeben, und die
entsprechende Zahl wird zurückgegeben. Ebenso werden die Patterns für die
booleschen Werte true und false definiert.
```rs
pub fn evaluate(&self) -> Option<Result<i32, bool>> {
    match self {
        Self::Zero => Some(Ok(0)),
        Self::One => Some(Ok(1)),
        Self::Two => Some(Ok(2)),
        Self::Three => Some(Ok(3)),
        Self::Four => Some(Ok(4)),
        Self::Five => Some(Ok(5)),
        Self::Six => Some(Ok(6)),
        Self::Seven => Some(Ok(7)),
        Self::Eight => Some(Ok(8)),
        Self::Nine => Some(Ok(9)),
        //other patterns
    }
}

```
### Evaluation von Plus-Expressions
Als Nächstes folgt das Pattern für eine Plus-Expression. Hier werden zunächst
beide Seiten der Plus-Expression evaluiert. Anschließend wird ein Pattern-Matching
auf die beiden Ergebnisse angewendet. Wenn beide Ergebnisse Zahlenwerte 
liefern `(Some(Ok(left value)))`, werden die beiden Werte von der linken und rechten
Seite aufaddiert und an die aufrufende Funktion zurückgegeben. Wenn eine der
beiden Seiten keine Zahl zurückliefert, wird dies mittels des don’t care 
Patterns im entsprechenden Fall erkannt, und es wird None zurückgegeben, um
anzuzeigen, dass keine Auswertung der Expression möglich war.
```rs
pub fn evaluate(&self) -> Option<Result<i32, bool>> {
    match self {
        //other patterns
        Self::Plus(left, right) => {
            let left_result = left.evaluate();
            let right_result = right.evaluate();
            match (left_result, right_result) {
                (Some(Ok(left_value)), Some(Ok(right_value))) => Some(Ok(left_value + right_value)),
                (_, _) => None
            }
        },
        //other other patterns
    }
}
```
### Evaluation von Multiplikation-Expressions
Als nächstes erfolgt die Pattern-Prüfung für eine Multiplikations-Expression.
Hier wird zuerst überprüft, ob an der linken Stelle der Expression eine 0 steht.
In diesem Fall kann die weitere Evaluierung übersprungen werden, und direkt
0 zurückgegeben werden , da 0 * immer 0 ergibt. In allen anderen Fällen wird
die linke Seite der Expression evaluiert. Anschließend wird ein Pattern-Matching
auf das Ergebnis dieser Evaluation angewendet. Falls die Evaluierung der linken
Seite der Expression ergibt, dass diese 0 ist, wird direkt 0 zurückgegeben. Wenn
die linke Seite zu einer Zahl ungleich 0 evaluiert wurde, wird auch die rechte Seite
evaluiert. Falls die linke Seite gar nicht evaluiert werden konnte, wird mittels
des don’t care Patterns None zurückgegeben. Nachdem die rechte Seite evaluiert
wurde, wird auch auf diesem Ergebnis ein Pattern-Matching durchgeführt, das
dieselben Fälle wie das Pattern-Matching für die linke Seite abdeckt. Wenn ein
Zahlenwert ungleich 0 ermittelt wurde, wird anders als im Fall für die linke
Seite, die Zahlen der linken und rechten Seite multipliziert und zurückgegeben.
```rs
pub fn evaluate(&self) -> Option<Result<i32, bool>> {
    match self {
        //other patterns
        Self::Mult(left, right) => {
            match left.as_ref(){
                &Expression::Zero => Some(Ok(0)),
                _ => {
                    let left_result = left.evaluate();
                    match left_result {
                        Some(Ok(0)) => Some(Ok(0)), // if left side is zero do not eval right side
                        Some(Ok(left_value)) => {
                            let right_result = right.evaluate();
                            match right_result {
                                Some(Ok(0)) => Some(Ok(0)),
                                Some(Ok(right_value)) => Some(Ok(left_value * right_value)),
                                _ => None
                            }
                        }
                        _ => None,
                    }
                }
            }
        },
        //other other patterns
    }
}
```
### Evaluation von Oder-Expressions
Um eine Oder-Expression auszuwerten, wird zunächst die linke Seite ausgewertet, und dann wird ein Pattern-Matching auf diesem Ergebnis durchgeführt.
Falls die linke Seite nicht ausgewertet werden konnte, wird direkt None zurückgegeben.
Wenn im Rahmen der Auswertung der linken Seite eine Zahl ermittelt wurde,
wird die Auswertung ebenfalls abgebrochen und None zurückgegeben. Wenn die
linke Seite jedoch zu einem logischen Wert ausgewertet werden konnte, wird auf
diesem Wert wiederum ein Pattern-Matching gestartet. Wenn die linke Seite der
Oder-Expression zu true ausgewertet wurde, kann direkt true als Wert für die
Expression zurückgegeben werden, da die rechte Seite das Ergebnis der Auswertung nicht mehr verändern kann. Wenn die linke Seite zu false ausgewertet
wurde, wird auch die rechte Seite ausgewertet, und dann wird auf diesem Ergebnis ebenfalls ein Pattern-Matching durchgeführt. Dabei wird geprüft, ob die
Expression überhaupt evaluiert werden konnte und ob ein boolescher Wert für
die rechte Seite evaluiert wurde. Falls ein boolescher Wert auf der rechten Seite
steht, wird dieser als Wert der Expression zurückgegeben, da der Wert der Expression nun nur noch von diesem abhängt.
```rs
pub fn evaluate(&self) -> Option<Result<i32, bool>> {
    match self {
        //other patterns
        Self::EOr(left, right) => {
            let left_result = left.evaluate();
            match left_result {
                None => None, // Expression could not be evaluated
                Some(Ok(_left_value)) => None, // got an int in logic eval
                Some(Err(left_value)) => {
                    match left_value {
                        true => Some(Err(left_value)), // pass true to caller
                        false => {
                            let right_result = right.evaluate();
                            match right_result {
                                None => None, // Expression could not be evaluated
                                Some(Ok(_right_value)) => None, // got an int in logic eval
                                Some(Err(right_value)) => Some(Err(right_value)), // pass value to caller
                            }
                        }
                    }
                }
            }
        },
        //other other patterns
    }
}
```
### Evaluation von Und-Expressions
Abschließend folgt das Pattern für die Und-Expression. Bei dieser wird ebenfalls
zuerst die linke Seite der Expression ausgewertet, und auf dem Ergebnis dieser
Auswertung wird ein Pattern-Matching durchgeführt. Falls die linke Seite nicht
evaluiert werden konnte, wird None zurückgegeben. Wenn die linke Seite zu
einer Zahl evaluiert wurde, wird die weitere Auswertung abgebrochen und None
zurückgegeben. Nur wenn die linke Seite zu einem booleschen Wert ausgewertet
wurde, wird auf diesem Ergebnis ein weiteres Pattern-Matching durchgeführt.
Dabei wird zunächst geprüft, ob der ermittelte Wert false ist. In diesem Fall
wird die rechte Seite gar nicht ausgewertet, da bei einer Und-Expression sowohl die rechte als auch die linke Seite true sein müssen, um true als Ergebnis
zurückzugeben. Wenn der linke Teil der Und-Expression true ist, wird auch der
rechte Teil ausgewertet und geprüft, ob die Auswertung einen Wahrheitswert
ergibt. Wenn die Auswertung der rechten Seite einen booleschen Wert ergibt,
wird dieser Wert als Ergebnis der Expression zurückgegeben, da der Wert der
Expression nun nur noch von diesem abhängt.
```rs
pub fn evaluate(&self) -> Option<Result<i32, bool>> {
    match self {
        //other patterns
        Self::EAnd(left, right) => {
            let left_result = left.evaluate();
            match left_result {
                None => None, // Expression could not be evaluated
                Some(Ok(_left_value)) => None, // got an int in logic eval
                Some(Err(left_value)) => {
                    match left_value {
                        false => Some(Err(left_value)), // pass false to caller
                        true => {
                            let right_result = right.evaluate();
                            match right_result {
                                None => None, // Expression could not be evaluated
                                Some(Ok(_right_value)) => None, // got an int in logic eval
                                Some(Err(right_value)) => Some(Err(right_value)), // pass value to caller
                            }
                        }
                    }
                }
            }
        }
    }
}
```
## Beispiele für die Evaluation

### Beispiel 1
Als kurzes Beispiel zur Erklärung der Funktionsweise der evaluate Funktion sei die folgende Beispielexpression 0 * (0 || 1) angenommen. Diese wird in Rust folgendermaßen dargestellt: 
```rs
Mult(Box::new(Zero), Box::new(EOr(Box::new(Zero), Box::new(One))))
```
Wenn nun die evaluate Funktion auf dieser Expression aufgerufen wird, wird diese Funktion auf der Mult Expression aufgerufen.

```rs
Self::Mult(left, right) => {
    match left.as_ref(){
    &Expression::Zero => Some(Ok(0)),
    _ => {
        let left_result = left.evaluate();
            match left_result {
                Some(Ok(0)) => Some(Ok(0)), // if left side is zero do not eval right side
                Some(Ok(left_value)) => {
                    let right_result = right.evaluate();
                    match right_result {
                        Some(Ok(0)) => Some(Ok(0)),
                        Some(Ok(right_value)) => Some(Ok(left_value * right_value)),
                        _ => None
                    }
                }
                _ => None,
            }
        }
    }
},
```

Aufgrund des in dieser Funktion verwendeten Pattern Matching ist nur dieser Teil der evaluate Funktion relevant. Direkt nach dem das Mult Pattern erkannt wurde, wird dann geprüft, ob der linke Teil eine 0 ist, in diesem simplen Beispiel ist das tatsächlich der Fall, weshalb die evaluate Funktion direkt 0 zurückgibt. Eigentlich würde diese Expression sich nicht auswerten lassen, aufgrund der short circuit evaluation, welche zunächst den linken Teil auf eine 0 prüft, wird die Expression jedoch trotzdem ausgewertet und 0 als Ergebnis zurückgegeben.

### Beispiel 2 
Als ein zweites etwas anspruchsvolleres Beispiel wird nun die Expression 4 * (2 + 9) betrachtet. Als Rust Code wird diese Expression folgendermaßen dargestellt: 
```rs
Mult(Box::new(Four), Box::new(Plus(Box::new(Two), Box::new(Nine))))
```
Wenn diese nun evaluiert werden soll, wird zunächst die Mult Expression evaluiert. Hierzu kommt, das bereits in Beispiel 1 gezeigte Code Fragment zum Einsatz. Anders als im ersten Beispiel, ist in diesem Beispiel jedoch die linke Seite nicht 0, sondern eine 4. Deswegen muss hier auch noch die rechte Seite evaluiert werden. Beim Blick auf die rechte Seite, zeigt sich, dass es sich hierbei wiederrum um eine Expression handelt, jedoch eine Plus-Expression.

Der Code zur Auswertung einer Plus-Expression sieht folgendermaßen aus:
```rs
Self::Mult(left, right) => {
Self::Plus(left, right) => {
    let left_result = left.evaluate();
    let right_result = right.evaluate();
    match (left_result, right_result) {
        (Some(Ok(left_value)), Some(Ok(right_value))) => Some(Ok(left_value + right_value)),
        (_, _) => None
    }
},
```
Hier werden direkt die linke und die rechte Seite ausgewertet, da nun die rechte und die linke Seite Zahlen sind, werden diese mittels Pattern Matching erkannt und als Zahl zurückgegeben.

```rs
Self::Two => Some(Ok(2)),
Self::Nine => Some(Ok(9)),
```

Danach wird geprüft, ob die linke und rechte Seite zu Zahlen ausgewertet werden konnten, was hier der Fall ist. Um die Evaluierung der Plus Expression abzuschließen, werden die Zahlen der ausgewertete linken und rechten Seite addiert und dann als Zahl zurückgegeben.

Nun liegen die linke und rechte Seite der Mult Expression ausgewertet vor. Nun wird hier im noch geprüft, ob beide Seiten eine Zahl sind, falls dies der Fall ist, werden die beiden Zahlen multipliziert und das Ergebnis als Zahl zurückgegeben. Damit ist die Evaluierung der Expression abgeschlossen und das Ergebnis der Auswertung kann an die aufrufende Funktion zurückgegeben werden.

## Parsen von Expressions
Die Funktion, die für das Parsen von Expression zuständig ist, funktioniert wie folgt.
Sie bekommt einen String und ein Präzedenzlevel, um sich in der Rekursion zu
merken was die Präzedenz der Parent-Expression ist. Die Ausgabe ist dann
ein Expression-Enum, das die eingegebene Expression beschreibt. Innerhalb des
Rumpfes bewegen wir uns von links nach rechts durch den übergebenen String.
Jeder Aufruf fängt damit an, dass wir den Charakter auf dem wir uns befinden
parsen. Dies ist immer eine Zahl, ein Boolean oder ein geklammerter Ausdruck.
Diese Expression merken wir uns als linken Teilbaum. Danach betrachten wir,
falls vorhanden, den nächsten Charakter, bei dem es sich in unserer Logik um
einen Operator handeln muss, da wir keine unnären Operatoren unterstützen.
Falls nicht vorhanden, geben wir die linke Seite als Expression zurück und sind
fertig. Für den Fall, dass es sich aber wirklich um einen Operator handelt,
bestimmen wir zu aller erst dessen Präzedenz. Falls die übergeordnete Präzedenz
der Parent Expression höher ist, so brechen wir an dieser Stelle ab und geben den
bis dato errechnetem Baum zurück. Das ist notwendig, da falls die Präzedenz
des übergeordneten Operators größer ist, der momentane Charakter teil der
Parent-Expression ist. Falls für den anderen Fall setzen wir den Operator in die
Wurzel unseres Baums. Jetzt fehlt nur noch die rechte Seite unseres Baumes.
Diese berechnen wir rekursiv auf dem Rest des Iterables und dem momentanen
Präzedenz-Operator. Falls der Unteraufruf den String nicht voll aufbraucht,
springen wir wieder in die While-Schleife der höheren Rekursionsebene. Jetzt
updaten wir den linken Teilbaum mit dem berechneten Ergebnis. Dies tun wir
so lange, bis keine Characters mehr übrig sind
```rs
pub(crate) fn parse_expr<I>(chars: &mut std::iter::Peekable<I>, parent_precedence: usize) -> Option<Expression>
where
    I: Iterator<Item = char>,
{
    let token = parse_token(chars)?;

    let mut lhs = Some(token);

    while let Some(next_char) = chars.peek().copied() {
        let precedence = operator_precedence(next_char);

        if precedence <= parent_precedence {
            break; // Current operator has lower precedence, stop parsing
        }

        chars.next(); // Consume the operator

        let rhs = parse_expr(chars, precedence)?;

        lhs = match next_char {
            '+' => Some(Expression::Plus(Box::new(lhs.unwrap()), Box::new(rhs))),
            '*' => Some(Expression::Mult(Box::new(lhs.unwrap()), Box::new(rhs))),
            '&' => Some(Expression::EAnd(Box::new(lhs.unwrap()), Box::new(rhs))),
            '|' => Some(Expression::EOr(Box::new(lhs.unwrap()), Box::new(rhs))),
            _ => {
                println!("ERROR: Invalid operator: {}", next_char);
                return None;
            }
        };
    }

    lhs
}
```
### Beispiel Parsing
Hier ein kurzes Beispiel für die Parsung von `1*2+3`
Die Funktion wird aufgerufen und der Character Iterator übergeben.
Da immer nur ein Charakter betrachtet wird und wir am Anfang sind ist der zu betrachtende Charakter die `1`. Diesen Parsen für sich via `parse_token`.
Somit erhalten wir die eine Int Expression mit dem Wert One. 
Der Baum den wir uns bis jetzt gebaut haben sieht also wie folgt aus:
- Links: One
- Operator: noch nicht bestimmt
- Rechts: noch nicht bestimmt


Jetzt steigen wir in den While loop, schauen uns den nächsten Operator an und bestimmten seine Präzedenz.
In diesem Fall also `*` mit Präzedenz 2.
Da unser nicht vorhandene Parent die default Präzedenz von 0 hat und somit geringer als 2 ist gehen wir in der Schleife weiter, merken uns den Operator und consumen.

Der nächste Schritt besteht darin die rechte Seite zu bestimmen.
Dies geschieht durch den rekursiven Aufruf auf dem restlichen Char Itterable `2+3` und der Präzedenz von 2.

Im Unteraufruf wird analog wie oben erstmal `2` geparst und als linke Seite des Baums festgelegt.
Der Baum des Unteraufrufs sieht dann so aus.
- Links: Two
- Operator: noch nicht bestimmt
- Rechts: noch nicht bestimmt

Da unser Operator `+` aber eine geringere Präzedenz hat als die von `*` fliegen wir aus dieser heraus und es wird nur linke Teilbaum zurückgegeben. Das bedeutet nämlich, dass das momentane Token zum Operator in der oberen Ebene gehört.

Wieder in der ersten Rekursionsebene angekommen haben wir also nun den Baum:
- Links: One
- Operator: noch nicht bestimmt
- Rechts: Two

Jetzt fehlt also nur noch der Operator zu bestimmen.
Da wir diesen vorhin zwischgespeichert haben, können wir einfach über das Pattern-Matching den Ausdruck bauen und erhalten somit.

- Links: One
- Operator: Mult
- Rechts: Two

Diesen Baum setzen jetzt wiederum als linken Teilbaum.

Da wir aber noch nich fertig sind, kommen wir mit der While-Schleife in die nächste Iterator und betrachten erneut Operator `+`, da diesen noch nicht consumt haben.

Die Präzendent passt hier also wieder, da die nicht vorhandene Parent-Expression jetzt wieder Präzedenz 0 hat.
Deswegen wird wieder der rechte Teilbaum bestimmt und wir erhalten.

- Links: Mult(One, Two)
- Operator noch nicht bestimmt
- Rechts: Three

Zu guter letzt wird nur der momentan betrachtete Operator gematcht, womit der fertige Baum so aussieht:

- Links: Mult(One, Two)
- Operator Plus
- Rechts: Three

Dieser wird als linker Teilbaum gesetzt, da wir aber keine Charakter mehr haben wird er auch einfach zurückgegeben und wir sind fertig.
## Expression Typcheck
Diese Methode prüft den Datentyp eines Ausdrucks und gibt ein Ergebnis
zurück. Wenn der ausdruck eine Zahl zwischen 0 und 9 ist, wird der Daten-
typ TInt zurückgegeben. Wenn der Ausdruck ETrue oder EFalse ist, wird der
Datentyp TBool zurückgegeben. Wenn der Ausdruck eine Addition, Multiplikation, Logische-Oder oder logische Und-Operation ist, werden die Datentypen
der linken und rechten Operanden überprüft. Wenn beide Operanden den richtigen Typ haben, wird TInt bzw. TBool zurückgegeben, andernfalls wird None
zurückgegeben.

```rs
impl Expression {
    pub fn type_check(&self) -> Option<Result<DataTypes, DataTypes>> {
        match self {
            Self::Zero => Some(Ok(TInt)),
            Self::One => Some(Ok(TInt)),
            Self::Two => Some(Ok(TInt)),
            Self::Three => Some(Ok(TInt)),
            Self::Four => Some(Ok(TInt)),
            Self::Five => Some(Ok(TInt)),
            Self::Six => Some(Ok(TInt)),
            Self::Seven => Some(Ok(TInt)),
            Self::Eight => Some(Ok(TInt)),
            Self::Nine => Some(Ok(TInt)),
            Self::ETrue => Some(Err(TBool)),
            Self::EFalse => Some(Err(TBool)),
            Self::Plus(left, right) => {
                let left_result = left.type_check();
                let right_result = right.type_check();
                match (left_result, right_result) {
                    (Some(Ok(_left_value)), Some(Ok(_right_value))) => Some(Ok(TInt)),
                    (_, _) => None
                }
            }
            Self::Mult(left, right) => {
                let left_result = left.type_check();
                let right_result = right.type_check();
                match (left_result, right_result) {
                    (Some(Ok(_left_value)), Some(Ok(_right_value))) => Some(Ok(TInt)),
                    (_, _) => None
                }
            }
            Self::EOr(left, right) => {
                let left_result = left.type_check();
                let right_result = right.type_check();
                match (left_result, right_result) {
                    (Some(Err(_left_value)), Some(Err(_right_value))) => Some(Err(TBool)),
                    (_, _) => None
                }
            }
            Self::EAnd(left, right) => {
                let left_result = left.type_check();
                let right_result = right.type_check();
                match (left_result, right_result) {
                    (Some(Err(_left_value)), Some(Err(_right_value))) => Some(Err(TBool)),
                    (_, _) => None
                }
            }
        }
    }
}
```

### Beispiele Typechecker
#### Beispiel erfolgreiche Bestimmung:
Hier eine kurze Erklärung am Beispiel von  
 `Expression::Plus(
        Box::new(Expression::Two),
        Box::new(Expression::Five)
    )`

Der Ablauf der Typüberprüfung ist wie folgt:
1. Die Plus-Variante wird erreicht, und die type_check()-Funktion wird für die beiden Operanden (Two und Five) aufgerufen.
2. Für beide Operanden wird Some(Ok(TInt)) zurückgegeben, da sie den erwarteten Typ TInt (Integer) haben.
3. Da sowohl der linke als auch der rechte Operand den erwarteten Typ haben, wird Some(Ok(TInt)) als Ergebnis der Plus-Variante zurückgegeben.
4. Schließlich wird das Ergebnis Some(Ok(TInt)) von der type_check()-Funktion des Gesamtausdrucks zurückgegeben.


#### Beispiel kein valider Typ:
Hier ein Beispiel zur Erkennung, falls ein Typ nicht gültig ist. Hier im Fall von  
` Expression::Plus(
        Box::new(Expression::Two),
        Box::new(Expression::ETrue)
    )`

Der Ablauf der Typüberprüfung ist wie folgt:
1. Die Plus-Variante wird erreicht, und die type_check()-Funktion wird für die beiden Operanden (Two und ETrue) aufgerufen.
2. Für den linken Operanden (Two) wird Some(Ok(TInt)) zurückgegeben, da er den erwarteten Typ TInt (Integer) hat.
3. Für den rechten Operanden (ETrue) wird Some(Err(TBool)) zurückgegeben, da er nicht den erwarteten Typ TInt, sondern den unerwarteten Typ TBool (Boolescher Wert) hat.
4. Da der rechte Operand den erwarteten Typ nicht hat, wird None als Ergebnis der Plus-Variante zurückgegeben.
5. Schließlich wird das Ergebnis None von der type_check()-Funktion des Gesamtausdrucks zurückgegeben.


## Vergleich zu Haskell

### Rust
#### Vorteile
- sehr gute Performance
- Fehler zur Laufzeit nahezu ausgeschlossen, wegen der Überprüfung bei der Kompilierung
- Die Fehlerbahandlung ist dank `Result` und `Option` sehr präzise.

#### Nachteile
- Rust hat einen verboseren Syntax, da wir bspw. mit Option und Result arbeiten müssen um das gewollte Verhalten zu implementieren. Darunter leidet die Leserlichkeit des Codes.
- Das Pattern-Matching wird bei großer Schachtelung sehr unübersichtlich.
- Komplexität bei Implementierung, höher aufgrund von Immutabilität und Borrow Checker

### Haskell
#### Vorteile
- Kompakterer Code, vorallem beim Pattern-Matching
- Dank, des Garbage Collectors müssen wir uns beim Programmieren keine Gedanken um Speicher machen.
- Durch Lazy Evaluation werden Ausdrücke erst ausgewertet wenn ihr Wert benötigt wird. Das führt zu höherer Performance bei sehr großen Ausdrücken

#### Nachteile
- Performance schlechter als bei Rust
