export default {
  nav: {
    home: 'Start',
    security: 'Sicherheit'
  },
  comingSoon: {
    title: 'Wir bauen SecureBeam neu',
    subtitle: 'Wir überdenken Dateitransfer von Grund auf. Browser-basiertes WebRTC hatte zu viele Einschränkungen, daher entwickeln wir native Apps für ein besseres Erlebnis.',
    nativeApps: {
      title: 'Native Anwendungen',
      description: 'Beginnend mit Windows entwickeln wir dedizierte Apps, die schnellere und zuverlässigere Transfers ermöglichen.'
    },
    betterProtocol: {
      title: 'Besseres Protokoll',
      description: 'Native Apps ermöglichen uns die Nutzung effizienterer Protokolle ohne Browser-Einschränkungen.'
    },
    stayTuned: 'Bleib dran für Updates. Etwas Großartiges kommt.'
  },
  home: {
    title: 'Sicherer Dateitransfer',
    subtitle: 'Sende Dateien direkt an ein anderes Gerät. Ende-zu-Ende verschlüsselt, Peer-to-Peer, keine Größenbeschränkung.',
    dropzone: {
      title: 'Dateien hier ablegen oder klicken zum Auswählen',
      subtitle: 'Keine Größenbeschränkung. Dateien werden direkt übertragen.'
    },
    or: 'oder',
    receiveButton: 'Ich habe einen Code zum Empfangen',
    backButton: 'Zurück zum Senden',
    codeInput: {
      placeholder: 'Transfer-Code eingeben',
      button: 'Verbinden'
    },
    selectedFiles: 'Ausgewählte Dateien',
    clearAll: 'Alle entfernen',
    sendFiles: '{count} Datei(en) senden',
    addMore: 'Weitere Dateien hinzufügen',
    cancel: 'Transfer abbrechen',
    complete: {
      title: 'Transfer abgeschlossen',
      message: '{count} Datei(en) erfolgreich übertragen.'
    },
    error: {
      title: 'Transfer fehlgeschlagen'
    },
    newTransfer: 'Neuen Transfer starten',
    tryAgain: 'Erneut versuchen'
  },
  code: {
    title: 'Dein Transfer-Code',
    subtitle: 'Teile diesen Code mit dem Empfänger',
    copied: 'Kopiert!',
    copy: 'Code kopieren',
    copyLink: 'Link kopieren',
    qrHint: 'Oder QR-Code scannen',
    qrSubtitle: 'Scannen zum Empfangen',
    linkSubtitle: 'Diesen Link direkt teilen',
    tabs: {
      code: 'Code',
      qr: 'QR-Code',
      link: 'Link'
    }
  },
  status: {
    waiting: 'Warte auf Empfänger...',
    connecting: 'Verbinde...',
    connected: 'Verbunden! Transfer wird vorbereitet...',
    awaitingAcceptance: 'Warte auf Bestätigung des Empfängers...',
    transferring: 'Sende Dateien...',
    receiving: 'Empfange Dateien...',
    completed: 'Transfer abgeschlossen!',
    error: 'Transfer fehlgeschlagen'
  },
  warning: {
    doNotClose: 'Dieses Fenster nicht schließen',
    transferWillFail: 'Die Übertragung wird abgebrochen, wenn Sie diese Seite schließen oder aktualisieren.'
  },
  download: {
    title: 'Mehrere Dateien eingehend',
    message: 'Du erhältst gleich {count} Dateien. Downloads erlauben?',
    info: 'Jede Datei wird in deinem Standard-Download-Ordner gespeichert.',
    allow: 'Downloads erlauben',
    deny: 'Abbrechen'
  },
  confirmation: {
    title: 'Eingehender Dateitransfer',
    message: 'Jemand möchte dir {count} Datei(en) senden ({size} gesamt)',
    files: 'Zu empfangende Dateien',
    warning: 'Akzeptiere nur Dateien von Personen, denen du vertraust. Die Dateien werden nach dem Akzeptieren auf dein Gerät heruntergeladen.',
    accept: 'Transfer akzeptieren',
    reject: 'Ablehnen'
  },
  security: {
    title: 'Wie SecureBeam funktioniert',
    subtitle: 'Verstehe unseren sicheren Dateitransfer-Prozess',
    steps: {
      title: 'Transfer-Prozess',
      step1: {
        title: 'Code generieren',
        description: 'Wenn du Dateien auswählst, generiert unser Server einen einzigartigen, kryptographisch sicheren Raum-Code (z.B. A7KN-P3XQ-8FDM). Dieser Code hat ~71 Bits Entropie und ist praktisch unmöglich zu erraten.'
      },
      step2: {
        title: 'Verbindung herstellen',
        description: 'Der Empfänger gibt den Code ein. Beide Browser verbinden sich über WebSocket mit unserem Signalisierungsserver. Der Server leitet nur Verbindungsinformationen weiter - er sieht niemals deine Dateien.'
      },
      step3: {
        title: 'WebRTC Handshake',
        description: 'Mittels WebRTC tauschen beide Browser Verbindungsdetails aus (ICE-Kandidaten, SDP-Angebote). Eine direkte Peer-to-Peer-Verbindung wird über STUN/TURN-Server für NAT-Traversal hergestellt.'
      },
      step4: {
        title: 'Direkter Transfer',
        description: 'Dateien werden direkt vom Sender zum Empfänger über den verschlüsselten WebRTC DataChannel gesendet. Daten passieren niemals unsere Server.'
      },
      step5: {
        title: 'Bestätigung',
        description: 'Der Empfänger bestätigt, dass jede Datei vollständig empfangen wurde. Erst dann sieht der Sender "Transfer abgeschlossen". So geht nichts verloren.'
      }
    },
    encryption: {
      title: 'Verschlüsselung & Sicherheit',
      dtls: {
        title: 'DTLS-Verschlüsselung',
        description: 'Alle WebRTC-Verbindungen nutzen DTLS 1.2+ Verschlüsselung. Deine Daten sind während der Übertragung verschlüsselt.'
      },
      p2p: {
        title: 'Peer-to-Peer',
        description: 'Dateien gehen direkt zwischen Geräten. Unsere Server helfen nur beim Verbindungsaufbau - sie speichern oder sehen niemals deine Dateien.'
      },
      codes: {
        title: 'Sichere Codes',
        description: '12-Zeichen-Codes mit 71 Bits Entropie. Generiert mit kryptographisch sicheren Zufallszahlen.'
      },
      noStorage: {
        title: 'Keine Speicherung',
        description: 'Wir speichern keine Dateien, Logs oder Transfer-Verläufe. Nach dem Transfer sind alle Daten weg.'
      }
    },
    diagram: {
      title: 'Architektur-Übersicht',
      sender: 'Sender',
      receiver: 'Empfänger',
      signaling: 'Signalisierungsserver',
      signalingDesc: 'Einmalige WebRTC-Verbindungsherstellung',
      turn: 'TURN Server',
      directConnection: 'Direkte P2P-Verbindung',
      directConnectionDesc: 'Verschlüsselter Dateitransfer'
    }
  },
  footer: {
    encrypted: 'Ende-zu-Ende verschlüsselt',
    noLimit: 'Keine Größenbeschränkung',
    p2p: 'Peer-to-Peer',
    transfers: 'Transfers abgeschlossen',
    impressum: 'Impressum',
    privacy: 'Datenschutz',
    legal: 'Rechtliches'
  },
  legal: {
    overview: {
      title: 'Rechtliches',
      subtitle: 'Alle wichtigen rechtlichen Informationen zu SecureBeam',
      impressumDesc: 'Informationen zum Betreiber und rechtliche Angaben gemäß Telemediengesetz.',
      privacyDesc: 'Informationen darüber, welche Daten wir erheben und wie wir sie verwenden.',
      readMore: 'Weiterlesen'
    },
    impressum: {
      title: 'Impressum',
      according: 'Angaben gemäß § 5 TMG',
      country: 'Deutschland',
      contact: 'Kontakt',
      email: 'E-Mail',
      responsible: 'Verantwortlich für den Inhalt nach § 55 Abs. 2 RStV',
      dispute: 'EU-Streitschlichtung',
      disputeText: 'Die Europäische Kommission stellt eine Plattform zur Online-Streitbeilegung (OS) bereit:',
      disputeNote: 'Wir sind nicht bereit oder verpflichtet, an Streitbeilegungsverfahren vor einer Verbraucherschlichtungsstelle teilzunehmen.',
      disclaimer: 'Haftungsausschluss',
      disclaimerText: 'SecureBeam ist ein Peer-to-Peer Dateitransfer-Dienst. Wir übernehmen keine Haftung für die übertragenen Inhalte, da diese direkt zwischen den Nutzern ausgetauscht werden und niemals unsere Server passieren. Die Nutzung erfolgt auf eigene Verantwortung.'
    },
    privacy: {
      title: 'Datenschutzerklärung',
      intro: {
        title: 'Einleitung',
        text: 'Der Schutz Ihrer persönlichen Daten ist uns wichtig. Diese Datenschutzerklärung informiert Sie darüber, welche Daten wir erheben und wie wir diese verwenden. SecureBeam wurde mit dem Prinzip der Datensparsamkeit entwickelt – wir erheben nur die Daten, die für den technischen Betrieb unbedingt erforderlich sind.'
      },
      controller: {
        title: 'Verantwortlicher'
      },
      data: {
        title: 'Welche Daten wir erheben',
        server: {
          title: 'Server-Logdaten',
          text: 'Bei der Nutzung unserer Website werden automatisch folgende Daten temporär verarbeitet:',
          ip: 'IP-Adresse (anonymisiert nach 24 Stunden)',
          time: 'Zeitpunkt des Zugriffs',
          browser: 'Browser-Typ und Version',
          basis: 'Rechtsgrundlage: Art. 6 Abs. 1 lit. f DSGVO (berechtigtes Interesse am sicheren Betrieb der Website).'
        },
        webrtc: {
          title: 'WebRTC-Verbindungsdaten',
          text: 'Für den Aufbau der Peer-to-Peer-Verbindung werden temporär Verbindungsinformationen (ICE-Kandidaten) über unseren Signalisierungsserver ausgetauscht. Diese Daten werden nur für die Dauer der Verbindungsherstellung gespeichert und danach sofort gelöscht. Die eigentlichen Dateiübertragungen erfolgen direkt zwischen den Browsern – wir haben keinen Zugriff auf die übertragenen Dateien.'
        },
        stats: {
          title: 'Anonyme Statistiken',
          text: 'Wir zählen die Anzahl der erfolgreich abgeschlossenen Transfers. Diese Statistik enthält keine personenbezogenen Daten – es wird lediglich ein anonymer Zähler erhöht.'
        }
      },
      cloudflare: {
        title: 'Cloudflare',
        text: 'Wir nutzen Cloudflare, Inc. als Content Delivery Network (CDN) und zum Schutz vor Angriffen. Cloudflare kann dabei als Auftragsverarbeiter IP-Adressen und technische Zugriffsdaten verarbeiten. Cloudflare ist nach dem EU-US Data Privacy Framework zertifiziert.',
        more: 'Weitere Informationen finden Sie in der Datenschutzerklärung von Cloudflare:'
      },
      cookies: {
        title: 'Cookies',
        text: 'SecureBeam selbst verwendet keine Cookies. Cloudflare kann jedoch technisch notwendige Cookies setzen, die für die Sicherheit und Funktionalität der Website erforderlich sind (z.B. zum Schutz vor Bots). Diese Cookies enthalten keine persönlichen Daten und dienen ausschließlich technischen Zwecken.'
      },
      rights: {
        title: 'Ihre Rechte',
        intro: 'Nach der DSGVO haben Sie folgende Rechte:',
        access: 'Recht auf Auskunft über Ihre gespeicherten Daten (Art. 15 DSGVO)',
        rectification: 'Recht auf Berichtigung unrichtiger Daten (Art. 16 DSGVO)',
        erasure: 'Recht auf Löschung Ihrer Daten (Art. 17 DSGVO)',
        restriction: 'Recht auf Einschränkung der Verarbeitung (Art. 18 DSGVO)',
        portability: 'Recht auf Datenübertragbarkeit (Art. 20 DSGVO)',
        objection: 'Widerspruchsrecht gegen die Verarbeitung (Art. 21 DSGVO)',
        complaint: 'Sie haben außerdem das Recht, sich bei einer Datenschutz-Aufsichtsbehörde zu beschweren, wenn Sie der Meinung sind, dass die Verarbeitung Ihrer Daten gegen die DSGVO verstößt.'
      },
      security: {
        title: 'Datensicherheit',
        text: 'Wir setzen technische und organisatorische Sicherheitsmaßnahmen ein, um Ihre Daten zu schützen. Die Verbindung zu unserer Website ist SSL/TLS-verschlüsselt. Dateiübertragungen zwischen Nutzern erfolgen über verschlüsselte WebRTC-Verbindungen (DTLS).'
      },
      changes: {
        title: 'Änderungen dieser Datenschutzerklärung',
        text: 'Wir behalten uns vor, diese Datenschutzerklärung bei Bedarf anzupassen, um sie an geänderte Rechtslage oder bei Änderungen des Dienstes anzupassen. Die aktuelle Version finden Sie immer auf dieser Seite.'
      },
      lastUpdate: 'Stand'
    }
  }
}
