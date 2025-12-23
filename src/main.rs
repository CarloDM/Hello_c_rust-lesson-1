// GLOBAL
//  ENUM
enum SettimanaGiorni {
    Lunedi(i32),
    Martedi,
    Mercoledi(i32),
    // ...
}


fn main() {
    // commenti println! è una macro riconoscibile con ! alla fine
    println!("Hello, world!!!, ciao mondo!!!----------------------------------------------------------------<//");

    // dichiarazione variabili
    let x = 32; //in questo caso assegna automaticamente il tipo i32

    // print! non va a capo
    print!("variabile x: {} ; ", x); 
    // la prima stringa in ordine ogni {} sostituito valori dopo virgola
    println!("più {} fa: {}.", "uno",  x + 1); 
    println!("moltiplichiamo {} per {} che fa: {}", x, 4 , x * 4);

    // dichiarazione variabili mutabili e non mutabili
    let mut mia_var_1 = 6;
    let mia_var_2 = 6;
    //  se non leggo in modo utile le variabili il compilatore mi dara un warning
    mia_var_1 = mia_var_1 + 3;
    println!("mia variabile modificabile : {}", mia_var_1);
    println!("mia variabile non modificabile : {}", mia_var_2);

    //  ridichiarare la variabile cosi non ho problemi di mutabilità, shadowing
    let  mia_var_3 = 80;
    println!("mia variabile 3  : {}", mia_var_3);
    let mia_var_3 = 50;
    println!("mia variabile 3 ridefinita : {}", mia_var_3);
    let mia_var_3 = "!!sono tipo stringa!!";
    println!("mia variabile 3 ridefinita mutata a stringa : {}", mia_var_3);

    // SCOPE DIFFERENTI creano una variante diversa della variabile
        {
            let mia_var_3 = 9000;
            println!("mia variabile 3 ridefinita in blocco : {}", mia_var_3);
        }
    println!("mia variabile 3 fuori dal blocco : {}", mia_var_3);

    // costanti. il tipo è obbligatorio va dichiarato altrimenti errore di compilazione
    const MIA_COSTANTE_1: u32 = 100; // dichiarata inline , dichiarata tutta su una riga per forza; 
    println!("mia costante 1 : {}", MIA_COSTANTE_1);
    
    // funzioni
    let base = 3.3;
    let altezza = 5.5;
    let area    = area_rettangolo(base, altezza);
    println!("area rettangolo base {}m altezza {}m è: {:.1}m2", base, altezza, area);

    // data types
    // scalar types: integer, floating-point, boolean, char, string
    // integer (signed, unsigned) [i8, i16, i32, i64, i128, isize | u8, u16, u32, u64, u128, usize]
    let mia_var_4: i128 = 999999000000000000000; // tipo i128 intero 128 bit
    println!("mia variabile 4 : {}", mia_var_4);
    let mia_var_4_1 = 3.14159; // tipo floating point f64
    println!("mia variabile 4_1 float : {}", mia_var_4_1);
    let mia_var_5 = true; // tipo booleano true/false
    println!("mia variabile 5 booleana : {}", mia_var_5);
    let mia_var_6 = "sono una stringa"; // tipo string &str doppi apici
    println!("mia variabile 6 string : {}", mia_var_6);
    let mia_var_6_1 = 'G'; // tipo char , singolo carattere singoli apici
    println!("mia variabile 6 char , (lettera singola) : {}", mia_var_6_1);

    // compound types: tuples, arrays
    // tuples
    let mia_tupla_1 : (&str, &str , i32) = ("Carlo","De Mauro", 36);
    println!("Sono una tupla dal nome: {} {}. e ho: {} anni.", mia_tupla_1.0, mia_tupla_1.1, mia_tupla_1.2);
    let (nome, cognome, eta) = mia_tupla_1;
    println!("Sono una tupla destrutturata con nome: {} {}. età: {} anni.", nome, cognome, eta);

    // arrays no types diversi, e lunghezza fissa
    let mia_array_1 = [2,3,4,5,6,7,9,128];
    println!("array indice 3 ha valore: {}", mia_array_1[3]);// classico indice di array parte da 0 
    let mia_array_2 = [12 ; 5]; // array di 5 elementi tutti inizializzati a 12
    println!("array 2 indice 4 ha valore: {}", mia_array_2[4]);
    let mia_array_string_1 = ["Gianni", "Marco", "Fabiola", "Pippo","Alberto"]; // array di 5 elementi tutti inizializzati a 12
    println!("array 2 indice 4 ha valore: {}", mia_array_2[4]);

    // control flow , operatori condizionali if / else, match, enum
    let mio_risultato_1 = is_eveni32( 4);
    println!("il numero 4 è pari? : {}", mio_risultato_1);
    let mio_risultato_2 = is_eveni32( mia_var_1);
    println!("la variabile mia_var_1: {} è pari? : {}", mia_var_1, mio_risultato_2);


    //  ENUM interno bypassed
    // enum SettimanaGiorni {
    //     Lunedi(i32),
    //     Martedi,
    //     Mercoledi(i32),
    //     // ...
    // }

    let giorno_1  = SettimanaGiorni::Lunedi(22); 
    let giorno_2  = SettimanaGiorni::Martedi; 
    let giorno_3  = SettimanaGiorni::Mercoledi(24); 

    // ENUM - PATTER METCHING -// controllo di un valore specifico di un enum va messo let davanti, 
    { 
        if  let SettimanaGiorni::Lunedi(giorn)    = giorno_1 {
            println!("PATTER METCHING:è lunedi oggi! giorno indicato: {}", giorn);
        }else 
    
        if  let SettimanaGiorni::Martedi   = giorno_1 {
            println!("PATTER METCHING:è martedi oggi oggi!");
        }else 
    
        if  let SettimanaGiorni::Mercoledi(giorn) = giorno_1 {
            println!("PATTER METCHING:è mercoledì oggi oggi! giorno indicato: {}", giorn);
        }
    }
    //  ENUM - FUNZIONE DI PATTERN METCHING - nota: una volta passata giorno_[n] la stessa non risulta riutilizzabile
    get_giorni(giorno_3);
    get_giorni(giorno_2); 
    get_giorni_match(giorno_1);
    // ricrea variabili
    let giorno_1  = SettimanaGiorni::Lunedi(22); 
    let giorno_2  = SettimanaGiorni::Martedi; 
    let giorno_3  = SettimanaGiorni::Mercoledi(24);
    // rilancia funzioni
    get_giorni_match(giorno_1);
    get_giorni_match(giorno_2);
    get_giorni_match(giorno_3);

    // ora...LOOP 3 tipi    
    // LOOP
    let mut count_1 = 0;
    loop{ //attenzione se non c e condizione d uscita ciclo infinito
        print!("cnt:{}-",count_1);
        count_1 += 1;
        if count_1 == 10 {
            println!("fine loop {}", count_1);
            break; // serve brack
        }
    }
    println!("count_1 arrivo {}", count_1);

    // WHILE 
    while count_1 > 0  {
        print!("cntW:{}-",count_1);
        count_1 -= 1;
    }
    println!("count_1 con while d arrivo {}", count_1);

    // FOR 
    for namefor1 in mia_array_string_1.iter() {
        println!("Nome {}", namefor1)
    }



    println!("Fine programma -------------------------------------------------------------------------------<//");
    }

// FUNZIONI ---

fn get_giorni(giorno:       SettimanaGiorni ){ // cosi enum dev esser dichiarato globalmente

    if  let SettimanaGiorni::Lunedi(giorn)    = giorno {
        println!("PATTER METCHING:è lunedi oggi! giorno indicato: {}", giorn);
    }else 

    if  let SettimanaGiorni::Martedi   = giorno {
        println!("PATTER METCHING:è martedi oggi!");
    }else 

    if  let SettimanaGiorni::Mercoledi(giorn) = giorno {
        println!("PATTER METCHING:è mercoledì oggi! giorno indicato: {}", giorn);
    }
}

fn get_giorni_match(giorno: SettimanaGiorni){ // sintassi differente comoda da vbisualizzare

    match giorno {
        SettimanaGiorni::Lunedi(giornoa)    => println!("lune{}", giornoa),
        SettimanaGiorni::Martedi                 => println!("marte"),
        SettimanaGiorni::Mercoledi(giornoa) => println!("merco{}", giornoa),
    }
}

fn is_eveni32(num: i32) -> bool {
    if num % 2 == 0 {
        return true
    } else {
        return false
    }
}

fn area_rettangolo(base: f64, altezza: f64) -> f64 {
    //return implicito senza ; usare: {return base * altezza;}
     base * altezza 
}
