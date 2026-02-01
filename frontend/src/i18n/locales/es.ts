export default {
  nav: {
    home: 'Inicio',
    security: 'Seguridad'
  },
  comingSoon: {
    title: 'Estamos reconstruyendo SecureBeam',
    subtitle: 'Estamos repensando la transferencia de archivos desde cero. WebRTC basado en navegador tenía demasiadas limitaciones, así que estamos desarrollando aplicaciones nativas para una mejor experiencia.',
    nativeApps: {
      title: 'Aplicaciones Nativas',
      description: 'Comenzando con Windows, estamos desarrollando aplicaciones dedicadas que ofrecen transferencias más rápidas y confiables.'
    },
    betterProtocol: {
      title: 'Mejor Protocolo',
      description: 'Las aplicaciones nativas nos permiten usar protocolos más eficientes sin las limitaciones del navegador.'
    },
    stayTuned: 'Mantente atento a las actualizaciones. Algo grandioso está por venir.'
  },
  home: {
    title: 'Transferencia Segura de Archivos',
    subtitle: 'Envía archivos directamente a otro dispositivo. Cifrado de extremo a extremo, peer-to-peer, sin límite de tamaño.',
    dropzone: {
      title: 'Arrastra archivos aquí o haz clic para seleccionar',
      subtitle: 'Sin límite de tamaño. Los archivos se transfieren directamente.'
    },
    or: 'o',
    receiveButton: 'Tengo un código para recibir archivos',
    backButton: 'Volver a enviar archivos',
    codeInput: {
      placeholder: 'Ingresa el código de transferencia',
      button: 'Conectar'
    },
    selectedFiles: 'Archivos Seleccionados',
    clearAll: 'Eliminar todos',
    sendFiles: 'Enviar {count} archivo(s)',
    addMore: 'Agregar más archivos',
    cancel: 'Cancelar Transferencia',
    complete: {
      title: 'Transferencia Completada',
      message: '{count} archivo(s) transferido(s) exitosamente.'
    },
    error: {
      title: 'Transferencia Fallida'
    },
    newTransfer: 'Iniciar Nueva Transferencia',
    tryAgain: 'Intentar de Nuevo'
  },
  code: {
    title: 'Tu Código de Transferencia',
    subtitle: 'Comparte este código con el destinatario',
    copied: '¡Copiado!',
    copy: 'Copiar Código',
    copyLink: 'Copiar Enlace',
    qrHint: 'O escanea el código QR',
    qrSubtitle: 'Escanear para recibir',
    linkSubtitle: 'Comparte este enlace directamente',
    tabs: {
      code: 'Código',
      qr: 'Código QR',
      link: 'Enlace'
    }
  },
  status: {
    waiting: 'Esperando al destinatario...',
    connecting: 'Conectando...',
    connected: '¡Conectado! Preparando transferencia...',
    awaitingAcceptance: 'Esperando que el destinatario acepte...',
    transferring: 'Enviando archivos...',
    receiving: 'Recibiendo archivos...',
    completed: '¡Transferencia completada!',
    error: 'Transferencia fallida'
  },
  warning: {
    doNotClose: 'No cierre esta ventana',
    transferWillFail: 'La transferencia se cancelará si cierra o actualiza esta página.'
  },
  downloadDialog: {
    title: 'Archivos múltiples entrantes',
    message: 'Vas a recibir {count} archivos. ¿Permitir descargas?',
    info: 'Cada archivo se descargará en tu carpeta de descargas.',
    allow: 'Permitir',
    deny: 'Cancelar'
  },
  confirmation: {
    title: 'Transferencia de archivos entrante',
    message: 'Alguien quiere enviarte {count} archivo(s) ({size} en total)',
    files: 'Archivos a recibir',
    warning: 'Solo acepta archivos de personas de confianza. Los archivos se descargarán después de aceptar.',
    accept: 'Aceptar transferencia',
    reject: 'Rechazar'
  },
  security: {
    title: 'Cómo Funciona SecureBeam',
    subtitle: 'Comprende nuestro proceso de transferencia segura',
    steps: {
      title: 'Proceso de Transferencia',
      step1: {
        title: 'Generar Código',
        description: 'Cuando seleccionas archivos, nuestro servidor genera un código de sala único y criptográficamente seguro (ej. A7KN-P3XQ-8FDM). Este código tiene ~71 bits de entropía, haciéndolo virtualmente imposible de adivinar.'
      },
      step2: {
        title: 'Establecer Conexión',
        description: 'El destinatario ingresa el código. Ambos navegadores se conectan a nuestro servidor de señalización via WebSocket. El servidor solo retransmite información de conexión - nunca ve tus archivos.'
      },
      step3: {
        title: 'Handshake WebRTC',
        description: 'Usando WebRTC, ambos navegadores intercambian detalles de conexión (candidatos ICE, ofertas SDP). Se establece una conexión directa peer-to-peer usando servidores STUN/TURN para traversal NAT.'
      },
      step4: {
        title: 'Transferencia Directa',
        description: 'Los archivos se envían directamente del emisor al receptor a través del canal de datos WebRTC cifrado. Los datos nunca pasan por nuestros servidores.'
      },
      step5: {
        title: 'Confirmación',
        description: 'El receptor confirma que cada archivo fue completamente recibido. Solo entonces el emisor ve "Transferencia Completada". Esto asegura que nada se pierda.'
      }
    },
    encryption: {
      title: 'Cifrado y Seguridad',
      dtls: {
        title: 'Cifrado DTLS',
        description: 'Todas las conexiones WebRTC usan cifrado DTLS 1.2+. Tus datos están cifrados en tránsito entre navegadores.'
      },
      p2p: {
        title: 'Peer-to-Peer',
        description: 'Los archivos van directamente entre dispositivos. Nuestros servidores solo ayudan a establecer la conexión - nunca almacenan ni ven tus archivos.'
      },
      codes: {
        title: 'Códigos Seguros',
        description: 'Códigos de 12 caracteres con 71 bits de entropía. Generados usando números aleatorios criptográficamente seguros.'
      },
      noStorage: {
        title: 'Sin Almacenamiento',
        description: 'No almacenamos tus archivos, registros ni historial de transferencias. Una vez completada la transferencia, todos los datos desaparecen.'
      }
    },
    diagram: {
      title: 'Visión General de la Arquitectura',
      sender: 'Emisor',
      receiver: 'Receptor',
      signaling: 'Servidor de Señalización',
      signalingDesc: 'Configuración única de conexión WebRTC',
      turn: 'Servidor TURN',
      directConnection: 'Conexión P2P Directa',
      directConnectionDesc: 'Transferencia de archivos cifrada'
    }
  },
  footer: {
    encrypted: 'Cifrado de extremo a extremo',
    noLimit: 'Sin límite de tamaño',
    p2p: 'Peer-to-peer',
    transfers: 'transferencias completadas',
    impressum: 'Aviso Legal',
    privacy: 'Privacidad',
    legal: 'Legal'
  },
  legal: {
    overview: {
      title: 'Legal',
      subtitle: 'Toda la información legal importante sobre SecureBeam',
      impressumDesc: 'Información sobre el operador y detalles legales de acuerdo con la ley de telecomunicaciones.',
      privacyDesc: 'Información sobre qué datos recopilamos y cómo los utilizamos.',
      readMore: 'Leer más'
    },
    impressum: {
      title: 'Aviso Legal',
      according: 'Información según § 5 TMG (Ley alemana de telemedios)',
      country: 'Alemania',
      contact: 'Contacto',
      email: 'Correo electrónico',
      responsible: 'Responsable del contenido según § 55 Abs. 2 RStV',
      dispute: 'Resolución de disputas UE',
      disputeText: 'La Comisión Europea proporciona una plataforma para la resolución de disputas en línea (ODR):',
      disputeNote: 'No estamos dispuestos ni obligados a participar en procedimientos de resolución de disputas ante una junta de arbitraje de consumidores.',
      disclaimer: 'Descargo de responsabilidad',
      disclaimerText: 'SecureBeam es un servicio de transferencia de archivos peer-to-peer. No asumimos responsabilidad por el contenido transferido, ya que se intercambia directamente entre usuarios y nunca pasa por nuestros servidores. El uso es bajo su propia responsabilidad.'
    },
    privacy: {
      title: 'Política de Privacidad',
      intro: {
        title: 'Introducción',
        text: 'La protección de sus datos personales es importante para nosotros. Esta política de privacidad le informa sobre qué datos recopilamos y cómo los utilizamos. SecureBeam fue desarrollado con el principio de minimización de datos – solo recopilamos datos que son absolutamente necesarios para la operación técnica.'
      },
      controller: {
        title: 'Responsable del tratamiento'
      },
      data: {
        title: 'Qué datos recopilamos',
        server: {
          title: 'Datos de registro del servidor',
          text: 'Al usar nuestro sitio web, los siguientes datos se procesan automáticamente de forma temporal:',
          ip: 'Dirección IP (anonimizada después de 24 horas)',
          time: 'Hora de acceso',
          browser: 'Tipo y versión del navegador',
          basis: 'Base legal: Art. 6(1)(f) RGPD (interés legítimo en la operación segura del sitio web).'
        },
        webrtc: {
          title: 'Datos de conexión WebRTC',
          text: 'Para establecer conexiones peer-to-peer, la información de conexión (candidatos ICE) se intercambia temporalmente a través de nuestro servidor de señalización. Estos datos solo se almacenan durante el establecimiento de la conexión y se eliminan inmediatamente después. Las transferencias de archivos reales ocurren directamente entre navegadores – no tenemos acceso a los archivos transferidos.'
        },
        stats: {
          title: 'Estadísticas anónimas',
          text: 'Contamos el número de transferencias completadas exitosamente. Esta estadística no contiene datos personales – solo se incrementa un contador anónimo.'
        }
      },
      cloudflare: {
        title: 'Cloudflare',
        text: 'Utilizamos Cloudflare, Inc. como red de distribución de contenido (CDN) y para protección contra ataques. Cloudflare puede procesar direcciones IP y datos técnicos de acceso como encargado del tratamiento. Cloudflare está certificado bajo el Marco de Privacidad de Datos UE-EE.UU.',
        more: 'Para más información, consulte la política de privacidad de Cloudflare:'
      },
      cookies: {
        title: 'Cookies',
        text: 'SecureBeam en sí no utiliza cookies. Sin embargo, Cloudflare puede establecer cookies técnicamente necesarias para la seguridad y funcionalidad del sitio web (por ejemplo, para protección contra bots). Estas cookies no contienen datos personales y sirven exclusivamente para fines técnicos.'
      },
      rights: {
        title: 'Sus derechos',
        intro: 'Bajo el RGPD, usted tiene los siguientes derechos:',
        access: 'Derecho de acceso a sus datos almacenados (Art. 15 RGPD)',
        rectification: 'Derecho de rectificación de datos inexactos (Art. 16 RGPD)',
        erasure: 'Derecho de supresión de sus datos (Art. 17 RGPD)',
        restriction: 'Derecho a la limitación del tratamiento (Art. 18 RGPD)',
        portability: 'Derecho a la portabilidad de datos (Art. 20 RGPD)',
        objection: 'Derecho de oposición al tratamiento (Art. 21 RGPD)',
        complaint: 'También tiene derecho a presentar una queja ante una autoridad de supervisión de protección de datos si cree que el tratamiento de sus datos viola el RGPD.'
      },
      security: {
        title: 'Seguridad de datos',
        text: 'Empleamos medidas de seguridad técnicas y organizativas para proteger sus datos. La conexión a nuestro sitio web está cifrada con SSL/TLS. Las transferencias de archivos entre usuarios se realizan a través de conexiones WebRTC cifradas (DTLS).'
      },
      changes: {
        title: 'Cambios a esta política de privacidad',
        text: 'Nos reservamos el derecho de adaptar esta política de privacidad según sea necesario para cumplir con los requisitos legales modificados o cuando se realicen cambios en el servicio. La versión actual siempre se puede encontrar en esta página.'
      },
      lastUpdate: 'Última actualización'
    }
  }
}
