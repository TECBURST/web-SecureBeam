export default {
  nav: {
    home: 'Strona Główna',
    security: 'Bezpieczeństwo'
  },
  comingSoon: {
    title: 'Przebudowujemy SecureBeam',
    subtitle: 'Przemyślimy transfer plików od podstaw. WebRTC oparte na przeglądarce miało zbyt wiele ograniczeń, więc tworzymy natywne aplikacje dla lepszych doświadczeń.',
    nativeApps: {
      title: 'Natywne Aplikacje',
      description: 'Zaczynając od Windows, tworzymy dedykowane aplikacje oferujące szybsze i bardziej niezawodne transfery.'
    },
    betterProtocol: {
      title: 'Lepszy Protokół',
      description: 'Natywne aplikacje pozwalają nam używać wydajniejszych protokołów bez ograniczeń przeglądarki.'
    },
    stayTuned: 'Bądź na bieżąco. Nadchodzi coś wielkiego.'
  },
  home: {
    title: 'Bezpieczny Transfer Plików',
    subtitle: 'Wysyłaj pliki bezpośrednio na inne urządzenie. Szyfrowanie end-to-end, peer-to-peer, bez limitów rozmiaru.',
    dropzone: {
      title: 'Upuść pliki tutaj lub kliknij, aby przeglądać',
      subtitle: 'Bez limitu rozmiaru plików. Pliki są przesyłane bezpośrednio.'
    },
    or: 'lub',
    receiveButton: 'Mam kod do odbioru plików',
    backButton: 'Powrót do wysyłania',
    codeInput: {
      placeholder: 'Wprowadź kod transferu',
      button: 'Połącz'
    },
    selectedFiles: 'Wybrane Pliki',
    clearAll: 'Wyczyść wszystko',
    sendFiles: 'Wyślij {count} plik(ów)',
    addMore: 'Dodaj więcej plików',
    cancel: 'Anuluj Transfer',
    complete: {
      title: 'Transfer Zakończony',
      message: '{count} plik(ów) przesłano pomyślnie.'
    },
    error: {
      title: 'Transfer Nieudany'
    },
    newTransfer: 'Nowy Transfer',
    tryAgain: 'Spróbuj Ponownie'
  },
  code: {
    title: 'Twój Kod Transferu',
    subtitle: 'Udostępnij ten kod odbiorcy',
    copied: 'Skopiowano!',
    copy: 'Kopiuj Kod',
    copyLink: 'Kopiuj Link',
    qrHint: 'Lub zeskanuj kod QR',
    qrSubtitle: 'Skanuj aby odebrać',
    linkSubtitle: 'Udostępnij ten link bezpośrednio',
    tabs: {
      code: 'Kod',
      qr: 'Kod QR',
      link: 'Link'
    }
  },
  status: {
    waiting: 'Oczekiwanie na odbiorcę...',
    connecting: 'Łączenie...',
    connected: 'Połączono! Przygotowywanie transferu...',
    awaitingAcceptance: 'Oczekiwanie na akceptację odbiorcy...',
    transferring: 'Wysyłanie plików...',
    receiving: 'Odbieranie plików...',
    completed: 'Transfer zakończony!',
    error: 'Transfer nieudany'
  },
  warning: {
    doNotClose: 'Nie zamykaj tego okna',
    transferWillFail: 'Transfer zostanie anulowany, jeśli zamkniesz lub odświeżysz tę stronę.'
  },
  download: {
    title: 'Wiele plików przychodzących',
    message: 'Otrzymasz {count} plików. Zezwolić na pobieranie?',
    info: 'Każdy plik zostanie pobrany do folderu pobierania.',
    allow: 'Zezwól',
    deny: 'Anuluj'
  },
  confirmation: {
    title: 'Przychodzący transfer plików',
    message: 'Ktoś chce wysłać ci {count} plik(ów) ({size} łącznie)',
    files: 'Pliki do odebrania',
    warning: 'Akceptuj pliki tylko od osób, którym ufasz. Pliki zostaną pobrane po zaakceptowaniu.',
    accept: 'Akceptuj transfer',
    reject: 'Odrzuć'
  },
  security: {
    title: 'Jak Działa SecureBeam',
    subtitle: 'Zrozum nasz bezpieczny proces transferu plików',
    steps: {
      title: 'Proces Transferu',
      step1: {
        title: 'Generowanie Kodu',
        description: 'Po wybraniu plików nasz serwer generuje unikalny, kryptograficznie bezpieczny kod pokoju (np. A7KN-P3XQ-8FDM). Ten kod ma ~71 bitów entropii, co czyni go praktycznie niemożliwym do odgadnięcia.'
      },
      step2: {
        title: 'Nawiązywanie Połączenia',
        description: 'Odbiorca wprowadza kod. Obie przeglądarki łączą się z naszym serwerem sygnalizacyjnym przez WebSocket. Serwer przekazuje tylko informacje o połączeniu - nigdy nie widzi twoich plików.'
      },
      step3: {
        title: 'Handshake WebRTC',
        description: 'Poprzez WebRTC obie przeglądarki wymieniają szczegóły połączenia (kandydaci ICE, oferty SDP). Bezpośrednie połączenie peer-to-peer jest nawiązywane za pomocą serwerów STUN/TURN do przechodzenia NAT.'
      },
      step4: {
        title: 'Bezpośredni Transfer',
        description: 'Pliki są wysyłane bezpośrednio od nadawcy do odbiorcy przez zaszyfrowany kanał danych WebRTC. Dane nigdy nie przechodzą przez nasze serwery.'
      },
      step5: {
        title: 'Potwierdzenie',
        description: 'Odbiorca potwierdza, że każdy plik został w pełni odebrany. Dopiero wtedy nadawca widzi "Transfer Zakończony". To zapewnia, że nic nie zostanie utracone.'
      }
    },
    encryption: {
      title: 'Szyfrowanie i Bezpieczeństwo',
      dtls: {
        title: 'Szyfrowanie DTLS',
        description: 'Wszystkie połączenia WebRTC używają szyfrowania DTLS 1.2+. Twoje dane są szyfrowane podczas przesyłania między przeglądarkami.'
      },
      p2p: {
        title: 'Peer-to-Peer',
        description: 'Pliki przechodzą bezpośrednio między urządzeniami. Nasze serwery pomagają tylko nawiązać połączenie - nigdy nie przechowują ani nie widzą twoich plików.'
      },
      codes: {
        title: 'Bezpieczne Kody',
        description: '12-znakowe kody z 71 bitami entropii. Generowane przy użyciu kryptograficznie bezpiecznych liczb losowych.'
      },
      noStorage: {
        title: 'Brak Przechowywania',
        description: 'Nie przechowujemy twoich plików, logów ani historii transferów. Po zakończeniu transferu wszystkie dane znikają.'
      }
    },
    diagram: {
      title: 'Przegląd Architektury',
      sender: 'Nadawca',
      receiver: 'Odbiorca',
      signaling: 'Serwer Sygnalizacyjny',
      signalingDesc: 'Jednorazowa konfiguracja połączenia WebRTC',
      turn: 'Serwer TURN',
      directConnection: 'Bezpośrednie Połączenie P2P',
      directConnectionDesc: 'Zaszyfrowany transfer plików'
    }
  },
  footer: {
    encrypted: 'Szyfrowanie end-to-end',
    noLimit: 'Bez limitu rozmiaru plików',
    p2p: 'Peer-to-peer',
    transfers: 'transferów zakończonych',
    impressum: 'Informacje Prawne',
    privacy: 'Polityka Prywatności',
    legal: 'Informacje Prawne'
  },
  legal: {
    overview: {
      title: 'Informacje Prawne',
      subtitle: 'Wszystkie ważne informacje prawne o SecureBeam',
      impressumDesc: 'Informacje o operatorze i dane prawne zgodnie z ustawą o mediach elektronicznych.',
      privacyDesc: 'Informacje o tym, jakie dane zbieramy i jak je wykorzystujemy.',
      readMore: 'Czytaj więcej'
    },
    impressum: {
      title: 'Informacje Prawne',
      according: 'Informacje zgodnie z § 5 TMG (niemiecka ustawa o mediach elektronicznych)',
      country: 'Niemcy',
      contact: 'Kontakt',
      email: 'E-mail',
      responsible: 'Odpowiedzialny za treść zgodnie z § 55 Abs. 2 RStV',
      dispute: 'Rozstrzyganie sporów UE',
      disputeText: 'Komisja Europejska udostępnia platformę do internetowego rozstrzygania sporów (ODR):',
      disputeNote: 'Nie jesteśmy skłonni ani zobowiązani do uczestnictwa w postępowaniach rozstrzygania sporów przed komisją arbitrażową dla konsumentów.',
      disclaimer: 'Wyłączenie odpowiedzialności',
      disclaimerText: 'SecureBeam jest usługą transferu plików peer-to-peer. Nie ponosimy odpowiedzialności za przesyłane treści, ponieważ są one wymieniane bezpośrednio między użytkownikami i nigdy nie przechodzą przez nasze serwery. Korzystanie odbywa się na własne ryzyko.'
    },
    privacy: {
      title: 'Polityka Prywatności',
      intro: {
        title: 'Wstęp',
        text: 'Ochrona Twoich danych osobowych jest dla nas ważna. Niniejsza polityka prywatności informuje o tym, jakie dane zbieramy i jak je wykorzystujemy. SecureBeam został opracowany zgodnie z zasadą minimalizacji danych – zbieramy tylko dane absolutnie niezbędne do technicznego działania.'
      },
      controller: {
        title: 'Administrator danych'
      },
      data: {
        title: 'Jakie dane zbieramy',
        server: {
          title: 'Dane dziennika serwera',
          text: 'Podczas korzystania z naszej strony internetowej następujące dane są automatycznie tymczasowo przetwarzane:',
          ip: 'Adres IP (anonimizowany po 24 godzinach)',
          time: 'Czas dostępu',
          browser: 'Typ i wersja przeglądarki',
          basis: 'Podstawa prawna: Art. 6(1)(f) RODO (uzasadniony interes w bezpiecznym działaniu strony).'
        },
        webrtc: {
          title: 'Dane połączenia WebRTC',
          text: 'W celu nawiązania połączeń peer-to-peer informacje o połączeniu (kandydaci ICE) są tymczasowo wymieniane przez nasz serwer sygnalizacyjny. Dane te są przechowywane tylko przez czas nawiązywania połączenia i są natychmiast usuwane. Rzeczywiste transfery plików odbywają się bezpośrednio między przeglądarkami – nie mamy dostępu do przesyłanych plików.'
        },
        stats: {
          title: 'Anonimowe statystyki',
          text: 'Liczymy liczbę pomyślnie zakończonych transferów. Ta statystyka nie zawiera danych osobowych – zwiększany jest tylko anonimowy licznik.'
        }
      },
      cloudflare: {
        title: 'Cloudflare',
        text: 'Korzystamy z Cloudflare, Inc. jako sieci dostarczania treści (CDN) oraz do ochrony przed atakami. Cloudflare może przetwarzać adresy IP i techniczne dane dostępowe jako podmiot przetwarzający. Cloudflare jest certyfikowany w ramach EU-US Data Privacy Framework.',
        more: 'Więcej informacji znajdziesz w polityce prywatności Cloudflare:'
      },
      cookies: {
        title: 'Pliki cookie',
        text: 'SecureBeam sam nie używa plików cookie. Cloudflare może jednak ustawiać technicznie niezbędne pliki cookie wymagane dla bezpieczeństwa i funkcjonalności strony (np. do ochrony przed botami). Te pliki cookie nie zawierają danych osobowych i służą wyłącznie celom technicznym.'
      },
      rights: {
        title: 'Twoje prawa',
        intro: 'Zgodnie z RODO masz następujące prawa:',
        access: 'Prawo dostępu do przechowywanych danych (Art. 15 RODO)',
        rectification: 'Prawo do sprostowania nieprawidłowych danych (Art. 16 RODO)',
        erasure: 'Prawo do usunięcia danych (Art. 17 RODO)',
        restriction: 'Prawo do ograniczenia przetwarzania (Art. 18 RODO)',
        portability: 'Prawo do przenoszenia danych (Art. 20 RODO)',
        objection: 'Prawo do sprzeciwu wobec przetwarzania (Art. 21 RODO)',
        complaint: 'Masz również prawo do złożenia skargi do organu nadzorczego ds. ochrony danych, jeśli uważasz, że przetwarzanie Twoich danych narusza RODO.'
      },
      security: {
        title: 'Bezpieczeństwo danych',
        text: 'Stosujemy techniczne i organizacyjne środki bezpieczeństwa w celu ochrony Twoich danych. Połączenie z naszą stroną jest szyfrowane SSL/TLS. Transfery plików między użytkownikami odbywają się przez szyfrowane połączenia WebRTC (DTLS).'
      },
      changes: {
        title: 'Zmiany w niniejszej polityce prywatności',
        text: 'Zastrzegamy sobie prawo do dostosowania niniejszej polityki prywatności w razie potrzeby, aby dostosować ją do zmienionych wymogów prawnych lub zmian w usłudze. Aktualna wersja jest zawsze dostępna na tej stronie.'
      },
      lastUpdate: 'Ostatnia aktualizacja'
    }
  }
}
