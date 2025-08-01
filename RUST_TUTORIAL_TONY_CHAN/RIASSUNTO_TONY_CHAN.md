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


### Termini comuni in rust
   * Inferire  
        Il compilatore puo inferire o ricavare i tipi di variabili  
        senza dichiararli im modo esplicito ma sono dedotti automaticamente  
        dalla verbosita.  
   * Monomorfizzazione:  processo in cui il compilatore genera funzioni generiche  
        per ciascun tipo utilizzato ottimizzando il codice ed eliminando l'overhead  
        associato al dispach dinamico.
            spiegazione: (In sintesi, l'overhead del dispatch dinamico è il costo aggiuntivo, .
            in termini di tempo di esecuzione, associato alla determinazione  
            della funzione da chiamare a runtime. Questo costo è assente nel dispatch statico,  
            che risolve la chiamata di funzione a tempo di compilazione.)  
   * Puntatori e puntatori raw:  
      a)  i puntatori in Rust, come & e &mut, sono sicuri perché rispettano le regole  
          del  borrowing  e  dell'ownership;  
      b) i  puntatori  raw,  come  *const  e  *mut,  sono  meno  sicuri  perché  
         permettono l'accesso diretto alla memoria senza le garanzie del borrowing.  
   * Heap e stack:  
      a) heap  =  il primo è usato per dati dinamici con dimensioni variabili, richiedendo però più risorse  
         per gestire la memoria.  
      b) stack = L’altro per dati di dimensioni fisse. Ha un accesso molto più rapido.  
   * Ownership  e  borrowing:  
      a) ownership    =  l'ownership  rappresenta  il  controllo  esclusivo  di  una  variabile  
      b) borrowing    =   prendere in prestito il valore di una variabile temporaneamente, sia in modo  
         _**mutabile**_   che _**immutabile**_, mantenendo la sicurezza della memoria. 
   * Operazioni  atomiche: 
      sono  operazioni  che  vengono  eseguite  come  un'unica  operazione  indivisibile,  
      usate per sincronizzare l'accesso concorrente ai dati tra thread senza incorrere in race conditions.  

   * Struct,  trait  e  crate:  
         - struct       = è  una  struttura  dati  che  raggruppa  variabili;  
         -  trait       = i  trait  definiscono  il comportamento  che  può  essere  
                        implementato  dalle  strutture  
         - crate        =  è  un'unità  di compilazione, come una libreria o un pacchetto.  
   * Null  pointer,  race  condition:
         - null  pointer   =   rappresenta  un  puntatore  che  non  punta  a  nessun  
                           valore  valido
         - race  condition =   si  verifica  quando  più  thread  accedono  a  dati  condivisi  
                  senza la corretta sincronizzazione, causando comportamenti imprevedibili. 
   * Attributi e riferimenti:  
      - gli attributi    = servono a modificare il comportamento del compilatore o del codice.  
      - i riferimenti    =  (&  e  &mut)  sono  puntatori  sicuri  che  permettono  l'accesso  
                  - a  valori  senza  trasferirne l'ownership. 
                  - 
   * Pattern  Matching:  
         consente  di  DESTRUTTURARE, CONFRONTARE  E GESTIRE I DATI in  modo  sicuro  e  conciso.  
         Utilizzando  la parola chiave  match, si  possono  esaminare  diversi casi di un  tipo,  
         come un  enum  o  Option, e gestire tutte le possibilità in modo esaustivo,  
         migliorando la sicurezza del codice. 
   * Polimorfismo ad hoc:  
         La capacita di una funzione o di un metodo di lavorare con diversi tipi di dati ma con  
         comportamenti diversi per ciascuno di loro. Si utilizza il 
         **<span style="font-size: 24px;">trait</span>**  che consentono di definire un insieme di
         metodi che devono essere implementati dai tipi che aderiscono al metodo. Ogni metodo puo
         avere una implementazione specifica per comportamenti differenziati.

   * Pattern
         Sono schemi ripetibili ed efficaci come soluzioni ricorrente ad esemp Option e Result  
         che gestiscono valori opzionali e gli errori in modo sicuro.  
   * Anti Pattern 
         Pratica dannosa che comporta la cattiva manutenzione degli errori. Ad es. utilizzo eccessivo ..
         dei puntatori raw con vulnerabilita della memoria in quanto non hanno il sistema  
         ownership e borrowing.  
### Commentare il codice
   - Commenti su riga      //
   - Commenti multilinea   /**/
   - Commenti per la documentazione  ///  che sono utili per le funzioni le strutture ed i moduli  
         per spiegare come utilizzarli e con l'utility rustdoc genero la documentazione Html. 







## 1 Le prime basi
      Rust utilizza **ownership** che permette di sapere chi possiede una risorsa in ogni 
      momento evitando i problemi dell'accesso concorrente.  
      FFI Foreign Function Interface che permette l'integrazione  con C e C++ quindi permette si  
      di scrivere codice sicuro, ma anche senza sacrificare la flessibilita sia per i progetti  
      nuovi e per quelli gia avviati.  
### Cargo e la sintassi di base
   Strumenti  di gestione dei paccjhetti e il build sistem per rust e gestisce tutto il ciclo  
   di vita dall'inizio dello svilupp fino alla distribuzione.
   * cargo new primo_progetto 
         crea un nuovo progetto con questo schema
            primo_progetto 
            ├── Cargo.toml       = Il file Cargo.toml contiene i metadati del progetto e la lista delle dipendenze.
            └── src 
                └── main.rs  
   * cargo test 
      * test
         Compila il progetto, esegue i test definiti e mostra i risultati. 
            Puoi utilizzare cargo test  che compila il progetto ed esegure i test definiti e mostra  
            risultati.
               * use             = importa i moduli e le funzionalità specifiche da librerie esterne o interne.  
               * main            = punto di ingresso dell'applicazione che non accetta argomenti per default  
                                    ma puo essere utilizzata.
               * mut             = le variabili sono immutabili per default salvo con l'utilizzo di mut.  
                  esempio:  
         
                        fn main() { 
                            let messaggio = "Ciao, mondo!"; // Variabile immutabile 
                            println!("{}", messaggio); // Stampa sulla console 
                         
                            let mut numero = 42; // Variabile mutabile 
                            numero = 43; // È possibile modificarla 
                            println!("Il numero è {}", numero); 
                        } 
      * cicli ripetuti  = if,else, loop, while e for  ..
      * Result  e  Option.
               permette la gestione sicura degli errori; esempio funzione che restituiscono un  
               Result che viene gestito con un match oppure utilizzanto :
                     - unwrap(), .expect(); oppure l'operatore ? per propgare l'errore; esempio
                         
                         fn main() { 
                            let risultato = divisione(10, 2); 
                             
                            match risultato { 
                                Ok(valore) => println!("Risultato: {}", valore), 
                                Err(e) => println!("Errore: {}", e), 
                            } 
                        } 
                         
                        fn divisione(a: i32, b: i32) -> Result<i32, String> { 
                            if b == 0 { 
                                Err(String::from("Divisione per zero")) 
                            } else { 
                                Ok(a / b) 
                            } 
                        } 
      * mod    = moduli 
            rust puo organizzare il codice  in moduli  per suddividere il codice in  parte  
            piu piccolo e gestibili, possono definiti nello stesso file e su file diversi e
            resi pubblici mediante pub.

                
                     mod calcoli { 
                         pub fn somma(a: i32, b: i32) -> i32 { 
                             a + b 
                         } 
                     } 
                      
                     fn main() { 
                         let risultato = calcoli::somma(5, 3); 
                         println!("Il risultato della somma è: {}", risultato); 
                     } 
      * use std::io 
            Partiamo con use std::io per importare le funzionalità di input/output. La funzione 
            main legge un numero dall'utente, lo converte da stringa a intero e lo valuta con 
            una struttura if. Il programma gestisce eventuali errori durante la lettura dell'input   
            o la conversione del numero, utilizzando .expect() per fornire messaggi di errore   
            chiari in caso di fallimento.
      * ownership
            sistema che gestisce la memoria in modo sicuro ogni valore spetta al proprietario 
            e quando esce dall'ambito (scope) viene automaticamente deallocato ed in questo modo
            elimina la garbace collection e previene i bug.
      * borrowing 
            permette di prestare una variabile senza trasferirne la proprieta sia con prestito
            MUTABILE che IMMUTABILE.
               ESEMPIO:
                  fn main() { 
                      let s = String::from("ciao"); 
                      prendi_ownership(s); // Ownership trasferita 
                      // Non possiamo più usare `s` qui 
                   
                      let x = 10; 
                      prendi_in_prestito(&x); // `x` viene preso in prestito, ownership non trasferita 
                      // Possiamo ancora usare `x` qui 
                     } 
 
                  fn prendi_ownership(s: String) { 
                      println!("{}", s); 
                  } 
                   
                  fn prendi_in_prestito(y: &i32) { 
                      println!("{}", y); 
                  } 
      * use
         serve per importare  
            - i moduli                       = interi moduli o sottosezioni di essi;
            - le funzioni, strutture e tipi  = specifici elementi di un modulo  
            - Enum e varianti                = tipi di enum e loro varianti
            - Elementi di un modulo standard o di librerie esterne 
                  puoi accedere alle funzionalita delle librerie standard di terze parti.
                  ed un esempio la libreria st ricca di moduli.
            - Vedi esempio con l'utilizzo di std::io  che gestisce l'imputo e l'output  
                  della tastiera e lo schermo e fornisce gli strumenti per la gestione 
                  dei flussi stream i/o e gestione errori.
              Vedi In io::stdin  legge  una  linea  di  input  dall'utente  
                     
            - Vedi  std::fs: fornisce funzionalità per lavorare con il file system,  
            - 
            - Vedi  std::collection: fornisce strutture dati utili come HashMap, Vec, BTreeMap  
                  strutture  dati  fondamentali  per  organizzare  e  gestire  le  collezioni  in  
                  modo efficiente:  
                  
            - Vedi  std::thread: gestisce la concorrenza con i thread
            - Vedi  std::async : per la sincronizzazione dei thread
            - Vedi  std::time  : per gestire il tempo e le durate temporali.
            - Vedi  std::env   : interagisco con l'ambiente di sistema come le variabili 
                              - di ambiente
            - 
                  vedi i progetti  :
                  progetto ----> @std@io           = gestisce input output
                  progetto ----> @std@in           = in attesa di leggere linea input
                  progetto ----> @std@fs           = file system
                  progetto ----> @std@fcollection  = gestione strutture dati
                  progetto ----> @std@thread       = migliora le prestazioni con i thread
                  progetto ----> @std@async.04?    = sincronizzazione dei thread con Mutex e Arc
                  progetto ----> @std@time.05?     = misura la durata delle operazioni
                  progetto ----> @std@env.06?      = misura la durata delle operazioni



         use strumento performante
            La  dichiarazione  use  è  uno  strumento  performante  che  semplifica  l'accesso  alle 
            funzionalità  dei  moduli.  Che  si  tratti  di  leggere  input,  lavorare  con  file,  utilizzare 
            strutture  dati  avanzate  o  gestire  la  concorrenza,  Rust  offre  una  vasta  gamma  di 
            moduli standard accessibili tramite questa istruzione.
            Ciò permette di mantenere il codice organizzato e pulito, evitando ripetizioni inutili. 

            Questo è l'elenco degli strumenti  potenti  per  gestire  I/O,  dati,  concorrenza,  
            sincronizzazione,  e temporizzazione:
            +-------------------+-------------------------------+-------------------------------------------------------------+
            | Modulo            | Classi / Funzioni             | Descrizione                                                 |
            +-------------------+-------------------------------+-------------------------------------------------------------+
            | std::io           | stdin, stdout, Read, Write    | Gestisce l'I/O; lettura e scrittura da/verso flussi.       |
            | std::fs           | File, read_to_string, write   | Interazione con il file system; leggere/scrivere file.      |
            | std::collections  | HashMap, Vec, BTreeMap        | Strutture dati come mappe hash, vettori e alberi.           |
            | std::thread       | spawn, JoinHandle             | Concorrenza tramite thread per esecuzione parallela.        |
            | std::sync         | Mutex, Arc, RwLock            | Sincronizzazione e accesso sicuro a risorse condivise.      |
            | std::time         | Duration, Instant             | Tempo e durate temporali; utile per timeout e misurazioni.  |
            | std::env          | args, var, set_var            | Interazione con ambiente di sistema (argomenti/env).        |
            | std::net          | TcpListener, TcpStream,       | Funzionalità di rete per TCP e UDP.                         |
            |                   | UdpSocket                     |                                                             |
            | std::process      | Command, exit, Child          | Esecuzione di processi esterni e comandi di sistema.        |
            | std::cmp          | min, max, Ordering            | Funzioni e tipi per confronto e ordinamento.                |
            | std::option       | Option                        | Rappresenta un valore opzionale (presente o assente).       |
            | std::result       | Result                        | Gestione di successo o errore nelle operazioni.             |
            +-------------------+-------------------------------+-------------------------------------------------------------+


       Rust linguaggio sicuro 
         è progettato per il codice sicuro privo di errori nella gestione della memoria come ad
         esempio: dangling pointer
            (Un dangling pointer (in italiano: puntatore pendente o appeso) 
            è un puntatore che fa riferimento a una zona di memoria non 
            più valida o deallocata. Usarlo può causare crash, 
            comportamento indefinito o vulnerabilità di sicurezza.)

               es.

               int* p;
                  {
                      int x = 42;
                      p = &x; // p punta a x
                  } // x non esiste più: p ora è dangling!
                  printf("%d", *p); // ERRORE: accesso a memoria invalida

            🔥 Conclusione
               Un dangling pointer è un pericolo nei linguaggi che non gestiscono la memoria in modo sicuro.
               In Rust, non può succedere a meno che tu usi unsafe.
               Il sistema di ownership + lifetimes lo impedisce a compile time, proteggendo il tuo programma.
         Quando si tratta di mutuo accesso ai dati in ambienti concorrenti, Rust introduce 
         un meccanismo di borrowing e reference counting che garantisce che due parti del 
         codice non possano accedere mutualmente e simultaneamente ai dati in modo che 
         possa  causare  conflitti
         Previene il race conditions,
              linguaggio  assicura  che  un  dato  non  possa essere mutato mentre 
              viene letto da altre parti del programma
      Protezione lifetime e delle reference
         cioè è il compilatore che individua e traccia chi sta accedendo a cosa e per quanto tempo.
         inoltre estende la protezione anche alle operazioni concorrenti e cioè eseguire delle 
         operazioni in parellelo senza  preoccupazione di deallocare la memoria oppure che si
         creano race conditions = dati lette mutato da altri.
         La programmazione asincrona è sicura in quanto perche utilizza
            Un task asincrono è un'unità di lavoro che può sospendersi e riprendere più tardi, senza bloccare il thread.
            Ti permette di scrivere codice che fa "altro" mentre aspetta (es. attende dati dalla rete, un file, un timer…).
            Viene gestito da una runtime asincrona (come tokio o async-std in Rust), che programma i task in modo efficiente.
            Cos'è un Future? = Un Future è un oggetto che rappresenta il risultato di un'operazione asincrona che 
            non è ancora disponibile, ma lo sarà in futuro.

### Scope delle variabili
   todo : da finire




### Le costanti e le variabili
   Le costanti mantengono i loro valori immutabili. Le variabili consentono la  
   la memorizzazione e la manipolazione dei dati.  
   Pag. 38 - Sia le COSTANTI che le VARIABILI sfruttano la memoria RAM e cessano al termine  
   delle'esecuzione salvo in cui sono salvata su dispositivi esterni.  
   Let = parola chiave di dichiarazione e di default immutabili salvo se necesario per cambiare valore occorre  
   la parola chiame: mut  
         es. immutabile
            let x = 5; 
            println!("Il valore di x è: {}", x);  
         es. MUTABILE  
                let mut y = 10; //DICHIARATA 10 MA MUTABILE
                println!("Il valore iniziale di y è: {}", y); 
                y = 20; //ASSEGNO NUOVO VALORE
                println!("Il valore modificato di y è: {}", y);  
   Le costanti dichiarate con const sono empre immutabili. Rispetto alle variabili le costanti devono avere sempre  
      il tipo dichiarato  e possono essere utilizzate ovuncque nel programma; es   const MAX_PUNTI: u32 = 100_000;    
      MAX_PUNTI = valore massimo dichiarato come u32.   
   shadowing = consente di riutilizzare il nome della variabile in uno scope successivo quindi utile quando si ha  
      bisogno di trasformare il valore della variabile SENZA MODIFICARNE IL TIPO O LA MUTABILITA.  
      Con shadowing una nuova variabile con lo stesso nome viene creata nascondente la precedente es.  
            let z = 6; 
            let z = z + 1;       //shadowing della variabile con lo stesso nome aumenta di valore  
            let z = z * 2;       //shadowing della variabile con lo stesso nome aumenta di valore * 2 fino a 14
            Il compilatore capisce che z è dello stesso tipo.  

            ATTENZIONE che quando necessario è megli dichiare il tipo:   let a: f64 = 3.14;  // dichiarazione di una variabile di tipo floating point  
            la corretta tipizzazione delle variabili rende i codice Rust sicuro. 
### Tipologia delle variabili
PAG 40 - Le variabili sono contenitori che consentono ai programmatori di memorizzare e manipolare di dati durante l'esecuzione  
del programma; ed in base al tipo associato, si stabilisce il valore che puo essere manipolato. In Rust le variabili sono divise  
in tipologie per consentire al compilatore di eseguire controlli in sicurezza ed ottimizzazioni; le tipologie sono:  
   * __Tipi scalari__   : sono variabili che rappresentano il numero INTERO, VIRGOLA MOBILE, BOOLEANO e UN CARATTERE
      let intero: i32 = 42;         // i32, u32, i64  
      let float: f64 = 3.14;        // Virgola mobile f32 f64  
      let booleano: bool = true;  
      let carattere: char = 'R';  
   * __Tipi composti__  : sono variabili che che COMBINANO PIU VALORI in un **singolo tipo** cioè tuple ed array.  
         
         TUPLA  
            es:  let tupla: (i32, f64, char) = (42, 3.14, 'R');  
             let (x, y, z) = tupla;  
            questa tupla contiene un inter, un virgola mobile ed un carattere ed i valori possono essre destrutturati in x,y e z.  
         
         ARRAY  
                let array: [i32; 3] = [1, 2, 3];  contiene 3 numeri interi e gli elementi sono accessibile con la funzione len:  
                     println!("L'array ha {} elementi.", array.len());    //len resituisce la lunghezza
         
         PATTERN MATCHING 
           PAG 41 = consente di verificare e destrutturare i valori in base al loro schema come ad esempio nelle tuple:  

               fn main() { 
                   let tupla = (1, 0); 
                   match tupla { 
                        (0, y) => println!("Il primo valore è zero e il secondo è: {}", y), 
                        (x, 0) => println!("Il secondo valore è zero e il primo è: {}", x), 
                          _ => println!("Nessuno dei valori è zero"), 
                      } 
                  } 
               con il match controlliamo la tupla e determiniamo il comportamento in base al valore.  
         
         PROPRIETA DELLE VARIABILI  
            quando una variabile viene assegnata ad un'altra variabile oppure viene passata ad una funzione la  proprieta viene  
            trasferita e la variabile originale non è piu utilizzabile, in questo modo gli errori legati alla gestione della  
            memoria vengono evitati es.  

                  let s1 = String::from("Ciao"); 
                   let s2 = s1;  // La proprietà di s1 è trasferita a s2 
                   
                   // println!("{}", s1); // Questo darà errore perché s1 non è più valido 
                   println!("{}", s2);  // Questo è valido
         BORROW - PRESTITO  
            PAG 42 - borrow è il prestito ossia la possibilita di accedere ai dati di una variabile SENZA TRASFERIRE LA PROPRIETA. 
              - primo modo    = leggo i dati senza modifica (PRESTITO IMMUTABILE)
              - secondo modo = leggo i dati senza modifica

   * __Le stringhe__    : sono piu complesse rispetto agli altri linguaggi per la sicurezza, e sono due tipi :  
         - &str   = stringhe slice di tipo immutabile  
                 let s_slice: &str = "Ciao, mondo!"; // stringa slice che punta a una stringa immutabile, utile  
                                    per operazioni leggere e quando non è necessario modificare la stringa   
         - String = stringhe allocate dinamicamente.  
               -  let mut s_string = String::from("Ciao"); 
                  s_string.push_str(", mondo!"); 
                     Adatta per le modifiche durante l'esecuzione del programma, l'esempio inzia con Ciao e poi viene  
                     modificata con mondo.
         - 

   * __I booleani: s__  : sono i tipi bool con due valori false o true; e quindi utilizzati per il controllo di flusso  
         con strutture if ed while o nei pattern matching.  
         __Option<T>__  : Pag 45 - In rust non abbiamo i valori nulli (null pointer exceptions) ed i nulli vengono gestiti con  
               con il tipo Option che rappresenta un valore  :
                  - Valore presente (Some(T))  
                  - Valore assente  (None)  
               con questo meccanismo gestisco in MODO ESPLICITO ci casi di null es:  

                  fn main() { 
                      //2 variabile un valida 42 ed una nulla
                      let some_value: Option<i32> = Some(42); 
                      let no_value: Option<i32> = None; 
                   
                      match some_value { 
                          Some(x) => println!("Il valore è: {}", x), 
                          None => println!("Non c'è valore."), 
                      } 
                        //CON IL MATH viene gestito sia il valore valido   e sia il null con un msg
                        match no_value { 
                             Some(x) => println!("Il valore è: {}", x),    //esiste il valore
                             None => println!("Non c'è valore."),          //valore null msg
                         } 
                   }
                  - 


         __tipi unitari  
            PAG 45 -rappresentati da () = che è un tipo speciale  utilizzato quando non è necessario restituire  
                  alcune valore da una funzione o espressione è simile al void di C++ anche se viene trattato  
                  come un vero dato; con () il ritorno è implicito ed è utilie quanto una funzione esegue una  
                  azione come LA STAMPA DI UN MESSAGGIO SULLA CONSOLE.  
                     ESEMPIO  
                           fn main() { 
                               stampa_messaggio(); 
                           } 
                            
                           fn stampa_messaggio() { 
                               println!("Questa funzione non restituisce nulla."); 
                           } 

