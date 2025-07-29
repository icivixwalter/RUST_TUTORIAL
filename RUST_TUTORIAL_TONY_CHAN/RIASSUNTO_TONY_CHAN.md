#RIASSUNTO_TONY_CHAN.md
@RAUST@TUTORIAL@TONY@CHAN

## GUIDA MARK DOWN
   si trova qui: https://learnxinyminutes.com/it/markdown/

   esempio i titoli:

            # Questo è un <h1>
            ## Questo è un <h2>
            ### Questo è un <h3>
            #### Questo è un <h4>
            ##### Questo è un <h5>
            ###### Questo è un <h6>1

### Guida completa per sviluppatori e principianti @giacom\o
Libro cha ha acquistato giacomo.

### Introduzione
Sviluppato inizialmente da  Graydon Hoare e sponsorizzato  
da Mozzilla nato con l'obiettivo di combinare prestazioni elevate e  
controllo della memoria di C e C++ orientato alla prevenzione della 
sicurezza e degli errori tipo:  
   data races
   buffer overflows
Sponsorizzato da Mozzilla perche migliorava le prestazioni di Firefox-
Quali paradigmi di programmazione offre:
   programmazione imperativa
   programmazione funzionale
   programmazione orientata agli oggetti

   concetto di ownership   = per la gestione
      sicura della memoria e l'assenza di errore di concorenza.
   Cargo = il suo sistema di gestione dei pacchetti e build toll valido per sistema embeddet e motori di gioco.

   Sintassi familiare con C e C++

#### Breve storia
Inizia del 2006 quando Graydon Hoare creo nel tempo libero questo linguaggio cercando di
risolvere i problemi di sicurezza di C e C++ progettato per offrire prestazioni di basso livello di C e C++ 
senza i problemi di memoria di

   buffer overflows

   data races        = In ambito informatico, una data race (o corsa ai dati) è una condizione che si verifica 
   quando più thread o processi accedono contemporaneamente a una risorsa condivisa (come una variabile), 
   e almeno uno di questi accessi è una scrittura. Questo può portare a risultati inattesi, imprevedibili e, 
   spesso, errati, poiché l'ordine in cui i thread accedono alla risorsa non è controllato.

Nel 2009 fu sponsorizzato da mozzilla per miglioare le prestazione di Firefox e nel 2010 fu creato il primo 
compilatore rustc scritto interamente in Rust.

Innovativo è stato il sistema di "ownership = proprieta"

Nel **2015** ragiunse la prima versione stabile 1.0 e con esso fu utilizzato per il progetto  del nuovo

   motore di rendering

   Un motore di **rendering**

     è un componente software o hardware che trasforma dati
     codificati, come il codice HTML di una pagina web o i dati di
     un modello 3D, in una rappresentazione visiva, che può essere
     visualizzata su uno schermo. In altre parole, è il responsabile di prendere le istruzioni e trasformarle in immagini o video
     che possiamo vedere.
Nel 2017 vennero introdotte nuove funzionalita

   non-lexical lifetimes   = semplificarono la gestione della memoria
   async/await             = resero la programmazione asincrona piu accessibile e potente.


Nel 2021 con la versione 1.50 rusto miglioro il suo ecosistema ed ha
introdotto ottimizzazione del compilatore e nuove libreria standard.

#### Lo standard e le ultime versioni Lo standard viene mantenuta dalla
comunita attiva Rust Fundation con un modello di rilascio stabile e  
organizzato con versioni stabili ogni 6 settimane e con impegno alla 
retro compatibilita.

Le innovazioni ultime sono:

   - miglioramento della programmazione asincrona
   - trait migliorati per il polimorfismo
   - Cargo ha semplifato la gestione dei pacchetti e dipendenze;
   - vers. 1.81 migliorato la funzione estena con extern C che
      prima creava panico non gestito oggi blocca il compilatore;
   - i metodi ambigui sono stati migliorati con la nuova versione
         i metodi sono piu prevedibili

### Rust pro e contro

##### Tra i punti di forza la sicurezza di gestione e previene gli errori di

      - null pointer dreference
      - buffer overflows
      - la concorrenza senza la garbace colletion
   in questo modo è un liguaggio adatto per il software critico dove la sicurezza e l'affidambilita sono fondamentali;
   motoridi gioco e sofware di rete.

   La memoria viene gestia in modo sicuro con
      - ownership
         ogni valore stringa o numero ha un proprietario alla volta
         fino all'uscita dello scope dove la memoria viene deallocata
      - borrow checker
            Ruolo principale del borrow checker

               Garantire la sicurezza della memoria: Rust utilizza un
               sistema di ownership che stabilisce chi possiede un
               dato in memoria e per quanto tempo. Il borrow checker
               verifica che le regole di ownership siano rispettate:
               Ogni dato ha un proprietario. Può esistere solo un
               proprietario alla volta. Il dato viene deallocato
               automaticamente quando il proprietario esce dallo scope.

               Gestire il borrowing: Borrowing consente di accedere ai dati senza trasferire la proprietà. Questo può avvenire
               in due modi:
                     - Mutabile: un solo riferimento mutabile  alla volta.
                     - Immutabile: multipli riferimenti immutabili
                     sono consentiti, ma non contemporaneamente a
                     riferimenti  mutabili.

               Il borrow checker garantisce che questi riferimenti non entrino in conflitto tra loro.

               Prevenire data race: Una data race si verifica quando:
               Due o più thread accedono simultaneamente alla stessa risorsa. Almeno uno di essi modifica la risorsa.
               Non ci sono meccanismi di sincronizzazione per gestire l’accesso. Il borrow checker impedisce questi 
               scenari, obbligando l'accesso sicuro alle risorse condivise.

               Impedire l'uso di dati non validi: Il borrow checker garantisce che i dati non vengano utilizzati dopo
               essere stati spostati, invalidati o rilasciati.
         - Cargo, ecosistema che semplifica  la gestione dei pacchetti
               e tooll, questo è uno dei punti forza.
         - Rust supporta i paradigmi (modelli o insieme di teorie
               che la comunita scientifica utilizza per la propria
               ricerca pratica e quindi il cambio di paradigma o
               rivoluzione scientifica avviene quando un modello
               sostituisce un'altro) e i tre paradigmi sono:
                  - imperativa;
                  - funzionale;
                  - concorrente;
               rendendo rust flessibile adatto a diversi progetti e
               permettendo agli sviluppatore di sceglire l'approccio
               piu adatto.

               Esempi:

               1. ✅ Paradigma Imperativo
                  Nel paradigma imperativo descrivi passo dopo passo cosa deve fare il computer.

                  🔧 Esempio: Calcolo della somma di numeri
                  rust
                  Copia
                  Modifica

                        fn main() {
                            let mut sum = 0;
                            for i in 1..=5 {
                                sum += i;
                            }
                            println!("La somma è: {}", sum);
                        }
                  Spiegazione:

                        let mut sum = 0; definisce una variabile mutabile.

                        Il ciclo for è tipico dello stile imperativo.

                        Modifichiamo lo stato (variabile sum) passo per passo.
               2. ✅ Paradigma Funzionale

                  Rust supporta caratteristiche funzionali: funzioni pure, closure, iteratori, immutabilità, ecc.

                  🔧 Esempio: Somma con stile funzionale usando iteratori

                        fn main() {
                               let sum: i32 = (1..=5).sum();
                               println!("La somma è: {}", sum);
                           }

                     Oppure con map e filter:
                        fn main() {
                            let squares: Vec<i32> = (1..=5)
                                .map(|x| x * x)
                                .filter(|x| x % 2 == 0)
                                .collect();

                            println!("Quadrati pari: {:?}", squares);
                        }

                     Spiegazione:

                        map, filter, collect sono tipici della programmazione funzionale.

                        Non c’è mutabilità o gestione manuale dello stato.
               3. ✅ Paradigma Concorrente

                  Rust ha un eccellente supporto alla programmazione concorrente e parallela, grazie alla ownership e al
                  compilatore che garantisce la sicurezza dei thread.

                  🔧 Esempio: Esecuzione concorrente con std::thread
                        use std::thread;

                           fn main() {
                               let handle = thread::spawn(|| {
                                   for i in 1..=5 {
                                       println!("Thread secondario: {}", i);
                                   }
                               });

                               for i in 1..=5 {
                                   println!("Thread principale: {}", i);
                               }

                               handle.join().unwrap(); // Attende la fine del thread secondario
                           }

                   Spiegazione:

                     thread::spawn crea un nuovo thread.

                     join() sincronizza e attende il completamento.

                     Nessun uso esplicito di unsafe, tutto è gestito in sicurezza dal compilatore.

                  Altri strumenti per la concorrenza:
                     std::sync::mpsc per canali (message passing)

                     tokio per programmazione asincrona

                     rayon per parallelismo con iteratori paralleli

            ✅ Conclusione

               Paradigma   | Caratteristiche in Rust                        | Esempio
               ------------|------------------------------------------------|-------------------------------
               Imperativo  | Stato mutabile, cicli, controllo esplicito     | for, let mut, if, while
               Funzionale  | Closure, immutabilità, iteratori               | map, filter, fold, sum()
               Concorrente | Sicurezza a compile-time, gestione thread-safe | thread::spawn, join, channel

   Gli svantaggi
   - curva di apprendimento difficile
   - sintassi complessa e gestione della ownership complicata;
   - linguaggio eccessivo per progetti semplici;
   - tempo di compilazione lungo;

#### Strumenti e tools di lavoro
   
      puo essere utilizzato un semplice codice di testo ma quando i progetti
      sono complesso occorre un IDE; inoltre non utilizza il modello 
   
      JIT = just in time come C# ma il modello:
   
         AOT = ahead-of-time = ossia il codice viene tradotto in binari
         eseguibili  nella compilazione prima della sua esecuzione, garantendo
         sicurezza e ottimizzazione delle prestazione.

#### LINGUAGGIO MACCHINA
   La programmazione si distingue in BASSO LIVELLO = come assembly che costringe i  
   programmatori a lavorare molto vicino all'hardware ed a controllare la memoria.  
   
   Rust è un linguaggio ad ALTO LIVELLO  progettato per fornire astrazioni moderne  
   e controllando le risorse come la memoria.
   Essendo un linguaggio AOT = aead-of-time = significa che il compilatore rustc  
   trasforma in binario eseguibile il listato del programmatore risultando diverso  
   dai linguaggi Just-In-Time tipo Java o C# dove il linguaggio viene parzialmente  
   interpretato. Inoltre a differenza di C++ o Assembly dove il programmatore deve  
   deallocare la memoria, rust utilizza  il suo sitema di ownership e borrow cheching  
   consentendo sicurezza della memoria.
   Rust è anche cross-compilation = puo essere compilato per diverse piattaforme.  
#### AI INTELLIGENZA ARTIFICIALE
   Possiamo utilizzare Ai per avere un tutor in rust.  

### Debug, Boilerplate code e convenzioni
      - debugging
         Rust utilizza macro dbg!() che stampa su console il valore e la posizione del codice  
         es.   let x = 5; dbg!(x) //stampa il valore 5;  

      - println!()  = consente di stampare variabili in fase di esecuzione ma alcuni tipi devono  
      essere implementati con trait Debug; e puoi anche creare tipi personalizzati da richiamare  
      con #[derive(Debug)]

      - boilreplate code  
         rust riduce la possibilita di costruire codice codice ripetitivo e rindodante con :
            @ i trait = che permetto di generare codice ripetitivo automaticamente  
            @ macro   = tipo vec![] crea un vettore con tutti gli elementi in modo automatico.  
      - zero-cost abstractions  
         puoi utilizzare strutture ad altro livello tipo Result + Option senza costi extra in termini  
         di prestazioni e potento scrivere codice boilerplate per la gestione degli errori e dei valori  
         nulli.


   
   

   




## 1 Le prime basi
         ddd dd
