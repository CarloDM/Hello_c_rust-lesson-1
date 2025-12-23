### LESSON 1 23/12/2025

Complimenti! 
In un solo giorno hai scritto un codice che copre circa il **70% dei concetti fondamentali** che si usano quotidianamente in Rust. Hai toccato con mano la differenza tra "scrivere codice" e "gestire la memoria".

Ecco il riassunto sintetico e strutturato di ciò che hai imparato oggi:

### 1. Gestione delle Variabili e Tipi

* **Immutabilità di default:** Hai imparato che `let` crea variabili blindate. Devi usare `mut` se vuoi cambiarle.
* **Shadowing:** Hai scoperto che puoi dichiarare `let x` più volte. Non stai "aggiornando" la variabile, ne stai creando una nuova che nasconde la vecchia (utile per cambiare tipo al volo, es. da numero a stringa).
* **Tipizzazione Forte:** Hai usato `i128`, `f64`, `bool`. Rust vuole sapere *esattamente* quanto spazio occupare in memoria.
* **Scope:** Hai visto che le variabili create dentro `{ ... }` muoiono quando la parentesi si chiude.

### 2. Funzioni e Flusso

* **Return Implicito:** La cosa più "Rust" di tutte. L'ultima riga senza punto e virgola `;` è il valore restituito.
* **Loop:** Hai visto che `loop` è infinito (serve `break`), mentre `for` è elegantissimo per scorrere gli array (`.iter()`).

### 3. Enums e Pattern Matching (La parte potente)

Qui sei andato oltre il C++.

* **Enum con Dati:** I tuoi `Lunedi(i32)` non sono semplici etichette, ma trasportano dati.
* **Match:** Hai capito che `match` è l'evoluzione dell'`if`. Ti obbliga a gestire *tutti* i casi possibili dell'Enum. Non puoi lasciare nulla "in sospeso" (perfetto per la tua mentalità).

### 4. Ownership e Memoria (Il cuore di Rust)

Hai toccato il concetto più difficile.

* **Stack vs Heap:** Hai notato che i numeri (`i32`) vengono copiati velocemente (Stack), mentre le stringhe complesse (`String`) hanno un "proprietario" unico.
* **Borrowing (`&`):** Hai imparato a "prestare" la variabile (`&miastr`) alla funzione `get_string_length`. Se non avessi messo `&`, la funzione si sarebbe "mangiata" la variabile e non avresti potuto riusarla dopo.

---

### Analisi del tuo codice (Feedback Tecnico)

Il tuo codice è sintatticamente molto pulito.

* **Ottimo:** L'uso di `{:p}` per stampare i puntatori di memoria. È un modo eccellente per vedere cosa succede "sotto il cofano".
* **Da notare:** Hai usato correttamente `String::from` per creare dati nell'Heap (modificabili/complessi) contro le stringhe letterali `"..."` che sono slice statiche (`&str`).

**Sei pronto per il prossimo passo logico?**
Visto che hai capito le `Enum` e le `Struct` (anche se qui hai usato tuple), il prossimo step naturale è creare una tua **Struct** (es. `Utente` con nome, cognome, età) e scriverci sopra dei "metodi" (funzioni che appartengono a quella struct).

Vuoi provare a trasformare le tue tuple `("Carlo", "De Mauro", 36)` in una vera `struct Persona`?