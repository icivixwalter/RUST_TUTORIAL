@README_TUTOTORIAL_RUST.MD

APERTURA COMANDI SUBLIMETEXT 
   Note
      Tutti i file che verrano descritti si trovano in questa path: 
         Directory di C:\Users\icivi\AppData\Roaming\Sublime Text\Packages\User
      
      Nella directory use sono stati creati 3 file Build_*.* dei seguenti tipi:
         1) Build_ESEGUI_DOS.sublime-build
               questo è il comando sublime text che attiva il seconda file .python
         2) Build_ESEGUI_Python.py
               Tale file *.py crea almeno 3 scelte nel menu di sublime text e passa
               la scelta come parametro al successivo file *.bat

         3) Build_APRI_CMD.BAT
               Questo è il bat che esegue il comandodi apertura del file o progetto
               scelto.
         4) Default.sublime-commands
            questo file contiene il menu da attivare con il comando: CTRL+SHIFT+P

      FLUSSO DEL PROGRAMMA
         
         
          Ecco una guida ordinata e chiara per gestire e ampliare il sistema di apertura file README in Sublime Text usando menu, Python e batch.
         
         ✅ SISTEMA DI APERTURA FILE README IN SUBLIME TEXT
         
            📁 1. STRUTTURA DEI FILE
            Tipo  Nome file   Descrizione
            .sublime-build Build_ESEGUI_DOS.sublime-build   Attiva il comando Python personalizzato apri_readme.
            .py   Build_ESEGUI_Python.py  Mostra il menu delle opzioni e avvia lo script .BAT con il numero scelto.
            .BAT  Build_APRI_CMD.BAT   Apre il file README corrispondente al numero ricevuto.
         
         
            ⚙️ 2. COME FUNZIONA IL FLUSSO
               [Sublime Build] ─▶ [Python Command: apri_readme] ─▶ [Menu: scegli un file] ─▶ [Esecuzione BAT] ─▶ [Apertura file README]
            
            🪛 3. MODIFICHE PER AGGIUNGERE NUOVE OPZIONI
                     qui poi aggiungere altre righe in python
            
                3.1 Modifica Build_ESEGUI_Python.py
                  self.options = [
                         "1. Apri README TUTORIAL RUST",
                         "2. Apri README COMANDI VARI",
                         "3. Apri TUTORIAL POCKET OPTION"
                     ]
         
         
            🔹 3.2 Modifica Build_APRI_CMD.BAT
                  qui puoi aggiugere le altre voci da chiamare con i 
                  parametri passati da python:
                        @echo off
         
                        if "%1"=="1" start "" "C:\path\to\README_TUTORIAL_RUST.md"
                        if "%1"=="2" start "" "C:\path\to\README_COMANDI_VARI.md"
                        if "%1"=="3" start "" "C:\path\to\TUTORIAL_POCKET_OPTION.md"
                        if "%1"=="4" start "" "C:\path\to\------"
         


   Note
      I COMANDI DEI LINGUAGGI gli originali si trova qui NELLA CARTELLA DI STUDIO DI RUST

         START "APRI ORIGINALI COMANDI README.MD" c:\Casa\LINGUAGGI\RUST\RUST_TUTORIAL\RUST_TUTORIAL_TONY_CHAN\README.MD

GITHUB
   @Progetto@web
      si trova in questa posizione su github:
         https://github.com/icivixwalter/RUST_TUTORIAL

   @comando@githu@unico
      copiare il comando ..
         git status & git pull & git add . & git commit -m "agg" & git push

@DOS
     dir /A /b *.*  >> A.TXT           = salvo su a.txt le cartellse senza path
     dir /s /b >> output.txt           = salvo tutte le sotto cartelle ed i file con la path completa
     dir *.* /w >a.txt                 = salvo 
SUBLIMETEXT
   Comandi contenuti in Preferences-->Key Bindings

      ctrl+shift+t                     = terminale basso
      ctrl+shift+y                     = chiudo il terminale basso
      ctrl+shift+r                     = terminale a destra nuovo tab

COMANDI MARKDOWN 
   Preview
      per aprire in visualizzazine diretta scrivere:

         view ---> row2                 = visualizzo in orizzontale pannello 1 sopra e 2 sotto
         ctrl+shift+p+ OPEN PREVIEW     = command palette + OPEN PREVIEW ---> PREVIEW IN MARKDOWN IN 
                                          UNA TAB SEPARATA,   @visualizza@markdown@preview
         alt+shift+1                    = a 1 scheda
         ctrl+shift+p                   = command palette
         cr                             = ritorno a capo con 2 spazi
                                        oppure <br> come forzatura
         grassetto                      = puoi utilizzare doppio asterisco o doppio undescore
                                          es.   **grassetto** oppure __grassetto__
         dimensione carattere           = non esiste in mardown la possibilita di aumentare la   
                                        dimensione del carattere ma si puo utilizzare Html es:  
                                        grassetto + carattere 24 px
                                        **<span style="font-size: 24px;">trait</span>**

         alt+shift+8                   = visualizzo in 2 pannelli orizzontali



TRASFORMARE .MD IN @WORD
         pandoc                        = scarica pandoc come programma esterno +
                                       1) installa pandoc: https://github.com/jgm/pandoc/releases/tag/3.7.0.2   
                                       2) Crea un Build System in Sublime
                                          Vai su:Tools > Build System > New Build System  
                                          Incolla questo codice
                                             {
                                              "cmd": ["pandoc", "$file", "-o", "${file_base_name}.docx"],
                                              "file_regex": "^(...*?):([0-9]*):?([0-9]*)",
                                              "selector": "text.html.markdown"
                                             }

                                       3) per attivare pandoc e trasformare il file aperto corrente  
                                       da .md a docx attivare lo script :  
                                          Tools --> Builds with --> Pandoc
                                          e verra creato il file .docs nella stessa cartella.
                                             @creare@file@doc@docs
                                          


                                    
