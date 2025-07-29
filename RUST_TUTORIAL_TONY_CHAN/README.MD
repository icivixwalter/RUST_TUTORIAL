@TUTOTIAL@TONY@CHAN
   Note
      faccio un riassunto del libro di rust


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
                                          UNA TAB SEPRATA
         alt+shift+1                    = a 1 scheda
         ctrl+shift+p                   = command palette
         cr                             = ritorno a capo con 2 spazi
                                        oppure <br> come forzatura

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

                                       3) per attivare pando e trasformare il file aperto corrente  
                                       da .md a docx attivare lo script :  
                                          Tools --> Builds with --> Pandoc
                                          e verra creato il file .docs nella stessa cartella.
                                          


                                    
