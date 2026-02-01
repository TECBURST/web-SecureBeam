export default {
  nav: {
    home: '首页',
    security: '安全'
  },
  comingSoon: {
    title: '我们正在重建 SecureBeam',
    subtitle: '我们正在从头开始重新思考文件传输。基于浏览器的 WebRTC 有太多限制，因此我们正在开发原生应用以提供更好的体验。',
    nativeApps: {
      title: '原生应用',
      description: '从 Windows 开始，我们正在开发专用应用程序，提供更快、更可靠的传输。'
    },
    betterProtocol: {
      title: '更好的协议',
      description: '原生应用让我们能够使用更高效的协议，不受浏览器限制。'
    },
    stayTuned: '请继续关注更新。精彩即将到来。'
  },
  home: {
    title: '安全文件传输',
    subtitle: '直接向另一台设备发送文件。端到端加密，点对点传输，无大小限制。',
    dropzone: {
      title: '将文件拖放到此处或点击选择',
      subtitle: '无文件大小限制。文件直接传输。'
    },
    or: '或',
    receiveButton: '我有接收码',
    backButton: '返回发送文件',
    codeInput: {
      placeholder: '输入传输码',
      button: '连接'
    },
    selectedFiles: '已选文件',
    clearAll: '清除全部',
    sendFiles: '发送 {count} 个文件',
    addMore: '添加更多文件',
    cancel: '取消传输',
    complete: {
      title: '传输完成',
      message: '{count} 个文件传输成功。'
    },
    error: {
      title: '传输失败'
    },
    newTransfer: '开始新传输',
    tryAgain: '重试'
  },
  code: {
    title: '您的传输码',
    subtitle: '将此码分享给接收方',
    copied: '已复制！',
    copy: '复制代码',
    copyLink: '复制链接',
    qrHint: '或扫描二维码',
    qrSubtitle: '扫描接收文件',
    linkSubtitle: '直接分享此链接',
    tabs: {
      code: '代码',
      qr: '二维码',
      link: '链接'
    }
  },
  status: {
    waiting: '等待接收方...',
    connecting: '正在连接...',
    connected: '已连接！正在准备传输...',
    awaitingAcceptance: '等待接收方确认...',
    transferring: '正在发送文件...',
    receiving: '正在接收文件...',
    completed: '传输完成！',
    error: '传输失败'
  },
  warning: {
    doNotClose: '请勿关闭此窗口',
    transferWillFail: '如果您关闭或刷新此页面，传输将被取消。'
  },
  downloadDialog: {
    title: '多个文件传入',
    message: '您将收到 {count} 个文件。允许下载吗？',
    info: '每个文件将下载到您的默认下载文件夹。',
    allow: '允许',
    deny: '取消'
  },
  confirmation: {
    title: '收到文件传输请求',
    message: '有人想向您发送 {count} 个文件（共 {size}）',
    files: '待接收的文件',
    warning: '请只接受来自信任的人的文件。接受后文件将被下载到您的设备。',
    accept: '接受传输',
    reject: '拒绝'
  },
  security: {
    title: 'SecureBeam 工作原理',
    subtitle: '了解我们的安全文件传输过程',
    steps: {
      title: '传输过程',
      step1: {
        title: '生成代码',
        description: '当您选择文件时，我们的服务器会生成一个唯一的、加密安全的房间代码（例如 A7KN-P3XQ-8FDM）。此代码具有约71位熵，几乎不可能被猜测。'
      },
      step2: {
        title: '建立连接',
        description: '接收方输入代码。两个浏览器通过WebSocket连接到我们的信令服务器。服务器仅中继连接信息 - 它永远不会看到您的文件。'
      },
      step3: {
        title: 'WebRTC 握手',
        description: '使用WebRTC，两个浏览器交换连接详细信息（ICE候选、SDP提议）。通过STUN/TURN服务器建立直接的点对点连接以进行NAT穿越。'
      },
      step4: {
        title: '直接传输',
        description: '文件通过加密的WebRTC数据通道直接从发送方发送到接收方。数据永远不会经过我们的服务器。'
      },
      step5: {
        title: '确认',
        description: '接收方确认每个文件已完全接收。只有这样，发送方才会看到"传输完成"。这确保不会丢失任何内容。'
      }
    },
    encryption: {
      title: '加密与安全',
      dtls: {
        title: 'DTLS 加密',
        description: '所有WebRTC连接都使用DTLS 1.2+加密。您的数据在浏览器之间传输时是加密的。'
      },
      p2p: {
        title: '点对点',
        description: '文件直接在设备之间传输。我们的服务器仅帮助建立连接 - 它们永远不会存储或查看您的文件。'
      },
      codes: {
        title: '安全代码',
        description: '12字符代码，具有71位熵。使用加密安全随机数生成。'
      },
      noStorage: {
        title: '无存储',
        description: '我们不存储您的文件、日志或传输历史。传输完成后，所有数据都会消失。'
      }
    },
    diagram: {
      title: '架构概述',
      sender: '发送方',
      receiver: '接收方',
      signaling: '信令服务器',
      signalingDesc: '一次性WebRTC连接建立',
      turn: 'TURN 服务器',
      directConnection: '直接P2P连接',
      directConnectionDesc: '加密文件传输'
    }
  },
  footer: {
    encrypted: '端到端加密',
    noLimit: '无文件大小限制',
    p2p: '点对点',
    transfers: '次传输完成',
    impressum: '法律声明',
    privacy: '隐私政策',
    legal: '法律信息'
  },
  legal: {
    overview: {
      title: '法律信息',
      subtitle: '关于 SecureBeam 的所有重要法律信息',
      impressumDesc: '根据电信法提供的运营商信息和法律详情。',
      privacyDesc: '关于我们收集哪些数据以及如何使用这些数据的信息。',
      readMore: '了解更多'
    },
    impressum: {
      title: '法律声明',
      according: '根据德国电信媒体法 § 5 TMG 提供的信息',
      country: '德国',
      contact: '联系方式',
      email: '电子邮件',
      responsible: '根据 § 55 Abs. 2 RStV 对内容负责',
      dispute: '欧盟争议解决',
      disputeText: '欧盟委员会提供在线争议解决平台 (ODR)：',
      disputeNote: '我们不愿意也无义务参与消费者仲裁委员会的争议解决程序。',
      disclaimer: '免责声明',
      disclaimerText: 'SecureBeam 是一个点对点文件传输服务。我们不对传输的内容承担任何责任，因为内容直接在用户之间交换，从不经过我们的服务器。使用风险由用户自行承担。'
    },
    privacy: {
      title: '隐私政策',
      intro: {
        title: '简介',
        text: '保护您的个人数据对我们很重要。本隐私政策告知您我们收集哪些数据以及如何使用这些数据。SecureBeam 遵循数据最小化原则开发——我们只收集技术运营所绝对必需的数据。'
      },
      controller: {
        title: '数据控制者'
      },
      data: {
        title: '我们收集哪些数据',
        server: {
          title: '服务器日志数据',
          text: '使用我们的网站时，以下数据会被自动临时处理：',
          ip: 'IP 地址（24小时后匿名化）',
          time: '访问时间',
          browser: '浏览器类型和版本',
          basis: '法律依据：GDPR 第6条第1款f项（网站安全运营的合法利益）。'
        },
        webrtc: {
          title: 'WebRTC 连接数据',
          text: '为建立点对点连接，连接信息（ICE候选）通过我们的信令服务器临时交换。这些数据仅在连接建立期间存储，之后立即删除。实际文件传输直接在浏览器之间进行——我们无法访问传输的文件。'
        },
        stats: {
          title: '匿名统计',
          text: '我们统计成功完成的传输数量。此统计不包含任何个人数据——仅增加匿名计数器。'
        }
      },
      cloudflare: {
        title: 'Cloudflare',
        text: '我们使用 Cloudflare, Inc. 作为内容分发网络 (CDN) 并用于防御攻击。Cloudflare 可能作为数据处理者处理 IP 地址和技术访问数据。Cloudflare 已获得欧盟-美国数据隐私框架认证。',
        more: '更多信息请参阅 Cloudflare 的隐私政策：'
      },
      cookies: {
        title: 'Cookies',
        text: 'SecureBeam 本身不使用 cookies。但是，Cloudflare 可能会设置网站安全和功能所需的技术必要 cookies（例如，用于防机器人保护）。这些 cookies 不包含个人数据，仅用于技术目的。'
      },
      rights: {
        title: '您的权利',
        intro: '根据 GDPR，您享有以下权利：',
        access: '访问您存储数据的权利（GDPR 第15条）',
        rectification: '更正不准确数据的权利（GDPR 第16条）',
        erasure: '删除您数据的权利（GDPR 第17条）',
        restriction: '限制处理的权利（GDPR 第18条）',
        portability: '数据可携带权（GDPR 第20条）',
        objection: '反对处理的权利（GDPR 第21条）',
        complaint: '如果您认为对您数据的处理违反了 GDPR，您还有权向数据保护监管机构提出投诉。'
      },
      security: {
        title: '数据安全',
        text: '我们采用技术和组织安全措施来保护您的数据。与我们网站的连接采用 SSL/TLS 加密。用户之间的文件传输通过加密的 WebRTC 连接 (DTLS) 进行。'
      },
      changes: {
        title: '本隐私政策的变更',
        text: '我们保留在必要时调整本隐私政策的权利，以符合变更的法律要求或服务变更。当前版本始终可在此页面找到。'
      },
      lastUpdate: '最后更新'
    }
  }
}
