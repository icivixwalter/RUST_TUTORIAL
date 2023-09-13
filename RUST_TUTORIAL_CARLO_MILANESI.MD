RUST_TUTORIAL_CARLO_MILANESI
	Note
		questo è un tutorial in italiano di carlo milanensi e 
		si trova qui:
			https://carlomilanesi.github.io/linguaggio-rust/getting-started.html

	CREARE_FILE_PROGETTO
		rust è create per Tier o 3 livelli con diverse garanzie.
		Le piattaforme sono identificate con la target-triple = ossia la stringa che (terna-bersalio) che informa il compilatore su quale output deve essere prodotto.

		LIVELLO 1
			Le piattaforme di livello 1 possono essere pensate come "con garanzia di compilazione e funzionamento".
		LIVELLO 2
			Le piattaforme di livello 2 possono essere pensate come "con compilazione garantita". I collaudi automatizzati non vengono eseguiti, e quindi non è garantito che si producano eseguibili che funzionino, ma queste piattaforme spesso funzionano abbastanza bene e le migliorie sono sempre benvenute! 

		LIVELLO 3
			Le piattaforme di livello 3 sono quelle per cui Rust ha supporto, ma per applicare delle modifiche non è necessario né eseguire la compilazione, né passare i testi di collaudo. Gli eseguibili funzionanti su tali piattaforme possono essere lacunosi, dato che la loro affidabilità è spesso definita in termini di contributi dalla comunità. In aggiunta, gli artefatti di rilascio e gli installatori non sono forniti, ma ci possono essere infrastrutture comunitarie che li producono in luoghi non ufficiali.

	INSTALLAZIONE
		su linux, windows ecc..

	NUMERO_DI_VERSIONE
		Aprire il terminale e controllare il numero di @versione di @rust:
			rustc --version

		Per windows controllare la variabile di sistema con 
			echo %PATH%.
		se non esiste impostarla manualmente in quanto  gli eseguibili di Rust si trovano in una directory come "C:\Program Files\Rust stable GNU 1.x\bin".

		... RUST non fa i link

	CREARE DIRECTORY PER I PROGETTI
		per iniziare creare una directory di progetto e con la funzione main. es.
			fn main() {
		main senza parametri () vuote, e con una macro
			println!("hello word");
		infatti println senza 	(!) = è una funzione	
		infatti println! con 	(!) = è una MACRO
		{};							= le parentesi racchiudono tutte le funzioni e la riga finisce con 
									il ; perchè Rust è un linguaggio orientato alle ESPRESSIONI infatti
									la maggior parte delle cose sono espressioni e con il ; si dichiara
									la fine della espressione.

	COMPILARE ED ESEGUIRE SONO PASSI SEPARATI
		Rust è un linguaggio compilato in anticipo tipo il c++ perchè si ha il sorgente ed un eseguibile
		sulle piattaform a differenza degli  dei linguaggi dinamici tipo Python dove occore consegnare
		il sorgente .py ma occorre anche la macchina virtuale per l'esecuzione, cià che non serve in rust.
		La compilazione con rust c = va bene per i programmi semplici altrimenti occorre usare cargo.

		CARGO
			È un sistema di buile e di gestore di pacchetti Rust  per i progetti ed esegue 3 cose:
				- compila il codice;
				- scarica le librerie;
				- e costruisce le librerie;

			la versione di cargo è con : cargo --version

			Cargo utilizza un sistema ad ordine, la directory principale per README + LA LICENZA + I LINK quindi il sorgente va nella directory 
				src

			nella src ci sono i sorgenti tipo main.src oppure anche librerie lib.rs

			Per gestire con cargo occorre creare una direcotry madre
			una directory src dove inseire i file sorgenti menetre nella directory principale il Cargo.toml
			il toml = Questo file è nel formato TOML (Tom's Obvious Minimal Language). TOML è simile al formato INI, ma ha alcuni vantaggi, e viene usato come formato di configurazione di Cargo.
			Dentro questo file, si digiti la seguente informazione:

					[package]				= 	le seguenti direttive costruiscono un pacchetto

					name = "hello_world"	= il nome del programma
					version = "0.0.1"		= la versione
					authors = [ "Il tuo nome <tu@esempio.it>" ] = chi lo ha scritto
				
				hello_word_cargo
					Cargo.toml
					src
						main.rs


			Cargo viene utilizzato per grandi progetti, perchè  si iniziano ad usare i "crate"; questi, in altri linguaggi di programmazione, sono chiamati ‘librerie’ o ‘pacchetti’. 

			Caon cargo per rilasciare il progetto finale occorre questo comando:
				cargo build --release 
			con questa secondo profilo la compilazione è piu lenta ma viene ottimimizzata e velocizzato
			il file finale .exe

			CARGO LOCK
				è il file in cui cargo tiene conto delle dipendenze dell'applicazione.


			CARGO CREA PROGETTO
				per creare un nuovo progetto occorre eseguire questo comando
					cargo new nome_progetto --bin
					 						--bin = opzione per creare direttamente un eseguibile e non
					 						una libreria





	GIOCO-INDOVINA
		Impostazione
			imposto il nuovo progetto nella directory con il comando
				cargo new gioco_indovina --bin
				                         --bin = creato un nuovo progetto eseguibile non una libreria.

         Elaborare un tentativo
         	Elaboraiamo il file main per consentire al giocatore di inserire i tentativi:

         		use std::io;		= libreria per lo standard io
         		fn main() { 		= e () indicano che non ci sono argomenti,
         		let 				= istruzione let, che viene usata per creare ‘legami a variabili’ 
         							  [variable bindings].

				  esempio
				  	let foo = bar;
					  	Questa istruzione crea un nuovo legame chiamato foo, e lo lega al valore bar. In molti linguaggi, questo si chiama ‘variabile’, ma i legami di variabile di Rust hanno alcuni assi nella manica.

					  	l'istruzione senza mut rende immutabile il legame in quanto di default

					  		let foo = 5; // immutabile.
							let mut bar = 5; // mutabile

						con l'istruzione 
							    let mut tentativo = String::new();
					    viene creata una variabile mutevole di tipo Stringa associata ad un tipo String
					    e non un nuova istanza di String e quindi è un METODO DI CLASSE o un METODO STATICO.

				    	con l'istruzione
				    		io::stdin().read_line(&mut tentativo).expect("non si riesce a leggere la riga");
				    		utilizza lo std::io
				    			std::io::stdin   = legge l'imput da linea di comando
		    		    		(&mut tentativo) = imposta la variabile tentativo mutevole con il 					risultato
		    		    		.expect("non ..) = visto che anche un Result (io::Result) come primo
		    		    		caso generico e anche sottolibreria ma viene utilizzato per il messaggio di errore se si va in panic.

    		    		le {} nella macro sono segna posto
    		    		Questa riga stampa la stringa dove abbiamo salvato il nostro input. Le parentesi graffe {} sono dei segna-posto per passare tentativo come argomento. Se avessimo scritto diversi {}, avremmo passato diversi argomenti:

									let x = 5;
									let y = 10;

									println!("x e y: {} e {}", x, y);



	GENERARE UN NUMERO SEGRETO 
		crate rand
			cassone o scatola oppure libreria che permette per i numeri casuali, nel frattempo viene
			creato un nuovo progetto per i numero segreto:
				mkdir genera_numero_segreto

			con 
				cargo new numero_segreto --bin 

			dopo aver creato il progetto per utilizzare la libreria rand occore indicare la dipendenza
			nel toml
				rand="0.3.0"

			dependencies per sapere quali dipendenze ci sono da crate esterni, e quali versioni di essi sono richieste. In questo caso, abbiamo specificato la versione 0.3.0, che Cargo capisce essere qualunque rilascio che è compatibile con questa specifica versione.
			Cargo va a prendere le versioni più recenti di ogni cosa dal registry, che è una copia di dati presi da Crates.io. Crates.io è il posto dove la gente nell'ecosistema di Rust invia i suoi progetti open source in Rust per farli usare ad altri.

			Dopo aver aggiornato il registry, Cargo verifica la nostra sezione [dependencies] e scarica tutti i pacchetti che non abbiamo ancora. In questo caso, mentre abbiamo detto soltanto che volevamo dipendere da rand, ci siamo presi anche una copia di libc. Questo perché rand dipende da libc per funzionare. Dopo averli scaricati, li compila, e poi compila il nostro progetto.
			Quando costruiamo un progetto da zero ed impostiamo la dipendenza ad es. rand=0.3.0, cargo
			imposta il Cargo Lock a questa versione e se uscira una successiva tipo  rand=0.3.9 il 
			cargo lock impedisce di scacarica la nuova versione per il pericolo della incompatibilita
			retroattiva a meno che non impostiamo a mano il 0.3.9 NEL TOML
			ALTERNATIVA per FORZARE IL CARGO.LOCK possiamo usare
				cargo update
			che significa ignora il cargo.lock e cerchera di aggiornare le versioni > 0.3.0 and < 0.3.9
			ma non la versione 0.4.0 che deve essere esplicitamente impostata nel .toml

			per utilizzare rand dopo averlo dichiarato nelle dipendence del .toml
			occore inserire nel file la dichiarazione di utilizzo:
				extern crate rand;


			viene inserito 
				use std::cmp::Ordering;

			STD::CMP = è un metodo che restituisce un ORDERIN cioè un enum  < = >.
			cmp = viene chiamato per qualunque oggetto da confrontare con riferimento allo
			stesso oggetto da confrontare  e restisce un Ordering.
			Ordering = è un enum con 3 POSSIBILI VARIANTI
				LESS, EQUAL, GREATER

			serve per confrontare i numeri  e deve essere integrato con un match
				
				match tentativo.cmp(&numero_segreto) {
				    Ordering::Less    => println!("Troppo piccolo!"),
				    Ordering::Greater => println!("Troppo grande!"),
				    Ordering::Equal   => println!("Hai vinto!"),
				}


			
			CONVERSIONE DEL TIPO STRINGA CON PARSE	

				Il metodo parse() applicato a una stringa analizza la stringa estraendone un numero di qualche tipo. Dato che tale metodo può riconoscere vari tipi di numeri, dobbiamo suggerire a Rust il tipo esatto del numero che vogliamo. Pertanto, scriviamo let tentativo: u32. I due-punti (:) dopo tentativo dicono a Rust che stiamo annotando il tipo del legame. u32 è il tipo intero senza segno a trentadue bit. 
					fn main() {
				        let tentativo: u32 = tentativo.trim().parse()
				        .expect("Prego, digita un numero!");
				}


			LOOP
				Viene creato un loop
				ed inserito la gestione del panic

	LEGAMI DI VARIABILE - 4.1
			
			Per introdurre un legame, si usa la parola-chiave let, così:
					let x = 5;

			I PATTERN - 4.1.1
				negli altri linguaggi si chiama variabile, ma i legami di variabile di Rust hanno alcuni assi nella manica. Per esempio, il lato sinistro di un'istruzione let è un ‘pattern’, non un semplice nome di variabile

						let (x, y) = (1, 2);
						Dopo che questa istruzione viene eseguita, x varrà uno, e y varrà due. (vedere i pattern)

			ANNOTAZIONI DI TIPO - 4.1.2
				Rust è un linguaggio tipizzato staticamente, il che significa che specifichiamo subito i tipi, e questi vengono verificati in fase di compilazione.
				Ma rust ha anche la 
					INFERENZA DI TIPO. Se riesce a desumere qual'è il tipo di qualche 
										dato, Rust non costringe a digitarlo splicitamente.

				Se vogliamo, possiamo aggiungere il tipo di dato.
					let x: i32 = 5;

				Rust ha molti tipi interi primitivi. I loro nomi cominciano con i per gli interi con segno, e con u per gli interi senza segno (unsigned). Le dimensioni intere possibili sono 8, 16, 32, e 64 bit.

			MUTABILITÀ
				Di default, i legami sono immutabili. Questo codice non compilerà:
					let x = 5;
					x = 10;
				Se si vuole che un legame sia mutabile, si deve usare mut:
					let mut x = 5; // mut x: i32
					x = 10;

				LA RAGIONE PRINCIPALE DELLA IMMUTABILITA DEI LEGAMI E' LA SICUREZZA.
				Se ci si dimentica di dire mut, il compilatore se ne accorgerà, e farà sapere che si ha mutato qualcosa che si potrebbe non aver inteso mutare. Se i legami fossero mutabili di default, il compilatore non sarebbe in grado di dirlo. Se si intendesse proprio la mutazione, allora la 
				soluzione è facilissima: aggiungere mut.

			INIZIALIZZARE I LEGAMI
				I legami di variabile in Rust hanno un altro aspetto che differisce da altri linguaggi: I LEGAMI DEVONO ESSERE INIZIALIZZATI CON UN VALORE PRIMA DI POTERLI USARE.
				Se utilizziamo ugualmente una variabile senza inizializzabile Rust ci da errore; Rust non ci permetterà di usare un valore che non è stato inizializzato.
					es.
						fn main() {
						    let x: i32;

						   !=macro
						   				   1°               2°
						   2 argomenti |------------------||--|
						    println!("Il valore di x è: {}", x);
				
					Rust non ci permetterà di usare un valore che non è stato inizializzato.
			INTERPOLAZIONE STRINGA
					@interpolazione_(la @interporabilita della stringa è l'inserimento
						di una stringa all'interno di un'altra stringa,)
					@{}_(le @grasse sono dei @segnaposto per dire dopo la virgola interpoliamo il valore)

				println!("Il valore di x è: {}", x); utilizza l'interpolazione stringa ossia l'inserimento di una stringa all'interno di un'altra stringa al posto di altrettanti segnaposto. Infatti le {} sono dei segnaposto per cui viene detto a rust dopo la stringa tipo :
					"Il valore di x è: {}",  = indichiamo a rust che dopo la virgola (,)
					interpoliamo il valore di x e la virgola serve a separare gli argomenti che passiamo alle funzioni e alle macro.


			AMBITO ED OSCURAMENTO
				I legami di variabile hanno un ambito - ossia sono vincolati a risiedere nel blocco in cui sono stati definiti. 
				Un blocco è una collezione di istruzioni racchiuse da { e }. Anche le definizioni di funzione sono blocchi! 

				Nell'esempio seguente definiamo due legami di variabile, x e y, che risiedono in blocchi diversi. Si può accedere a x da tutto il blocco fn main() {}, mentre si può accedere a y solamente dal blocco più interno:

						fn main() { //I° BLOCCO GRAFFE
						    let x: i32 = 17;

						    //II° BLOCCO GRAFFE SEPARATO
						    {
						        let y: i32 = 3;
						        println!("Il valore di x è {} e il valore di y è {}", x, y);
						    }

						    //non puoi stampare x
						    println!("Il valore di x è {} e il valore di y è {}", x, y); // Questo non funziona Y non accessibile
						}
				La prima println! stamperebbe "Il valore di x è 17 e il valore di y è 3", ma questo esempio non può essere compilato con successo, perché la seconda println! non può accedere al valore di y, dato che non è più nel suo ambito. Otteniamo invece questo errore:




				OSCURAMENTO - SHADOWED
				 	Inoltre, i legami di variabile possono venire oscurati ("shadowed"). Ciò significa che un successivo legame di variabile con il medesimo nome di un legame attualmente nel suo ambito scavalcherà il legame precedente.

					let x: i32 = 8;
					{
					    println!("{}", x); // Stampa "8"
					    let x = 12; 	   //OSCURA LA X =8 
					    println!("{}", x); // Stampa "12"
					}
					println!("{}", x); // Stampa "8"
					let x =  42;       //OSCURA LA X =8 
					println!("{}", x); // Stampa "42"

					ATTENZIONE l'ocsuramento lega un nome ad un valore diverso ed è ammissibile il mut (es. x = 5 poi shadowe = x="pippo" è un testo non un numero)
						@oscuramento_(è possibile il mutamento? si NON VIENE DISTRUTO MA NON E' ACCESSIBILE)
					l'oscuramento non alter o distrugge il valore ma continua ad esistere
					finche non esce dall'ambito MA NON E' ACCESSIBILE.
						ES.
							let mut x: i32 = 1;
							x = 7;
							let x = x; // x adesso è immutabile ed è legato a 7

							let y = 4;
							let y = "Posso anche essere legato a un testo!"; // y adesso è di un altro tipo


	FUNZIONI 4.3
		MAIN
					@funzioni_(nelle funzioni i @parametri devono essere dichiarati )

			Ogni programma Rust ha almeno una funzione, la funzione main:

				fn main() {
				}

			NO ARGOMENTI
				Questo è la dichiarazione di funzione più semplice possibile. Come accennato prima, fn indica che ‘questa è una funzione’, ed è seguita dal nome della funzione, da due parentesi vuote perché questa funzione non prende argomenti, e poi da parentesi graffe che contengono il corpo della funzione. Ecco una funzione chiamata foo:

					fn foo() {
					}

			PARAMETRI
				fn main() {
						//chiamo la funzione e passo il parametro
					    stampa_numero(5);
					}

					//creato fuori main la funzione con 1 parametro
					fn stampa_numero(x: i32) {
					    println!("x is: {}", x);
					}

			NOME + TIPO ARGOMENTO
				Note
					Come si vede, gli argomenti delle funzioni funzionano in modo molto simile alle dichiarazioni let: si aggiunge un tipo al nome dell'argomento, dopo i due punti :.

					Ecco un programma completo che somma due numeri e stampa il risultato:

						fn main() {
						    stampa_somma(5, 6); //Virgola quando chiamo la funzione
						}
						
						//			la funzione è esterna a main
						//--------------------------------------------------------------//
						//x: i32 = NOME+TIPO ..., passo 2 argomenti numerici x,y
						fn stampa_somma(x: i32, y: i32) { //virgola quando dichiaro la funzione
													   //	
						    println!("la somma è: {}", x + y);
						}

				LA VIRGOLA
					Si separano gli argomenti usando una virgola, sia quando si chiama la funzione, che quando la si dichiara.

				OBBLIGATORIA LA DICHIARAZIONE DEI TIPI
					Diversamente dall'istruzione let, i tipi degli argomenti delle funzioni devono essere dichiarati. Pertanto questo non funziona:

							fn stampa_somma(x, y) {
							    println!("la somma è: {}", x + y);
							}
							Si ottiene l'errore:
								previsto 1 tra  ! o @ ..
								expected one of `!`, `:`, or `@`, found `)`
									fn print_sum(x, y) {

				INFERENZA
							@inferenza_(determinazione del tipo di dato non dichiarato)

						E' una attivita in cui il compilatore  usa l'inferenza dei tipi  per determinare i tipi di dati delle variabili locali dichiarate senza un Tipo. Il compilatore inferisce il tipo della variabile dal tipo dell'espressione di inizializzazione. Ciò consente di dichiarare le variabili senza specificare in modo esplicito un tipo, come illustrato nell'esempio seguente. Come risultato delle dichiarazioni, entrambe num1 e num2 sono fortemente tipizzata come interi.
							   ' //esempio IN VB
								    Dim num1 As Integer = 3

								    ' Using local type inference.
								    Dim num2 = 3  //inferenza il compilatore comprende che è un intero

								//il vb all'interno della funzione @INFERENZA    
								Public Sub inferenceExample()

								    ' Using explicit typing. - tipo dichiarato
								    Dim num1 As Integer = 3

								    ' Using local type inference. 
								    Dim num2 = 3  //imposta come intero simile al tipo dichiarato perchè all'interno della funzione

								End Sub

					    In Rust L'INFERENZA è applicata all'interno del { corpo delle
					    	funzioni }, mentre è OBBLIGATORIA LA DICHIARAZIONE DEI TIPI
					    	nella DEFINIZIONE DELLE FUNZIONI.

			    VALORE DI RITORNO DELLA FUNZIONE
			    			@funzioni_(il valore di @ritorno con @->)
			    		Le funzioni di Rust restituiscono esattamente un valore, e si dichiara il tipo dopo una ‘freccia’, che è un trattino (-) seguito da un segno di maggiore (>). L'ultima riga di una funzione determina che cosa restituisce. Qui si noterà la mancanza di un punto-e-virgola. Se l'avessimo aggiunto:

									fn somma_uno(x: i32) -> i32 {
									    x + 1 // senza il; permette il valore di ritorno altrimenti errore
									}

				ESPRESSIONI CONTRO ISTRUZIONI
						@istruzioni_(in rust son di due tipi istruzioni quelle che restiscono un valore e quelle no. )

						@istruzioni@espressioni_(in rust restituiscono una tupla)

				ESPRESSIONI ED ISTRUZIONI DIFFERENZA

					Rust è primariamente un linguaggio basato sulle espressioni. Ci sono solamente due tipi di istruzioni, e ogni altra cosa è un'espressione.

					la differenza è che:
						espressioni = restiscono un valore
						istruzioni	= non restituiscono valori
							l'istruzione x + 1; non restituisce un valore. 

				ISTRUZIONI
					ci sono due tipi di istruzioni:

						ISTRUZIONE DI DICHIARAZIONE = con Let 
						ISTRUZIONE DI ESPRESSIONE 	= trasforma qualsiasi istruzione in 
													una espressione; significa che rust si aspetta da ogni istruzione segua un'altra istruzione.
				ISTRUZIONE DI DICHIARAZIONE

					in altri linguaggi esempio in Ruby pui creare espressioni e fare il 
					legame delle variabili es.
						 x = y =5;  //Ruby si puo' creare una espressione

				    In Rust in non è ammesso il legame delle variabile con una espressione es:
				    	let x = (let y = 5); // atteso un identificatore, trovata la parola-chiave `let` - produce errore.

			    	Ma rust utilizza let per attivare una istruzione e quindi non si puo assegnare come negli altri linguaggi con let una espressione, perche a differenza degli altri linguaggi con con l'istruzione:
			    		x = y = 5
			    		x ha 5 assegnato da y
		    		nel caso di rust
		    			let x = (let y = 5); // da errore in quanto  x non ha il valore
		    			di y  perche in:
		    				Rust = si ha un solo possessore  e quindi 5 non puo essere
		    				assegnato sia a y e sia ad x quindi in questo caso viene dato un errore.
    				nel caso invece di una istruzione di dichiarazione del tipo
    					let mut y = 5
    					let x = (y=6);
    					    x = ha valore di tupla vuota non di 6 mentre y contiene il 6.

			    ISTRUZIONE DI ESPRESSIONE
			    	le istruzioni di espressione il II tipo di istruzione, servono a Rust per trasformare QUALUNQUE ESPRESSIONE IN UNA ISTRUZIONE.
			    	Perchè rust si aspetta che ad ogni istruzione segua un'altra istruzione sempre separata da un ;
			    	
			    	UNICA ECCEZIONE AL ; è quando costruiamo una funzione che restituisce
			    	un valore nel cui corpo viene omessa il punto e virgola (;) perche è
			    	un valore di ritorno es.
			    		fn somma_uno (x: i32) {
		    				x+1  //senza ; restituisce il valore incrementato
			    		}

		    				.
    				e quindi in rust L'ISTRUZIONE DI ESPRESSIONE è sempre una istruzione di assegnamento e non di espressione in quanto anche un codice di questo tipo:
    					let mut y = 5;  //creo y mutubile

						let x = (y = 6);  // x ha valore `()`, non `6`
    				 
    				 let x è un tupla vuota ma non 6 non è un risutato di una espression
    				 di y che assegna a x, MA UNA ISTRUZIONE CHE ASSEGNA A X UNA TUPLA VUOTA.
    				 mentre y è uguale a 6.

    				 conclusione in rust ci sono istruzioni di assegnazioni o istruzioni di espressione, mentre tutto il resto sono espressioni.


				USCITE PRECOCI
					con la parola chiave return
						fn foo(x: i32) -> i32 {
						    return x;

						    // non si eseguirà mai questo codice!
						    x + 1
						}

					oppure con stile elegante:
						fn foo(x: i32) -> i32 {
								    return x + 1;
								}

			FUNZIONI DIVERGENTI

			sono le funzioni che non restituiscono mai il controllo al chiamante:

				fn diverge() -> ! {
				    panic!("Questa funzione non restituisce mai il controllo al chiamante!");
				}

				panic! è una macro, come lo è println!() che abbiamo già visto. Diversamente da println!(), panic!() manda in crash il thread corrente, stampando il messaggio ricevuto come argomento. Dato che questa funzione provocherà un crash, non restituirà mai il controllo al chiamante, e quindi ha il tipo ‘!’, che si legge ‘diverge’


			Se si aggiunge una funzione main che chiama diverge() e la si esegue, si otterrà un output simile a questo:
				thread ‘main’ panicked at ‘Questa funzione non restituisce mai il controllo al chiamante!’, main.rs:2

			Una funzione divergente può essere usata dove ci si aspetta un'espressione di qualunque tipo:

					let x: i32 = diverge();
					let x: String = diverge();

						






TODO: @da@finire
			si trova qui:
			https://carlomilanesi.github.io/linguaggio-rust/getting-started.html


INDICE_COMANDI_CARGO+RUST

	C
		CONVENZIONE IN RUST
					Vedi: https://www.freecodecamp.org/italian/news/impara-a-programmare-con-rust-corso-interattivo-in-linguaggio-rust-su-replit/#le-basi-di-rust
					
				NOMENCLATURA
					In Rust, le convenzioni di nomenclatura sono le seguenti:

					OGGETTO		STILE
					Variabili	snake_case
					Funzioni	snake_case
					File		snake_case
					Costanti	SCREAMING_SNAKE_CASE
					Static	    SCREAMING_SNAKE_CASE
					Tipi	    PascalCase
					Tratti	    PascalCase
					Enum	    PascalCase

				CONVENZIONE PER LE VARIABILI
					il trattino basso _ indica che la variabile non è utilizzata:
						fn main() {
								  let _unused_variable = my_func(10);
								}
				PARAMETRI CON SINTASSI :
					I parametri di funzione vengono scritti usando la sintassi :


						fn my_func(x: u8) -> i32 {
						  x as i32
						}

				STRINGHE E SLICE IN RUST
					Un motivo di confusione comune per i principianti di Rust è la differenza tra la struct String e il tipo str.

						//&str = memorizzata nello stack quindi fissa - riferimento c++ 
						let my_str: &str = "Hello, world!";
						//my_string = memorizzata nell'heap con grandezza sconosciuta al mento della compilazione

						let my_string: String = String::from("Hello, world!");


					Nell'esempio qui sopra, my_str è un riferimento a una stringa letterale, e my_string è un'istanza della struct String.

					Un'importante distinzione tra le due è che my_str è memorizzata nello stack, mentre my_string è allocata nella memoria heap. Ciò significa che il valore di my_str non può cambiare e la sua dimensione è fissa, mentre my_string può avere una dimensione sconosciuta al tempo di compilazione.

					Una stringa letterale è anche conosciuta come string slice (letteralmente "fetta"), poiché &str fa riferimento a parte di una stringa. In linea generale, ecco la somiglianza tra stringhe e array:


	I
		extern
			serve a comunicare a rust di utilizzare una dipendenza indicata nel .toml. CHE
			PROVVEDERA a scaricarla es. 0.3.0 SE NON esiste, e alla prima creazione del progetto
			imposta anche il Cargo.lock a questa dipendenza in modo che se usciranno ulteriori versioni
			es. 0.3.1 instabili non saranno scaricate a meno che non si imposti manualmente nel toml.
			Attenzione se esce altre due versione es. 0.3.9 e la 0.4.0 POSSIAMO USARE IL COMANDO
				cargo update
			che permette di aggiornare la versione solo tra > 0.3.0 and <= 0.3.9 quindi tutte le successive a quella imposta ma non la quattro che dovrà essere specificata nelle dipendenze
			del Cargo.toml.
			Il Cargo.lock fa una difesa del progetto in quanto utilizza sempre quella valida e senza errori.

				esempio
					.toml
						# Iimposto nel cargo.toml  la dipendenza della libreria rand
						[dependencies]
						rand="0.3.0"

					.rs
						//nel file main.rs COMUNICO a rust l'utilizzo della libreria rand con:

						extern crate rand;
