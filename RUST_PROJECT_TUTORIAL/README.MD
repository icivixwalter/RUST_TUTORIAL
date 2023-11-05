README
		@comandi@rust_(e anche i @comandi@cargo)

C
	
	Cargo
		
      cargo build
      			@cargo@build_(per costruire una @release)
      			
         Costruzione per il rilascio
         Quando il progetto è finalmente pronto per il rilascio, è possibile 
         compilarlo con le ottimizzazioni. Questo comando creerà un oggetto 
         eseguibile in target/release invece di target/debug. Le ottimizzazioni 
         rendere il tuo codice Rust più veloce, ma attivarli allunga il tempo 
         necessario per la compilazione del programma. Questo è il motivo per 
         cui ci sono due diversi profili: uno per lo sviluppo, quando si 
         desidera ricostruire rapidamente e spesso, e un altro per 
         Costruire il programma finale che darai a un utente che non verrà 
         ricompilato ripetutamente e che funzionerà il più velocemente possibile. 
         Se stai effettuando il benchmarking del tuo tempo di esecuzione del codice, 
         assicurarsi di eseguire ed eseguire il benchmark con L'eseguibile 
         in Target/Release.cargo  

             cargo build --release


	COMMAND MODE
		viene eliminato con esc+a

M
	MULTI CURSORE
		ALT+Maius

S
	SELEZIONA RIGHE
		si attiva con freccia + su	
		si attiva con freccia + down	


		



NUOVI COMANDI DI RUST

E
	ELENCO ERRORI DI RUST
		Prestiti di variabile
			1)  cannot borrow `v` as mutable because it is also borrowed as immutable
    			v.push(34);

    			 non può prendere in prestito 'v' come mutevole perché è anche preso in prestito come immutabile v.push(34);

    			note: previous borrow of `v` occurs here; the immutable borrow prevents
					subsequent moves or mutable borrows of `v` until the borrow ends
					for i in &v {
					          ^

	          	Nota: qui si verifica il precedente prestito di 'V'; Il prestito immutabile impedisce
					Mosse successive o prestiti mutevoli di 'V' fino alla fine del prestito
					per i in &v {
					          ^

P
	parse
			@convertire@stringa_(per @convertire una stringa in un numero utilizzare parse)

		metodo di String, una volta assegnato ad una variabile es.
			 let mut tentativo = String::new();

		 Stringa ha il metodo .tim per ripulire la stringa da spazi es. 
		 	digito 5\n
		 trim elimina lo spazio, e recupera il 5 (che è una stringa da convertire)	

	 il metodo .parse = analizza la stringa della variabile  tentativo e la trasforma
	 i u32

        let tentativo: u32 = tentativo.trim().parse()
        		.expect("Prego, digita un numero!");
		



T
	TIPI DI VALORI
			@tipi@di@valori_(intero @32, )
		u32 = intero senza segno a 32 bit

W
	WEB
	    il tutorial su rust si trova qui:
	        https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
