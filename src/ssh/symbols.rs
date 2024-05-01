// All of the symbolic name - reason code assignments are defined in SSH-NUMBERS

// 4.1.2
pub enum SSH_MSG {
    DISCONNECT = 1,
    IGNORE = 2,
    UNIMPLEMENTED = 3,
    DEBUG = 4,
    SERVICE_REQUEST = 5,
    SERVICE_ACCEPT = 6,
    KEXINIT = 20,
    NEWKEYS = 21,
    USERAUTH_REQUEST = 50,
    USERAUTH_FAILURE = 51,
    USERAUTH_SUCCESS = 52,
    USERAUTH_BANNER = 53,
    GLOBAL_REQUEST = 80,
    REQUEST_SUCCESS = 81,
    REQUEST_FAILURE = 82,
    CHANNEL_OPEN = 90,
    CHANNEL_OPEN_CONFIRMATION = 91,
    CHANNEL_OPEN_FAILURE = 92,
    CHANNEL_WINDOW_ADJUST = 93,
    CHANNEL_DATA = 94,
    CHANNEL_EXTENDED_DATA = 95,
    CHANNEL_EOF = 96,
    CHANNEL_CLOSE = 97,
    CHANNEL_REQUEST = 98,
    CHANNEL_SUCCESS = 99,
    CHANNEL_FAILURE = 100,
}

// 4.2.2
pub enum SSH_DISCONNECT {
    HOST_NOT_ALLOWED_TO_CONNECT = 1,
    PROTOCOL_ERROR = 2,
    KEY_EXCHANGE_FAILED = 3,
    RESERVED = 4,
    MAC_ERROR = 5,
    COMPRESSION_ERROR = 6,
    SERVICE_NOT_AVAILABLE = 7,
    PROTOCOL_VERSION_NOT_SUPPORTED = 8,
    HOST_KEY_NOT_VERIFIABLE = 9,
    CONNECTION_LOST = 10,
    BY_APPLICATION = 11,
    TOO_MANY_CONNECTIONS = 12,
    AUTH_CANCELLED_BY_USER = 13,
    NO_MORE_AUTH_METHODS_AVAILABLE = 14,
    ILLEGAL_USER_NAME = 15,
}

// 4.3.2
pub enum SSH_OPEN {
    ADMINISTRATIVELY_PROHIBITED = 1,
    CONNECT_FAILED = 2,
    UNKNOWN_CHANNEL_TYPE = 3,
    RESOURCE_SHORTAGE = 4,
}

// 4.4.2
pub enum SSH_EXTENDED {
    DATA_STDERR = 1,
}

// 4.5.2 - for PTY encoded terminal modes, which are responses to
// SSH_MSG_CHANNEL_REQUEST messages with a "pty-req" string
pub enum OPCODE_PTY_TERMINAL_MODE {
    TTY_OP_END = 0,      // Indicates end of options.
    VINTR = 1, //     Interrupt character; 255 if none.  Similarly for the other characters.  Not all of these characters are supported on all systems.
    VQUIT = 2, //     The quit character (sends SIGQUIT signal on POSIX systems).
    VERASE = 3, //     Erase the character to left of the cursor.
    VKILL = 4, //     Kill the current input line.
    VEOF = 5,  //     End-of-file character (sends EOF from the terminal).
    VEOL = 6,  //     End-of-line character in addition to carriage return and/or linefeed.
    VEOL2 = 7, //     Additional end-of-line character.
    VSTART = 8, //     Continues paused output (normally control-Q).
    VSTOP = 9, //     Pauses output (normally control-S).
    VSUSP = 10, //     Suspends the current program.
    VDSUSP = 11, //     Another suspend character.
    VREPRINT = 12, //   Reprints the current input line.
    VWERASE = 13, //   Erases a word left of cursor.
    VLNEXT = 14, //   Enter the next character typed literally, even if it is a special character
    VFLUSH = 15, //   Character to flush output.
    VSWTCH = 16, //   Switch to a different shell layer.
    VSTATUS = 17, //   Prints system status line (load, command, pid, etc).
    VDISCARD = 18, //   Toggles the flushing of terminal output.
    IGNPAR = 30, //   The ignore parity flag.  The parameter SHOULD be 0 if this flag is FALSE, and 1 if it is TRUE.
    PARMRK = 31, //   Mark parity and framing errors.
    INPCK = 32,  //   Enable checking of parity errors.
    ISTRIP = 33, //   Strip 8th bit off characters.
    INLCR = 34,  //   Map NL into CR on input.
    IGNCR = 35,  //   Ignore CR on input.
    ICRNL = 36,  //   Map CR to NL on input.
    IUCLC = 37,  //   Translate uppercase characters to lowercase.
    IXON = 38,   //   Enable output flow control.
    IXANY = 39,  //   Any char will restart after stop.
    IXOFF = 40,  //   Enable input flow control.
    IMAXBEL = 41, //   Ring bell on input queue full.
    ISIG = 50,   //   Enable signals INTR, QUIT, [D]SUSP.
    ICANON = 51, //   Canonicalize input lines.
    XCASE = 52, //   Enable input and output of uppercase characters by preceding their lowercase equivalents with "\".
    ECHO = 53,  //   Enable echoing.
    ECHOE = 54, //   Visually erase chars.
    ECHOK = 55, //   Kill character discards current line.
    ECHONL = 56, //   Echo NL even if ECHO is off.
    NOFLSH = 57, //   Don't flush after interrupt.
    TOSTOP = 58, //   Stop background jobs from output.
    IEXTEN = 59, //   Enable extensions.
    ECHOCTL = 60, //   Echo control characters as ^(Char).
    ECHOKE = 61, //   Visual erase for line kill.
    PENDIN = 62, //   Retype pending input.
    OPOST = 70, //   Enable output processing.
    OLCUC = 71, //   Convert lowercase to uppercase.
    ONLCR = 72, //   Map NL to CR-NL.
    OCRNL = 73, //   Translate carriage return to newline (output).
    ONOCR = 74, //   Translate newline to carriage return-newline (output).
    ONLRET = 75, //   Newline performs a carriage return (output).
    CS7 = 90,   //   7 bit mode.
    CS8 = 91,   //   8 bit mode.
    PARENB = 92, //   Parity enable.
    PARODD = 93, //   Odd parity, else even.
    TTY_OP_ISPEED = 128, // Specifies the input baud rate in bits per second.
    TTY_OP_OSPEED = 129, // Specifies the output baud rate in bits per second.
}

// 4.7 Service Names
pub enum SERVICE_NAME {
    SSH_USERAUTH = "ssh-userauth",
    SSH_CONNECTION = "ssh-connection",
}

// 4.8 Authentication Method Names
pub enum SSH_USERAUTH_AUTHENTICATION_METHOD_NAME {
    PUBLICKEY = "publickey",
    PASSWORD = "password",
    HOSTBASED = "hostbased",
    NONE = "none",
}

// 4.9.1 Connection Protocol Channel Types
pub enum SSH_CONNECT_GLOBAL_REQUEST_NAME {
    SESSION = "session",
    X11 = "x11",
    FORWARDED_TCPIP = "forwarded-tcpip",
    DIRECT_TCPIP = "direct-tcpip",
}

// 4.9.2 Connection Protocol Global Request Names
pub enum SSH_CONNECT_GLOBAL_REQUEST_NAME {
    TCPIP_FORWARD = "tcpip-forward",
    CANCEL_TCPIP_FORWARD = "cancel-tcpip-forward",
}

// 4.9.3 Connection Protocol Channel Request Names
pub enum SSH_CONNECT_CHANNEL_REQUEST_NAME {
    PTY_REQ = "pty-req",
    X11_REQ = "x11-req",
    ENV = "env",
    SHELL = "shell",
    EXEC = "exec",
    SUBSYSTEM = "subsystem",
    WINDOW_CHANGE = "window-change",
    XON_XOFF = "xon-xoff",
    SIGNAL = "signal",
    EXIT_STATUS = "exit-status",
    EXIT_SIGNAL = "exit-signal",
}

// 4.9.4 Connection Protocol Signal Names
pub enum SSH_CONNECT_SIGNAL_NAME {
    ABRT = "ABRT",
    ALRM = "ALRM",
    FPE = "FPE",
    HUP = "HUP",
    ILL = "ILL",
    INT = "INT",
    KILL = "KILL",
    PIPE = "PIPE",
    QUIT = "QUIT",
    SEGV = "SEGV",
    TERM = "TERM",
    USR1 = "USR1",
    USR2 = "USR2",
}

// 4.10 Key Exchange Method Names
pub enum SSH_TRANS_KEY_EXCHANGE_NAME {
    DIFFIE_HELLMAN_GROUP1_SHA1 = "diffie-hellman-group1-sha1",
    DIFFIE_HELLMAN_GROUP14_SHA1 = "diffie-hellman-group14-sha1",
}

// 4.11.1 Encryption Algorithm Names
pub enum SSH_TRANS_ENCRYPTION_ALGORITHM_NAME {
    THREE_DES_CBC = "3des-cbc",
    BLOWFISH_CBC = "blowfish-cbc",
    TWOFISH256_CBC = "twofish256-cbc",
    TWOFISH_CBC = "twofish-cbc",
    TWOFISH192_CBC = "twofish192-cbc",
    TWOFISH128_CBC = "twofish128-cbc",
    AES256_CBC = "aes256-cbc",
    AES192_CBC = "aes192-cbc",
    AES128_CBC = "aes128-cbc",
    SERPENT256_CBC = "serpent256-cbc",
    SERPENT192_CBC = "serpent192-cbc",
    SERPENT128_CBC = "serpent128-cbc",
    ARCFOUR_CBC = "arcfour",
    IDEA_CBC = "idea-cbc",
    CAST128_CBC = "cast128-cbc",
    NONE = "none",
    DES_CBC = "des-cbc",
}

// 4.11.2 MAC Algorithm Names
pub enum SSH_TRANS_MAC_ALGORITHM_NAME {
    HMAC_SHA1 = "hmac-sha1",
    HMAC_SHA1_96 = "hmac-sha1-96",
    HMAC_MD5 = "hmac-md5",
    HMAC_MD5_96 = "hmac-md5-96",
    NONE = "none",
}

// 4.11.3 Public Key Algorithm Names
pub enum SSH_TRANS_PUBLIC_KEY_ALGORITHM_NAMES {
    SSH_DSS = "ssh-dss",
    SSH_RSA = "ssh-rsa",
    PGP_SIGN_RSA = "pgp-sign-rsa",
    PGP_SIGN_DSS = "pgp-sign-dss",
}

// 4.11.4 Compression Algorithm Names
pub enum SSH_TRANS_COMPRESSION_ALGORITHM_NAME {
    NONE="none",
    ZLIB="zlib"
}