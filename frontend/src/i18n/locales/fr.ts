export default {
  nav: {
    home: 'Accueil',
    security: 'Sécurité'
  },
  comingSoon: {
    title: 'Nous reconstruisons SecureBeam',
    subtitle: 'Nous repensons le transfert de fichiers depuis le début. Le WebRTC basé sur navigateur avait trop de limitations, nous développons donc des applications natives pour une meilleure expérience.',
    nativeApps: {
      title: 'Applications Natives',
      description: 'À commencer par Windows, nous développons des applications dédiées offrant des transferts plus rapides et fiables.'
    },
    betterProtocol: {
      title: 'Meilleur Protocole',
      description: 'Les applications natives nous permettent d\'utiliser des protocoles plus efficaces sans les limitations du navigateur.'
    },
    stayTuned: 'Restez à l\'écoute pour les mises à jour. Quelque chose de génial arrive.'
  },
  home: {
    title: 'Transfert de Fichiers Sécurisé',
    subtitle: 'Envoyez des fichiers directement vers un autre appareil. Chiffrement de bout en bout, peer-to-peer, sans limite de taille.',
    dropzone: {
      title: 'Déposez les fichiers ici ou cliquez pour parcourir',
      subtitle: 'Pas de limite de taille. Les fichiers sont transférés directement.'
    },
    or: 'ou',
    receiveButton: 'J\'ai un code pour recevoir des fichiers',
    backButton: 'Retour à l\'envoi',
    codeInput: {
      placeholder: 'Entrez le code de transfert',
      button: 'Connecter'
    },
    selectedFiles: 'Fichiers Sélectionnés',
    clearAll: 'Tout effacer',
    sendFiles: 'Envoyer {count} fichier(s)',
    addMore: 'Ajouter des fichiers',
    cancel: 'Annuler le Transfert',
    complete: {
      title: 'Transfert Terminé',
      message: '{count} fichier(s) transféré(s) avec succès.'
    },
    error: {
      title: 'Échec du Transfert'
    },
    newTransfer: 'Nouveau Transfert',
    tryAgain: 'Réessayer'
  },
  code: {
    title: 'Votre Code de Transfert',
    subtitle: 'Partagez ce code avec le destinataire',
    copied: 'Copié !',
    copy: 'Copier le Code',
    copyLink: 'Copier le Lien',
    qrHint: 'Ou scannez le code QR',
    qrSubtitle: 'Scannez pour recevoir',
    linkSubtitle: 'Partagez ce lien directement',
    tabs: {
      code: 'Code',
      qr: 'QR Code',
      link: 'Lien'
    }
  },
  status: {
    waiting: 'En attente du destinataire...',
    connecting: 'Connexion en cours...',
    connected: 'Connecté ! Préparation du transfert...',
    awaitingAcceptance: 'En attente de l\'acceptation du destinataire...',
    transferring: 'Envoi des fichiers...',
    receiving: 'Réception des fichiers...',
    completed: 'Transfert terminé !',
    error: 'Échec du transfert'
  },
  warning: {
    doNotClose: 'Ne fermez pas cette fenêtre',
    transferWillFail: 'Le transfert sera annulé si vous fermez ou actualisez cette page.'
  },
  downloadDialog: {
    title: 'Fichiers multiples entrants',
    message: 'Vous allez recevoir {count} fichiers. Autoriser les téléchargements ?',
    info: 'Chaque fichier sera téléchargé dans votre dossier de téléchargements.',
    allow: 'Autoriser',
    deny: 'Annuler'
  },
  confirmation: {
    title: 'Transfert de fichiers entrant',
    message: 'Quelqu\'un veut vous envoyer {count} fichier(s) ({size} au total)',
    files: 'Fichiers à recevoir',
    warning: 'N\'acceptez que les fichiers de personnes de confiance. Les fichiers seront téléchargés après acceptation.',
    accept: 'Accepter le transfert',
    reject: 'Refuser'
  },
  security: {
    title: 'Comment Fonctionne SecureBeam',
    subtitle: 'Comprendre notre processus de transfert sécurisé',
    steps: {
      title: 'Processus de Transfert',
      step1: {
        title: 'Générer le Code',
        description: 'Lorsque vous sélectionnez des fichiers, notre serveur génère un code de salle unique et cryptographiquement sécurisé (ex. A7KN-P3XQ-8FDM). Ce code possède ~71 bits d\'entropie, le rendant pratiquement impossible à deviner.'
      },
      step2: {
        title: 'Établir la Connexion',
        description: 'Le destinataire entre le code. Les deux navigateurs se connectent à notre serveur de signalisation via WebSocket. Le serveur ne relaye que les informations de connexion - il ne voit jamais vos fichiers.'
      },
      step3: {
        title: 'Handshake WebRTC',
        description: 'Via WebRTC, les deux navigateurs échangent les détails de connexion (candidats ICE, offres SDP). Une connexion peer-to-peer directe est établie via les serveurs STUN/TURN pour la traversée NAT.'
      },
      step4: {
        title: 'Transfert Direct',
        description: 'Les fichiers sont envoyés directement de l\'expéditeur au destinataire via le canal de données WebRTC chiffré. Les données ne passent jamais par nos serveurs.'
      },
      step5: {
        title: 'Confirmation',
        description: 'Le destinataire confirme que chaque fichier a été entièrement reçu. Ce n\'est qu\'alors que l\'expéditeur voit "Transfert Terminé". Cela garantit que rien n\'est perdu.'
      }
    },
    encryption: {
      title: 'Chiffrement et Sécurité',
      dtls: {
        title: 'Chiffrement DTLS',
        description: 'Toutes les connexions WebRTC utilisent le chiffrement DTLS 1.2+. Vos données sont chiffrées en transit entre les navigateurs.'
      },
      p2p: {
        title: 'Peer-to-Peer',
        description: 'Les fichiers vont directement entre les appareils. Nos serveurs aident uniquement à établir la connexion - ils ne stockent ni ne voient jamais vos fichiers.'
      },
      codes: {
        title: 'Codes Sécurisés',
        description: 'Codes de 12 caractères avec 71 bits d\'entropie. Générés avec des nombres aléatoires cryptographiquement sécurisés.'
      },
      noStorage: {
        title: 'Aucun Stockage',
        description: 'Nous ne stockons pas vos fichiers, journaux ou historique de transfert. Une fois le transfert terminé, toutes les données disparaissent.'
      }
    },
    diagram: {
      title: 'Aperçu de l\'Architecture',
      sender: 'Expéditeur',
      receiver: 'Destinataire',
      signaling: 'Serveur de Signalisation',
      signalingDesc: 'Configuration unique de connexion WebRTC',
      turn: 'Serveur TURN',
      directConnection: 'Connexion P2P Directe',
      directConnectionDesc: 'Transfert de fichiers chiffré'
    }
  },
  footer: {
    encrypted: 'Chiffrement de bout en bout',
    noLimit: 'Pas de limite de taille',
    p2p: 'Peer-to-peer',
    transfers: 'transferts effectués',
    impressum: 'Mentions Légales',
    privacy: 'Confidentialité',
    legal: 'Mentions Légales'
  },
  legal: {
    overview: {
      title: 'Mentions Légales',
      subtitle: 'Toutes les informations juridiques importantes concernant SecureBeam',
      impressumDesc: 'Informations sur l\'opérateur et mentions légales conformément à la loi sur les télémédias.',
      privacyDesc: 'Informations sur les données que nous collectons et comment nous les utilisons.',
      readMore: 'En savoir plus'
    },
    impressum: {
      title: 'Mentions Légales',
      according: 'Informations conformément au § 5 TMG (loi allemande sur les télémédias)',
      country: 'Allemagne',
      contact: 'Contact',
      email: 'E-mail',
      responsible: 'Responsable du contenu conformément au § 55 Abs. 2 RStV',
      dispute: 'Règlement des litiges UE',
      disputeText: 'La Commission européenne met à disposition une plateforme de règlement en ligne des litiges (RLL) :',
      disputeNote: 'Nous ne sommes pas disposés ni obligés de participer à des procédures de règlement des litiges devant un organisme de médiation des consommateurs.',
      disclaimer: 'Avertissement',
      disclaimerText: 'SecureBeam est un service de transfert de fichiers peer-to-peer. Nous n\'assumons aucune responsabilité pour le contenu transféré, car il est échangé directement entre les utilisateurs et ne passe jamais par nos serveurs. L\'utilisation se fait à vos propres risques.'
    },
    privacy: {
      title: 'Politique de Confidentialité',
      intro: {
        title: 'Introduction',
        text: 'La protection de vos données personnelles est importante pour nous. Cette politique de confidentialité vous informe sur les données que nous collectons et comment nous les utilisons. SecureBeam a été développé selon le principe de minimisation des données – nous ne collectons que les données absolument nécessaires au fonctionnement technique.'
      },
      controller: {
        title: 'Responsable du traitement'
      },
      data: {
        title: 'Quelles données nous collectons',
        server: {
          title: 'Données de journal serveur',
          text: 'Lors de l\'utilisation de notre site web, les données suivantes sont automatiquement traitées temporairement :',
          ip: 'Adresse IP (anonymisée après 24 heures)',
          time: 'Heure d\'accès',
          browser: 'Type et version du navigateur',
          basis: 'Base juridique : Art. 6(1)(f) RGPD (intérêt légitime pour le fonctionnement sécurisé du site web).'
        },
        webrtc: {
          title: 'Données de connexion WebRTC',
          text: 'Pour établir des connexions peer-to-peer, les informations de connexion (candidats ICE) sont temporairement échangées via notre serveur de signalisation. Ces données ne sont stockées que pendant la durée de l\'établissement de la connexion et sont supprimées immédiatement après. Les transferts de fichiers réels se font directement entre les navigateurs – nous n\'avons pas accès aux fichiers transférés.'
        },
        stats: {
          title: 'Statistiques anonymes',
          text: 'Nous comptons le nombre de transferts réussis. Cette statistique ne contient aucune donnée personnelle – seul un compteur anonyme est incrémenté.'
        }
      },
      cloudflare: {
        title: 'Cloudflare',
        text: 'Nous utilisons Cloudflare, Inc. comme réseau de diffusion de contenu (CDN) et pour la protection contre les attaques. Cloudflare peut traiter des adresses IP et des données d\'accès techniques en tant que sous-traitant. Cloudflare est certifié dans le cadre du EU-US Data Privacy Framework.',
        more: 'Pour plus d\'informations, consultez la politique de confidentialité de Cloudflare :'
      },
      cookies: {
        title: 'Cookies',
        text: 'SecureBeam lui-même n\'utilise pas de cookies. Cependant, Cloudflare peut définir des cookies techniquement nécessaires pour la sécurité et la fonctionnalité du site web (par exemple, pour la protection contre les bots). Ces cookies ne contiennent pas de données personnelles et servent exclusivement à des fins techniques.'
      },
      rights: {
        title: 'Vos droits',
        intro: 'En vertu du RGPD, vous disposez des droits suivants :',
        access: 'Droit d\'accès à vos données stockées (Art. 15 RGPD)',
        rectification: 'Droit de rectification des données inexactes (Art. 16 RGPD)',
        erasure: 'Droit à l\'effacement de vos données (Art. 17 RGPD)',
        restriction: 'Droit à la limitation du traitement (Art. 18 RGPD)',
        portability: 'Droit à la portabilité des données (Art. 20 RGPD)',
        objection: 'Droit d\'opposition au traitement (Art. 21 RGPD)',
        complaint: 'Vous avez également le droit de déposer une plainte auprès d\'une autorité de contrôle de la protection des données si vous estimez que le traitement de vos données enfreint le RGPD.'
      },
      security: {
        title: 'Sécurité des données',
        text: 'Nous employons des mesures de sécurité techniques et organisationnelles pour protéger vos données. La connexion à notre site web est chiffrée SSL/TLS. Les transferts de fichiers entre utilisateurs sont effectués via des connexions WebRTC chiffrées (DTLS).'
      },
      changes: {
        title: 'Modifications de cette politique de confidentialité',
        text: 'Nous nous réservons le droit d\'adapter cette politique de confidentialité si nécessaire pour nous conformer aux exigences légales modifiées ou lors de modifications du service. La version actuelle peut toujours être consultée sur cette page.'
      },
      lastUpdate: 'Dernière mise à jour'
    }
  }
}
