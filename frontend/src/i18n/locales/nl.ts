export default {
  nav: {
    home: 'Home',
    security: 'Beveiliging'
  },
  comingSoon: {
    title: 'We bouwen SecureBeam opnieuw',
    subtitle: 'We herdenken bestandsoverdracht vanaf de grond. Browser-gebaseerd WebRTC had te veel beperkingen, dus bouwen we native apps voor een betere ervaring.',
    nativeApps: {
      title: 'Native Applicaties',
      description: 'Te beginnen met Windows bouwen we speciale apps die snellere en betrouwbaardere overdrachten bieden.'
    },
    betterProtocol: {
      title: 'Beter Protocol',
      description: 'Native apps stellen ons in staat efficiëntere protocollen te gebruiken zonder browserbeperkingen.'
    },
    stayTuned: 'Blijf op de hoogte voor updates. Er komt iets geweldigs aan.'
  },
  home: {
    title: 'Veilige Bestandsoverdracht',
    subtitle: 'Verstuur bestanden rechtstreeks naar een ander apparaat. End-to-end versleuteld, peer-to-peer, geen groottebeperkingen.',
    dropzone: {
      title: 'Sleep bestanden hierheen of klik om te bladeren',
      subtitle: 'Geen bestandsgrootte limiet. Bestanden worden direct overgedragen.'
    },
    or: 'of',
    receiveButton: 'Ik heb een code om bestanden te ontvangen',
    backButton: 'Terug naar verzenden',
    codeInput: {
      placeholder: 'Voer overdrachtscode in',
      button: 'Verbinden'
    },
    selectedFiles: 'Geselecteerde Bestanden',
    clearAll: 'Alles wissen',
    sendFiles: '{count} bestand(en) verzenden',
    addMore: 'Meer bestanden toevoegen',
    cancel: 'Overdracht Annuleren',
    complete: {
      title: 'Overdracht Voltooid',
      message: '{count} bestand(en) succesvol overgedragen.'
    },
    error: {
      title: 'Overdracht Mislukt'
    },
    newTransfer: 'Nieuwe Overdracht',
    tryAgain: 'Opnieuw Proberen'
  },
  code: {
    title: 'Uw Overdrachtscode',
    subtitle: 'Deel deze code met de ontvanger',
    copied: 'Gekopieerd!',
    copy: 'Code Kopiëren',
    copyLink: 'Link Kopiëren',
    qrHint: 'Of scan de QR-code',
    qrSubtitle: 'Scan om te ontvangen',
    linkSubtitle: 'Deel deze link direct',
    tabs: {
      code: 'Code',
      qr: 'QR-code',
      link: 'Link'
    }
  },
  status: {
    waiting: 'Wachten op ontvanger...',
    connecting: 'Verbinden...',
    connected: 'Verbonden! Overdracht voorbereiden...',
    awaitingAcceptance: 'Wachten op acceptatie door ontvanger...',
    transferring: 'Bestanden verzenden...',
    receiving: 'Bestanden ontvangen...',
    completed: 'Overdracht voltooid!',
    error: 'Overdracht mislukt'
  },
  warning: {
    doNotClose: 'Sluit dit venster niet',
    transferWillFail: 'De overdracht wordt geannuleerd als u deze pagina sluit of vernieuwt.'
  },
  downloadDialog: {
    title: 'Meerdere bestanden inkomend',
    message: 'U ontvangt {count} bestanden. Downloads toestaan?',
    info: 'Elk bestand wordt gedownload naar uw standaard downloadmap.',
    allow: 'Toestaan',
    deny: 'Annuleren'
  },
  confirmation: {
    title: 'Inkomende bestandsoverdracht',
    message: 'Iemand wil u {count} bestand(en) sturen ({size} totaal)',
    files: 'Te ontvangen bestanden',
    warning: 'Accepteer alleen bestanden van mensen die u vertrouwt. Bestanden worden gedownload na acceptatie.',
    accept: 'Overdracht accepteren',
    reject: 'Weigeren'
  },
  security: {
    title: 'Hoe SecureBeam Werkt',
    subtitle: 'Begrijp ons veilige bestandsoverdracht proces',
    steps: {
      title: 'Overdrachtsproces',
      step1: {
        title: 'Code Genereren',
        description: 'Wanneer u bestanden selecteert, genereert onze server een unieke, cryptografisch veilige kamercode (bijv. A7KN-P3XQ-8FDM). Deze code heeft ~71 bits entropie, waardoor het vrijwel onmogelijk te raden is.'
      },
      step2: {
        title: 'Verbinding Maken',
        description: 'De ontvanger voert de code in. Beide browsers verbinden met onze signaalserver via WebSocket. De server stuurt alleen verbindingsinformatie door - het ziet nooit uw bestanden.'
      },
      step3: {
        title: 'WebRTC Handshake',
        description: 'Via WebRTC wisselen beide browsers verbindingsdetails uit (ICE-kandidaten, SDP-aanbiedingen). Een directe peer-to-peer verbinding wordt opgezet via STUN/TURN-servers voor NAT-traversal.'
      },
      step4: {
        title: 'Directe Overdracht',
        description: 'Bestanden worden direct van verzender naar ontvanger gestuurd via het versleutelde WebRTC DataChannel. Data passeert nooit onze servers.'
      },
      step5: {
        title: 'Bevestiging',
        description: 'De ontvanger bevestigt dat elk bestand volledig is ontvangen. Pas dan ziet de verzender "Overdracht Voltooid". Dit zorgt ervoor dat niets verloren gaat.'
      }
    },
    encryption: {
      title: 'Versleuteling & Beveiliging',
      dtls: {
        title: 'DTLS Versleuteling',
        description: 'Alle WebRTC-verbindingen gebruiken DTLS 1.2+ versleuteling. Uw data is versleuteld tijdens overdracht tussen browsers.'
      },
      p2p: {
        title: 'Peer-to-Peer',
        description: 'Bestanden gaan direct tussen apparaten. Onze servers helpen alleen bij het opzetten van de verbinding - ze slaan nooit uw bestanden op en zien ze nooit.'
      },
      codes: {
        title: 'Veilige Codes',
        description: '12-karakter codes met 71 bits entropie. Gegenereerd met cryptografisch veilige willekeurige getallen.'
      },
      noStorage: {
        title: 'Geen Opslag',
        description: 'Wij slaan uw bestanden, logs of overdrachtsgeschiedenis niet op. Zodra de overdracht is voltooid, zijn alle gegevens verdwenen.'
      }
    },
    diagram: {
      title: 'Architectuur Overzicht',
      sender: 'Verzender',
      receiver: 'Ontvanger',
      signaling: 'Signaalserver',
      signalingDesc: 'Eenmalige WebRTC-verbinding setup',
      turn: 'TURN Server',
      directConnection: 'Directe P2P Verbinding',
      directConnectionDesc: 'Versleutelde bestandsoverdracht'
    }
  },
  footer: {
    encrypted: 'End-to-end versleuteld',
    noLimit: 'Geen bestandsgrootte limiet',
    p2p: 'Peer-to-peer',
    transfers: 'overdrachten voltooid',
    impressum: 'Juridische Informatie',
    privacy: 'Privacybeleid',
    legal: 'Juridisch'
  },
  legal: {
    overview: {
      title: 'Juridisch',
      subtitle: 'Alle belangrijke juridische informatie over SecureBeam',
      impressumDesc: 'Informatie over de operator en juridische details in overeenstemming met de telecommunicatiewet.',
      privacyDesc: 'Informatie over welke gegevens we verzamelen en hoe we deze gebruiken.',
      readMore: 'Lees meer'
    },
    impressum: {
      title: 'Juridische Informatie',
      according: 'Informatie volgens § 5 TMG (Duitse Telemediawet)',
      country: 'Duitsland',
      contact: 'Contact',
      email: 'E-mail',
      responsible: 'Verantwoordelijk voor de inhoud volgens § 55 Abs. 2 RStV',
      dispute: 'EU-geschillenbeslechting',
      disputeText: 'De Europese Commissie biedt een platform voor online geschillenbeslechting (ODR):',
      disputeNote: 'Wij zijn niet bereid of verplicht om deel te nemen aan geschillenbeslechtingsprocedures voor een consumentenarbitragecommissie.',
      disclaimer: 'Disclaimer',
      disclaimerText: 'SecureBeam is een peer-to-peer bestandsoverdrachtservice. Wij aanvaarden geen aansprakelijkheid voor overgedragen inhoud, aangezien deze rechtstreeks tussen gebruikers wordt uitgewisseld en nooit via onze servers gaat. Gebruik is op eigen risico.'
    },
    privacy: {
      title: 'Privacybeleid',
      intro: {
        title: 'Inleiding',
        text: 'De bescherming van uw persoonlijke gegevens is belangrijk voor ons. Dit privacybeleid informeert u over welke gegevens wij verzamelen en hoe wij deze gebruiken. SecureBeam is ontwikkeld met het principe van dataminimalisatie – wij verzamelen alleen gegevens die absoluut noodzakelijk zijn voor de technische werking.'
      },
      controller: {
        title: 'Verwerkingsverantwoordelijke'
      },
      data: {
        title: 'Welke gegevens wij verzamelen',
        server: {
          title: 'Serverloggegevens',
          text: 'Bij het gebruik van onze website worden de volgende gegevens automatisch tijdelijk verwerkt:',
          ip: 'IP-adres (geanonimiseerd na 24 uur)',
          time: 'Tijdstip van toegang',
          browser: 'Browsertype en -versie',
          basis: 'Rechtsgrondslag: Art. 6(1)(f) AVG (legitiem belang bij veilige websitewerking).'
        },
        webrtc: {
          title: 'WebRTC-verbindingsgegevens',
          text: 'Voor het opzetten van peer-to-peer verbindingen wordt verbindingsinformatie (ICE-kandidaten) tijdelijk uitgewisseld via onze signaalserver. Deze gegevens worden alleen opgeslagen voor de duur van het verbindingsproces en worden daarna onmiddellijk verwijderd. Daadwerkelijke bestandsoverdrachten vinden rechtstreeks plaats tussen browsers – wij hebben geen toegang tot de overgedragen bestanden.'
        },
        stats: {
          title: 'Anonieme statistieken',
          text: 'Wij tellen het aantal succesvol voltooide overdrachten. Deze statistiek bevat geen persoonlijke gegevens – alleen een anonieme teller wordt verhoogd.'
        }
      },
      cloudflare: {
        title: 'Cloudflare',
        text: 'Wij gebruiken Cloudflare, Inc. als Content Delivery Network (CDN) en voor bescherming tegen aanvallen. Cloudflare kan IP-adressen en technische toegangsgegevens verwerken als gegevensverwerker. Cloudflare is gecertificeerd onder het EU-US Data Privacy Framework.',
        more: 'Voor meer informatie, zie het privacybeleid van Cloudflare:'
      },
      cookies: {
        title: 'Cookies',
        text: 'SecureBeam zelf gebruikt geen cookies. Cloudflare kan echter technisch noodzakelijke cookies plaatsen die vereist zijn voor de veiligheid en functionaliteit van de website (bijv. voor botbescherming). Deze cookies bevatten geen persoonlijke gegevens en dienen uitsluitend technische doeleinden.'
      },
      rights: {
        title: 'Uw rechten',
        intro: 'Op grond van de AVG hebt u de volgende rechten:',
        access: 'Recht op inzage in uw opgeslagen gegevens (Art. 15 AVG)',
        rectification: 'Recht op rectificatie van onjuiste gegevens (Art. 16 AVG)',
        erasure: 'Recht op wissing van uw gegevens (Art. 17 AVG)',
        restriction: 'Recht op beperking van de verwerking (Art. 18 AVG)',
        portability: 'Recht op gegevensoverdraagbaarheid (Art. 20 AVG)',
        objection: 'Recht van bezwaar tegen de verwerking (Art. 21 AVG)',
        complaint: 'U hebt ook het recht om een klacht in te dienen bij een toezichthoudende autoriteit voor gegevensbescherming als u van mening bent dat de verwerking van uw gegevens in strijd is met de AVG.'
      },
      security: {
        title: 'Gegevensbeveiliging',
        text: 'Wij passen technische en organisatorische beveiligingsmaatregelen toe om uw gegevens te beschermen. De verbinding met onze website is SSL/TLS-versleuteld. Bestandsoverdrachten tussen gebruikers worden uitgevoerd via versleutelde WebRTC-verbindingen (DTLS).'
      },
      changes: {
        title: 'Wijzigingen in dit privacybeleid',
        text: 'Wij behouden ons het recht voor om dit privacybeleid indien nodig aan te passen om te voldoen aan gewijzigde wettelijke vereisten of wanneer wijzigingen aan de service worden aangebracht. De huidige versie is altijd te vinden op deze pagina.'
      },
      lastUpdate: 'Laatst bijgewerkt'
    }
  }
}
