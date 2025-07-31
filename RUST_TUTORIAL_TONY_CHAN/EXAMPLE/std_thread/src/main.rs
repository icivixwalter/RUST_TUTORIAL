/* PROGETTO DI GESTIONE DEI THREAD

   @std@thread_(esempio di gestione della concorrenza con i thread)


*/

// Importa il modulo 'thread' per creare ed eseguire thread paralleli
use std::thread;

// Importa 'Duration' per gestire le pause temporali tra le operazioni
use std::time::Duration;

fn main() {

     println!("PROGETTO DI GESTIONE DELLA LIBRERIA STD::Thread!");

    // Avvia un nuovo thread secondario usando thread::spawn
    // Il thread eseguirà la closure (funzione anonima) definita tra le parentesi
    let handle = thread::spawn(|| {
        // Il thread secondario esegue un ciclo da 1 a 9
        for i in 1..10 {
            // Stampa un messaggio dal thread secondario
            println!("Ciao dal thread secondario! {}", i);
            // Fa una pausa di 100 millisecondi per ogni iterazione
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Il thread principale esegue un ciclo da 1 a 4
    for i in 1..5 {
        // Stampa un messaggio dal thread principale
        println!("Ciao dal thread principale! {}", i);
        // Fa una pausa più breve: 50 millisecondi per ogni iterazione
        thread::sleep(Duration::from_millis(50));
    }

    // Aspetta che il thread secondario finisca la sua esecuzione
    // 'join' blocca l'esecuzione del thread principale finché il secondario non termina
    // 'unwrap' serve per gestire eventuali errori, e in caso di errore provoca un panic
    handle.join().unwrap();
}
