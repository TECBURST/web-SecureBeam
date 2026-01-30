export default {
  nav: {
    home: 'Home',
    security: 'Sicurezza'
  },
  comingSoon: {
    title: 'Stiamo ricostruendo SecureBeam',
    subtitle: 'Stiamo ripensando il trasferimento file da zero. WebRTC basato su browser aveva troppe limitazioni, quindi stiamo sviluppando app native per un\'esperienza migliore.',
    nativeApps: {
      title: 'Applicazioni Native',
      description: 'A partire da Windows, stiamo sviluppando app dedicate che offrono trasferimenti più veloci e affidabili.'
    },
    betterProtocol: {
      title: 'Protocollo Migliore',
      description: 'Le app native ci permettono di utilizzare protocolli più efficienti senza le limitazioni del browser.'
    },
    stayTuned: 'Resta sintonizzato per gli aggiornamenti. Qualcosa di grande sta arrivando.'
  },
  home: {
    title: 'Trasferimento File Sicuro',
    subtitle: 'Invia file direttamente a un altro dispositivo. Crittografia end-to-end, peer-to-peer, nessun limite di dimensione.',
    dropzone: {
      title: 'Trascina i file qui o clicca per sfogliare',
      subtitle: 'Nessun limite di dimensione. I file vengono trasferiti direttamente.'
    },
    or: 'oppure',
    receiveButton: 'Ho un codice per ricevere file',
    backButton: 'Torna a inviare file',
    codeInput: {
      placeholder: 'Inserisci il codice di trasferimento',
      button: 'Connetti'
    },
    selectedFiles: 'File Selezionati',
    clearAll: 'Rimuovi tutti',
    sendFiles: 'Invia {count} file',
    addMore: 'Aggiungi altri file',
    cancel: 'Annulla Trasferimento',
    complete: {
      title: 'Trasferimento Completato',
      message: '{count} file trasferito/i con successo.'
    },
    error: {
      title: 'Trasferimento Fallito'
    },
    newTransfer: 'Avvia Nuovo Trasferimento',
    tryAgain: 'Riprova'
  },
  code: {
    title: 'Il Tuo Codice di Trasferimento',
    subtitle: 'Condividi questo codice con il destinatario',
    copied: 'Copiato!',
    copy: 'Copia Codice',
    copyLink: 'Copia Link',
    qrHint: 'Oppure scansiona il codice QR',
    qrSubtitle: 'Scansiona per ricevere',
    linkSubtitle: 'Condividi questo link direttamente',
    tabs: {
      code: 'Codice',
      qr: 'QR Code',
      link: 'Link'
    }
  },
  status: {
    waiting: 'In attesa del destinatario...',
    connecting: 'Connessione in corso...',
    connected: 'Connesso! Preparazione trasferimento...',
    awaitingAcceptance: 'In attesa dell\'accettazione del destinatario...',
    transferring: 'Invio file...',
    receiving: 'Ricezione file...',
    completed: 'Trasferimento completato!',
    error: 'Trasferimento fallito'
  },
  warning: {
    doNotClose: 'Non chiudere questa finestra',
    transferWillFail: 'Il trasferimento verrà annullato se chiudi o aggiorni questa pagina.'
  },
  download: {
    title: 'File multipli in arrivo',
    message: 'Stai per ricevere {count} file. Consentire i download?',
    info: 'Ogni file verrà scaricato nella tua cartella download.',
    allow: 'Consenti',
    deny: 'Annulla'
  },
  confirmation: {
    title: 'Trasferimento file in arrivo',
    message: 'Qualcuno vuole inviarti {count} file ({size} in totale)',
    files: 'File da ricevere',
    warning: 'Accetta solo file da persone fidate. I file verranno scaricati dopo l\'accettazione.',
    accept: 'Accetta trasferimento',
    reject: 'Rifiuta'
  },
  security: {
    title: 'Come Funziona SecureBeam',
    subtitle: 'Comprendi il nostro processo di trasferimento sicuro',
    steps: {
      title: 'Processo di Trasferimento',
      step1: {
        title: 'Genera Codice',
        description: 'Quando selezioni i file, il nostro server genera un codice stanza unico e crittograficamente sicuro (es. A7KN-P3XQ-8FDM). Questo codice ha ~71 bit di entropia, rendendolo virtualmente impossibile da indovinare.'
      },
      step2: {
        title: 'Stabilisci Connessione',
        description: 'Il destinatario inserisce il codice. Entrambi i browser si connettono al nostro server di segnalazione via WebSocket. Il server inoltra solo informazioni di connessione - non vede mai i tuoi file.'
      },
      step3: {
        title: 'Handshake WebRTC',
        description: 'Usando WebRTC, entrambi i browser scambiano dettagli di connessione (candidati ICE, offerte SDP). Viene stabilita una connessione peer-to-peer diretta usando server STUN/TURN per il NAT traversal.'
      },
      step4: {
        title: 'Trasferimento Diretto',
        description: 'I file vengono inviati direttamente dal mittente al destinatario attraverso il canale dati WebRTC crittografato. I dati non passano mai attraverso i nostri server.'
      },
      step5: {
        title: 'Conferma',
        description: 'Il destinatario conferma che ogni file è stato completamente ricevuto. Solo allora il mittente vede "Trasferimento Completato". Questo assicura che nulla vada perso.'
      }
    },
    encryption: {
      title: 'Crittografia e Sicurezza',
      dtls: {
        title: 'Crittografia DTLS',
        description: 'Tutte le connessioni WebRTC usano crittografia DTLS 1.2+. I tuoi dati sono crittografati in transito tra i browser.'
      },
      p2p: {
        title: 'Peer-to-Peer',
        description: 'I file vanno direttamente tra i dispositivi. I nostri server aiutano solo a stabilire la connessione - non memorizzano né vedono mai i tuoi file.'
      },
      codes: {
        title: 'Codici Sicuri',
        description: 'Codici di 12 caratteri con 71 bit di entropia. Generati usando numeri casuali crittograficamente sicuri.'
      },
      noStorage: {
        title: 'Nessuna Memorizzazione',
        description: 'Non memorizziamo i tuoi file, log o cronologia trasferimenti. Una volta completato il trasferimento, tutti i dati sono eliminati.'
      }
    },
    diagram: {
      title: 'Panoramica Architettura',
      sender: 'Mittente',
      receiver: 'Destinatario',
      signaling: 'Server di Segnalazione',
      signalingDesc: 'Configurazione connessione WebRTC una tantum',
      turn: 'Server TURN',
      directConnection: 'Connessione P2P Diretta',
      directConnectionDesc: 'Trasferimento file crittografato'
    }
  },
  footer: {
    encrypted: 'Crittografia end-to-end',
    noLimit: 'Nessun limite di dimensione',
    p2p: 'Peer-to-peer',
    transfers: 'trasferimenti completati',
    impressum: 'Note Legali',
    privacy: 'Privacy',
    legal: 'Note Legali'
  },
  legal: {
    overview: {
      title: 'Note Legali',
      subtitle: 'Tutte le informazioni legali importanti su SecureBeam',
      impressumDesc: 'Informazioni sull\'operatore e dettagli legali in conformità con la legge sulle telecomunicazioni.',
      privacyDesc: 'Informazioni su quali dati raccogliamo e come li utilizziamo.',
      readMore: 'Leggi di più'
    },
    impressum: {
      title: 'Note Legali',
      according: 'Informazioni ai sensi del § 5 TMG (Legge tedesca sui media telematici)',
      country: 'Germania',
      contact: 'Contatto',
      email: 'E-mail',
      responsible: 'Responsabile del contenuto ai sensi del § 55 Abs. 2 RStV',
      dispute: 'Risoluzione delle controversie UE',
      disputeText: 'La Commissione Europea mette a disposizione una piattaforma per la risoluzione delle controversie online (ODR):',
      disputeNote: 'Non siamo disposti né obbligati a partecipare a procedure di risoluzione delle controversie davanti a un organismo di conciliazione dei consumatori.',
      disclaimer: 'Esclusione di responsabilità',
      disclaimerText: 'SecureBeam è un servizio di trasferimento file peer-to-peer. Non assumiamo alcuna responsabilità per i contenuti trasferiti, poiché vengono scambiati direttamente tra gli utenti e non passano mai attraverso i nostri server. L\'uso è a proprio rischio.'
    },
    privacy: {
      title: 'Informativa sulla Privacy',
      intro: {
        title: 'Introduzione',
        text: 'La protezione dei tuoi dati personali è importante per noi. Questa informativa sulla privacy ti informa su quali dati raccogliamo e come li utilizziamo. SecureBeam è stato sviluppato con il principio della minimizzazione dei dati – raccogliamo solo i dati assolutamente necessari per il funzionamento tecnico.'
      },
      controller: {
        title: 'Titolare del trattamento'
      },
      data: {
        title: 'Quali dati raccogliamo',
        server: {
          title: 'Dati di log del server',
          text: 'Quando utilizzi il nostro sito web, i seguenti dati vengono elaborati automaticamente in modo temporaneo:',
          ip: 'Indirizzo IP (anonimizzato dopo 24 ore)',
          time: 'Ora di accesso',
          browser: 'Tipo e versione del browser',
          basis: 'Base giuridica: Art. 6(1)(f) GDPR (interesse legittimo per il funzionamento sicuro del sito web).'
        },
        webrtc: {
          title: 'Dati di connessione WebRTC',
          text: 'Per stabilire connessioni peer-to-peer, le informazioni di connessione (candidati ICE) vengono scambiate temporaneamente tramite il nostro server di segnalazione. Questi dati vengono memorizzati solo per la durata dell\'instaurazione della connessione e vengono eliminati immediatamente dopo. I trasferimenti di file effettivi avvengono direttamente tra i browser – non abbiamo accesso ai file trasferiti.'
        },
        stats: {
          title: 'Statistiche anonime',
          text: 'Contiamo il numero di trasferimenti completati con successo. Questa statistica non contiene dati personali – viene incrementato solo un contatore anonimo.'
        }
      },
      cloudflare: {
        title: 'Cloudflare',
        text: 'Utilizziamo Cloudflare, Inc. come Content Delivery Network (CDN) e per la protezione contro gli attacchi. Cloudflare può elaborare indirizzi IP e dati tecnici di accesso come responsabile del trattamento. Cloudflare è certificato nell\'ambito del EU-US Data Privacy Framework.',
        more: 'Per ulteriori informazioni, consulta l\'informativa sulla privacy di Cloudflare:'
      },
      cookies: {
        title: 'Cookie',
        text: 'SecureBeam stesso non utilizza cookie. Tuttavia, Cloudflare può impostare cookie tecnicamente necessari per la sicurezza e la funzionalità del sito web (ad esempio, per la protezione dai bot). Questi cookie non contengono dati personali e servono esclusivamente a scopi tecnici.'
      },
      rights: {
        title: 'I tuoi diritti',
        intro: 'Ai sensi del GDPR, hai i seguenti diritti:',
        access: 'Diritto di accesso ai tuoi dati memorizzati (Art. 15 GDPR)',
        rectification: 'Diritto di rettifica dei dati inesatti (Art. 16 GDPR)',
        erasure: 'Diritto alla cancellazione dei tuoi dati (Art. 17 GDPR)',
        restriction: 'Diritto alla limitazione del trattamento (Art. 18 GDPR)',
        portability: 'Diritto alla portabilità dei dati (Art. 20 GDPR)',
        objection: 'Diritto di opposizione al trattamento (Art. 21 GDPR)',
        complaint: 'Hai anche il diritto di presentare un reclamo presso un\'autorità di controllo per la protezione dei dati se ritieni che il trattamento dei tuoi dati violi il GDPR.'
      },
      security: {
        title: 'Sicurezza dei dati',
        text: 'Adottiamo misure di sicurezza tecniche e organizzative per proteggere i tuoi dati. La connessione al nostro sito web è crittografata SSL/TLS. I trasferimenti di file tra utenti vengono effettuati tramite connessioni WebRTC crittografate (DTLS).'
      },
      changes: {
        title: 'Modifiche a questa informativa sulla privacy',
        text: 'Ci riserviamo il diritto di adattare questa informativa sulla privacy se necessario per conformarci ai requisiti legali modificati o quando vengono apportate modifiche al servizio. La versione attuale può sempre essere trovata su questa pagina.'
      },
      lastUpdate: 'Ultimo aggiornamento'
    }
  }
}
