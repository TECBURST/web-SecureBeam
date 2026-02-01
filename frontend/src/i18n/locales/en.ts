export default {
  nav: {
    home: 'Home',
    security: 'Security'
  },
  download: {
    available: 'Available Now',
    headline: 'You\'re just one click away',
    subline: 'Download SecureBeam and share files securely – end-to-end encrypted, directly from device to device, no cloud.',
    forWindows: 'Download for Windows',
    forMac: 'Download for macOS',
    forLinux: 'Download for Linux',
    otherPlatforms: 'Other platforms',
    freeForever: 'Free forever',
    encrypted: 'End-to-end encrypted',
    noCloud: 'No cloud',
    openSource: 'Open source'
  },
  comingSoon: {
    title: 'We\'re Rebuilding SecureBeam',
    subtitle: 'We\'re rethinking file transfer from the ground up. Browser-based WebRTC had too many limitations, so we\'re building native apps for a better experience.',
    nativeApps: {
      title: 'Native Applications',
      description: 'Faster and more reliable transfers with our dedicated desktop apps for Windows, macOS, and Linux.'
    },
    betterProtocol: {
      title: 'Better Protocol',
      description: 'Native apps allow us to use more efficient protocols without browser limitations.'
    },
    stayTuned: 'Stay tuned for updates. Something great is coming.'
  },
  home: {
    title: 'Secure File Transfer',
    subtitle: 'Send files directly to another device. End-to-end encrypted, peer-to-peer, no size limits.',
    dropzone: {
      title: 'Drop files here or click to browse',
      subtitle: 'No file size limit. Files are transferred directly.'
    },
    or: 'or',
    receiveButton: 'I have a code to receive files',
    backButton: 'Back to send files',
    codeInput: {
      placeholder: 'Enter transfer code',
      button: 'Connect'
    },
    selectedFiles: 'Selected Files',
    clearAll: 'Clear all',
    sendFiles: 'Send {count} file(s)',
    addMore: 'Add more files',
    cancel: 'Cancel Transfer',
    complete: {
      title: 'Transfer Complete',
      message: '{count} file(s) transferred successfully.'
    },
    error: {
      title: 'Transfer Failed'
    },
    newTransfer: 'Start New Transfer',
    tryAgain: 'Try Again'
  },
  code: {
    title: 'Your Transfer Code',
    subtitle: 'Share this code with the recipient',
    copied: 'Copied!',
    copy: 'Copy Code',
    copyLink: 'Copy Link',
    qrHint: 'Or scan QR code',
    qrSubtitle: 'Scan to receive files',
    linkSubtitle: 'Share this link directly',
    tabs: {
      code: 'Code',
      qr: 'QR Code',
      link: 'Link'
    }
  },
  status: {
    waiting: 'Waiting for recipient...',
    connecting: 'Connecting...',
    connected: 'Connected! Preparing transfer...',
    awaitingAcceptance: 'Waiting for recipient to accept...',
    transferring: 'Sending files...',
    receiving: 'Receiving files...',
    completed: 'Transfer complete!',
    error: 'Transfer failed'
  },
  warning: {
    doNotClose: 'Do not close this window',
    transferWillFail: 'The transfer will be cancelled if you close or refresh this page.'
  },
  download: {
    title: 'Multiple Files Incoming',
    message: 'You are about to receive {count} files. Allow downloads to continue?',
    info: 'Each file will be downloaded to your default downloads folder.',
    allow: 'Allow Downloads',
    deny: 'Cancel'
  },
  confirmation: {
    title: 'Incoming File Transfer',
    message: 'Someone wants to send you {count} file(s) ({size} total)',
    files: 'Files to receive',
    warning: 'Only accept files from people you trust. Files will be downloaded to your device after you accept.',
    accept: 'Accept Transfer',
    reject: 'Decline'
  },
  security: {
    title: 'How SecureBeam Works',
    subtitle: 'Understanding our secure file transfer process',
    steps: {
      title: 'Transfer Process',
      step1: {
        title: 'Generate Code',
        description: 'When you select files, our server generates a unique, cryptographically secure room code (e.g., A7KN-P3XQ-8FDM). This code has ~71 bits of entropy, making it virtually impossible to guess.'
      },
      step2: {
        title: 'Establish Connection',
        description: 'The recipient enters the code. Both browsers connect to our signaling server via WebSocket. The server only relays connection information - it never sees your files.'
      },
      step3: {
        title: 'WebRTC Handshake',
        description: 'Using WebRTC, both browsers exchange connection details (ICE candidates, SDP offers). A direct peer-to-peer connection is established using STUN/TURN servers for NAT traversal.'
      },
      step4: {
        title: 'Direct Transfer',
        description: 'Files are sent directly from sender to receiver through the encrypted WebRTC DataChannel. Data never passes through our servers.'
      },
      step5: {
        title: 'Confirmation',
        description: 'The receiver confirms each file was fully received. Only then does the sender see "Transfer Complete". This ensures nothing is lost.'
      }
    },
    encryption: {
      title: 'Encryption & Security',
      dtls: {
        title: 'DTLS Encryption',
        description: 'All WebRTC connections use DTLS 1.2+ encryption. Your data is encrypted in transit between browsers.'
      },
      p2p: {
        title: 'Peer-to-Peer',
        description: 'Files go directly between devices. Our servers only help establish the connection - they never store or see your files.'
      },
      codes: {
        title: 'Secure Codes',
        description: '12-character codes with 71 bits of entropy. Generated using cryptographically secure random numbers.'
      },
      noStorage: {
        title: 'No Storage',
        description: 'We don\'t store your files, logs, or transfer history. Once the transfer is complete, all data is gone.'
      }
    },
    diagram: {
      title: 'Architecture Overview',
      sender: 'Sender',
      receiver: 'Receiver',
      signaling: 'Signaling Server',
      signalingDesc: 'One-time WebRTC connection setup',
      turn: 'TURN Server',
      directConnection: 'Direct P2P Connection',
      directConnectionDesc: 'Encrypted file transfer'
    }
  },
  footer: {
    encrypted: 'End-to-end encrypted',
    noLimit: 'No file size limit',
    p2p: 'Peer-to-peer',
    transfers: 'transfers completed',
    impressum: 'Legal Notice',
    privacy: 'Privacy Policy',
    legal: 'Legal'
  },
  legal: {
    overview: {
      title: 'Legal',
      subtitle: 'All important legal information about SecureBeam',
      impressumDesc: 'Information about the operator and legal details in accordance with telecommunications law.',
      privacyDesc: 'Information about what data we collect and how we use it.',
      readMore: 'Read more'
    },
    impressum: {
      title: 'Legal Notice',
      according: 'Information pursuant to § 5 TMG (German Telemedia Act)',
      country: 'Germany',
      contact: 'Contact',
      email: 'Email',
      responsible: 'Responsible for content pursuant to § 55 Abs. 2 RStV',
      dispute: 'EU Dispute Resolution',
      disputeText: 'The European Commission provides a platform for online dispute resolution (OS):',
      disputeNote: 'We are not willing or obliged to participate in dispute resolution proceedings before a consumer arbitration board.',
      disclaimer: 'Disclaimer',
      disclaimerText: 'SecureBeam is a peer-to-peer file transfer service. We assume no liability for transferred content, as it is exchanged directly between users and never passes through our servers. Use is at your own risk.'
    },
    privacy: {
      title: 'Privacy Policy',
      intro: {
        title: 'Introduction',
        text: 'The protection of your personal data is important to us. This privacy policy informs you about what data we collect and how we use it. SecureBeam was developed with the principle of data minimization – we only collect data that is absolutely necessary for technical operation.'
      },
      controller: {
        title: 'Data Controller'
      },
      data: {
        title: 'What Data We Collect',
        server: {
          title: 'Server Log Data',
          text: 'When using our website, the following data is automatically processed temporarily:',
          ip: 'IP address (anonymized after 24 hours)',
          time: 'Time of access',
          browser: 'Browser type and version',
          basis: 'Legal basis: Art. 6(1)(f) GDPR (legitimate interest in secure website operation).'
        },
        webrtc: {
          title: 'WebRTC Connection Data',
          text: 'To establish peer-to-peer connections, connection information (ICE candidates) is temporarily exchanged via our signaling server. This data is only stored for the duration of the connection establishment and is deleted immediately afterwards. Actual file transfers occur directly between browsers – we have no access to the transferred files.'
        },
        stats: {
          title: 'Anonymous Statistics',
          text: 'We count the number of successfully completed transfers. This statistic contains no personal data – only an anonymous counter is incremented.'
        }
      },
      cloudflare: {
        title: 'Cloudflare',
        text: 'We use Cloudflare, Inc. as a Content Delivery Network (CDN) and for protection against attacks. Cloudflare may process IP addresses and technical access data as a data processor. Cloudflare is certified under the EU-US Data Privacy Framework.',
        more: 'For more information, see Cloudflare\'s privacy policy:'
      },
      cookies: {
        title: 'Cookies',
        text: 'SecureBeam itself does not use cookies. However, Cloudflare may set technically necessary cookies that are required for the security and functionality of the website (e.g., for bot protection). These cookies do not contain personal data and serve exclusively technical purposes.'
      },
      rights: {
        title: 'Your Rights',
        intro: 'Under the GDPR, you have the following rights:',
        access: 'Right to access your stored data (Art. 15 GDPR)',
        rectification: 'Right to rectification of inaccurate data (Art. 16 GDPR)',
        erasure: 'Right to erasure of your data (Art. 17 GDPR)',
        restriction: 'Right to restriction of processing (Art. 18 GDPR)',
        portability: 'Right to data portability (Art. 20 GDPR)',
        objection: 'Right to object to processing (Art. 21 GDPR)',
        complaint: 'You also have the right to lodge a complaint with a data protection supervisory authority if you believe that the processing of your data violates the GDPR.'
      },
      security: {
        title: 'Data Security',
        text: 'We employ technical and organizational security measures to protect your data. The connection to our website is SSL/TLS encrypted. File transfers between users are conducted via encrypted WebRTC connections (DTLS).'
      },
      changes: {
        title: 'Changes to This Privacy Policy',
        text: 'We reserve the right to adapt this privacy policy as needed to comply with changed legal requirements or when changes to the service are made. The current version can always be found on this page.'
      },
      lastUpdate: 'Last updated'
    }
  }
}
