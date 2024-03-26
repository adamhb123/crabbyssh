## [SSH-NUMBERS](https://datatracker.ietf.org/doc/html/rfc4250) - The Secure Shell (SSH) Protocol Assigned Numbers ([RFC 4250](https://datatracker.ietf.org/doc/html/rfc4250)) Summary (by section)
This is a summary of "The Secure Shell (SSH) Protocol Assigned Numbers" ([RFC 4250](https://datatracker.ietf.org/doc/html/rfc4250))

0. Abstract
    * This document defines the instructions to the IANA and the initial state of the IANA assigned numbers for
        the Secure Shell (SSH) protocol

    * This document is intended only for the initialization of the IANA registries referenced in the set of
        SSH documents
    
1. Introduction
    * This document DOES NOT define any new protocols
    * This document is intended only to create the initial state of the IANA databases for the SSH protocol and 
        also contains instructions for future assignments
    
    * Except for one HISTORIC algorithm generally regarded as OBSOLETE, this document does not define any new
        protocols or number ranges not already defined in:
        * [SSH-ARCH](./SSH-ARCH.md)
        * [SSH-TRANS](./SSH-TRANS.md)
        * [SSH-USERAUTH](./SSH-USERAUTH.md)
        * [SSH-CONNECT](./SSH-CONNECT.md)
    
2. Contributors
    * See original document

3. Conventions Used in This Document
    1. [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119) Keywords
        * See original document for details

    2. [RFC 2434](https://datatracker.ietf.org/doc/html/rfc2434) Keywords
        * See original document for details
    
    3. Protocol Fields and Values
        * See original document for details

4. IANA Considerations

    0. IANA Considerations

        * This entire document is the IANA considerations for the SSH protocol

        * This section contains conventions used in:
            1. Naming the namespaces
            2. The initial state of the registry
            3. Instructions for future assignments
    
    1. Message Numbers
        * The Message Number is a byte value that describes the payload of a packet
    
        1. Conventions
            * Protocol packets have message numbers in the range 1-255, which are allocated as follows:
                1. Transport Layer Protocol:
                    | Message Number Range | Description                                                                               |
                    |----------------------|-------------------------------------------------------------------------------------------|
                    | 1-19                 | Transport layer generic (e.g., disconnect, ignore, debug, etc.)                           |
                    | 20-29                | Algorithm negotiation                                                                     |
                    | 30-49                | Key exchange method specific (numbers can be reused for different authentication methods) |
                    
                2. User Authentication Protocol:
                    | Message Number Range | Description                                                                                      |
                    |----------------------|--------------------------------------------------------------------------------------------------|
                    | 50-59                | User authentication generic                                                                      |
                    | 60-79                | User authentication method specific (numbers can be reused for different authentication methods) |
                   
                3. Connection Protocol:
                    | Message Number Range | Description                 |
                    |----------------------|-----------------------------|
                    | 80-89                | Connection protocol generic |
                    | 90-127               | Channel related messages    |

                4. Reserved for Client protocols:
                    | Message Number Range | Description |
                    |----------------------|-------------|
                    | 128-191              | Reserved    |

                5. Local Extensions:
                    | Message Number Range | Description      |
                    |----------------------|------------------|
                    | 192-255              | Local extensions |

        2. Initial Assignments
            * The following table identifies the initial assignments of the Message ID values:
                | Message ID                        | Value | Reference                         |
                |-----------------------------------|-------|-----------------------------------|
                | SSH_MSG_DISCONNECT                | 1     | [SSH-TRANS](./SSH-TRANS.md)       |
                | SSH_MSG_IGNORE                    | 2     | [SSH-TRANS](./SSH-TRANS.md)       |
                | SSH_MSG_UNIMPLEMENTED             | 3     | [SSH-TRANS](./SSH-TRANS.md)       |
                | SSH_MSG_DEBUG                     | 4     | [SSH-TRANS](./SSH-TRANS.md)       |
                | SSH_MSG_SERVICE_REQUEST           | 5     | [SSH-TRANS](./SSH-TRANS.md)       |
                | SSH_MSG_SERVICE_ACCEPT            | 6     | [SSH-TRANS](./SSH-TRANS.md)       |
                | SSH_MSG_KEXINIT                   | 20    | [SSH-TRANS](./SSH-TRANS.md)       |
                | SSH_MSG_NEWKEYS                   | 21    | [SSH-TRANS](./SSH-TRANS.md)       |
                | SSH_MSG_USERAUTH_REQUEST          | 50    | [SSH-USERAUTH](./SSH-USERAUTH.md) |
                | SSH_MSG_USERAUTH_FAILURE          | 51    | [SSH-USERAUTH](./SSH-USERAUTH.md) |
                | SSH_MSG_USERAUTH_SUCCESS          | 52    | [SSH-USERAUTH](./SSH-USERAUTH.md) |
                | SSH_MSG_USERAUTH_BANNER           | 53    | [SSH-USERAUTH](./SSH-USERAUTH.md) |
                | SSH_MSG_GLOBAL_REQUEST            | 80    | [SSH-CONNECT](./SSH-CONNECT.md)   |
                | SSH_MSG_REQUEST_SUCCESS           | 81    | [SSH-CONNECT](./SSH-CONNECT.md)   |
                | SSH_MSG_REQUEST_FAILURE           | 82    | [SSH-CONNECT](./SSH-CONNECT.md)   |
                | SSH_MSG_CHANNEL_OPEN              | 90    | [SSH-CONNECT](./SSH-CONNECT.md)   |
                | SSH_MSG_CHANNEL_OPEN_CONFIRMATION | 91    | [SSH-CONNECT](./SSH-CONNECT.md)   |
                | SSH_MSG_CHANNEL_OPEN_FAILURE      | 92    | [SSH-CONNECT](./SSH-CONNECT.md)   |
                | SSH_MSG_CHANNEL_WINDOW_ADJUST     | 93    | [SSH-CONNECT](./SSH-CONNECT.md)   |
                | SSH_MSG_CHANNEL_DATA              | 94    | [SSH-CONNECT](./SSH-CONNECT.md)   |
                | SSH_MSG_CHANNEL_EXTENDED_DATA     | 95    | [SSH-CONNECT](./SSH-CONNECT.md)   |
                | SSH_MSG_CHANNEL_EOF               | 96    | [SSH-CONNECT](./SSH-CONNECT.md)   |
                | SSH_MSG_CHANNEL_CLOSE             | 97    | [SSH-CONNECT](./SSH-CONNECT.md)   |
                | SSH_MSG_CHANNEL_REQUEST           | 98    | [SSH-CONNECT](./SSH-CONNECT.md)   |
                | SSH_MSG_CHANNEL_SUCCESS           | 99    | [SSH-CONNECT](./SSH-CONNECT.md)   |
                | SSH_MSG_CHANNEL_FAILURE           | 100   | [SSH-CONNECT](./SSH-CONNECT.md)   |

        3. Future Assignments
            * Requests for assignments of new message numbers in the range of 1 - 29, 50 - 59, and 80 - 127 MUST be done through the STANDARDS ACTION method, as described in [RFC 2434](https://datatracker.ietf.org/doc/html/rfc2434)

            * The meanings of message numbers in the range 30-49 are specific to the key exchange method in use.
                * Their meaning will be specified by the definition of that method

            * The meanings of message numbers in the range 60-79 are specific to the user authentication method in use
                * Their meaning will be specified by the definition of that method

            * Requests for assignments of NEW message numbers in the range 128-191 MUST be done through the IETF CONSENSUS method,
                as described in [RFC 2434](https://datatracker.ietf.org/doc/html/rfc2434)

            * The IANA will not control message numbers in the range of 192-255
                * This range is left for PRIVATE USE
    
    2. Disconnection Messages Reason Codes and Descriptions

        0. Disconnection Messages Reason Codes and Descriptions
            * Disconnection Message 'reason code' is a uint32
            * Disconnection Message 'description' is a human-readable message that describes the disconnect reason

        1. Conventions
            * Protocol packets containing the SSH_MSG_DISCONNECT message MUST have Disconnection Message 'reason code' values
                in the range of 0x00000001 to 0xFFFFFFFF, which are described in [SSH-TRANS](./SSH-TRANS.md)

        2. Initial Assignments
            * The following table identifies the initial assignments of the SSH_MSG_DISCONNECT 'description' and 'reason code' values:
                | Symbolic Name                                 | Reason Code |
                |-----------------------------------------------|-------------|
                | SSH_DISCONNECT_HOST_NOT_ALLOWED_TO_CONNECT    | 1           |
                | SSH_DISCONNECT_PROTOCOL_ERROR                 | 2           |
                | SSH_DISCONNECT_KEY_EXCHANGE_FAILED            | 3           |
                | SSH_DISCONNECT_RESERVED                       | 4           |
                | SSH_DISCONNECT_MAC_ERROR                      | 5           |
                | SSH_DISCONNECT_COMPRESSION_ERROR              | 6           |
                | SSH_DISCONNECT_SERVICE_NOT_AVAILABLE          | 7           |
                | SSH_DISCONNECT_PROTOCOL_VERSION_NOT_SUPPORTED | 8           |
                | SSH_DISCONNECT_HOST_KEY_NOT_VERIFIABLE        | 9           |
                | SSH_DISCONNECT_CONNECTION_LOST                | 10          |
                | SSH_DISCONNECT_BY_APPLICATION                 | 11          |
                | SSH_DISCONNECT_TOO_MANY_CONNE* Disconnection Message 'reason code' values MUST be assigned sequentially

            * Requests for assignments of new Disconnection Message 'reason code' values, and their associated
                Disconnection Message 'description' text, in the range of 0x00000010 through 0xFDFFFFFF, MUST be done
                through the IETF CONSENSUS method, as described in [RFC 2434](https://datatracker.ietf.org/doc/html/rfc2434)
            
            * Disconnection Message 'reason code' values in the range of 0xFE000000 - 0x0xFFFFFFFF are left for PRIVATE USE,
                as described in [RFC 2434](https://datatracker.ietf.org/doc/html/rfc2434)CTIONS           | 12          |
                | SSH_DISCONNECT_AUTH_CANCELLED_BY_USER         | 13          |
                | SSH_DISCONNECT_NO_MORE_AUTH_METHODS_AVAILABLE | 14          |
                | SSH_DISCONNECT_ILLEGAL_USER_NAME              | 15          |

        3. Future Assignments
            * See original document for details

    3. Channel Connection Failure Reason Codes and Descriptions

        0. Channel Connection Failure Reason Codes and Descriptions
            * Channel Connection Failure 'reason code' is a uint32 value
            * Channel Connection Failure 'description' string is a human- readable message that describes the channel
                connection failure reason
            * These are described in [SSH-CONNECT](./SSH-CONNECT.md)
        
        1. Conventions
            * Protocol packets containing the SSH_MSG_CHANNEL_OPEN_FAILURE message MUST have Connection Failure
                'reason code' values in the range of 0x00000001 - 0xFFFFFFFF
            
        2. Initial Assignments
            | Symbolic Name                        | Reason Code |
            |--------------------------------------|-------------|
            | SSH_OPEN_ADMINISTRATIVELY_PROHIBITED | 1           |
            | SSH_OPEN_CONNECT_FAILED              | 2           |
            | SSH_OPEN_UNKNOWN_CHANNEL_TYPE        | 3           |
            | SSH_OPEN_RESOURCE_SHORTAGE           | 4           |

        3. Future Assignments
            * See original document for details
        
        4. Notes about the PRIVATE USE Range
            * See original document for details
        
    4. Extended Channel Data Transfer data_type_code and Data

        0. Extended Channel Data Transfer data_type_code and Data

            * Extended Channel Data Transfer 'data_type_code' is a uint32 value
            
            * Extended Channel Data Transfer 'data' is a human-readable message that describes the type of data allowed to be
                transferred in the channel

        1. Conventions
            * Protocol packets containing the SSH_MSG_CHANNEL_EXTENDED_DATA message MUST have Extended Channel Data Transfer
                'data_type_code' values in the range of 0x00000001 - 0xFFFFFFFF.  This is described in [SSH-CONNECT](./SSH-CONNECT.md).

        2. Initial Assignments
            | Symbolic Name            | data_type_code |
            |--------------------------|----------------|
            | SSH_EXTENDED_DATA_STDERR | 1              |

        3. Future Assignments
            * See original document for details

    5. Pseudo-Terminal Encoded Terminal Modes

        0. Pseudo-Terminal Encoded Terminal Modes
            * SSH_MSG_CHANNEL_REQUEST messages with a "pty-req" string MUST contain
                'encoded terminal modes'

            * 'encoded_terminal_modes' is a byte stream of opcode-argument pairs
        
        1. Conventions
            * Protocol packets containing the SSH_MSG_CHANNEL_REQUEST
                message with a "pty-req" string MUST contain an 'encoded
                terminal modes' value.
            * Opcode values consist of a single byte, and are in the range
                1 - 255
                * Opcodes 1 - 159 have a uint32 argument
                * Opcodes 160 - 255 are not yet defined
            
        2. Initial Assignments
            | Opcode | Mnemonic      | Description                                                                                                                       |
            |--------|---------------|-----------------------------------------------------------------------------------------------------------------------------------|
            | 0      | TTY_OP_END    | Indicates end of options.                                                                                                         |
            | 1      | VINTR         | Interrupt character; 255 if none.  Similarly for the other characters.  Not all of these characters are supported on all systems. |
            | 2      | VQUIT         | The quit character (sends SIGQUIT signal on POSIX systems).                                                                       |
            | 3      | VERASE        | Erase the character to left of the cursor.                                                                                        |
            | 4      | VKILL         | Kill the current input line.                                                                                                      |
            | 5      | VEOF          | End-of-file character (sends EOF from the terminal).                                                                              |
            | 6      | VEOL          | End-of-line character in addition to carriage return and/or linefeed.                                                             |
            | 7      | VEOL2         | Additional end-of-line character.                                                                                                 |
            | 8      | VSTART        | Continues paused output (normally control-Q).                                                                                     |
            | 9      | VSTOP         | Pauses output (normally control-S).                                                                                               |
            | 10     | VSUSP         | Suspends the current program.                                                                                                     |
            | 11     | VDSUSP        | Another suspend character.                                                                                                        |
            | 12     | VREPRINT      | Reprints the current input line.                                                                                                  |
            | 13     | VWERASE       | Erases a word left of cursor.                                                                                                     |
            | 14     | VLNEXT        | Enter the next character typed literally, even if it is a special character                                                       |
            | 15     | VFLUSH        | Character to flush output.                                                                                                        |
            | 16     | VSWTCH        | Switch to a different shell layer.                                                                                                |
            | 17     | VSTATUS       | Prints system status line (load, command, pid, etc).                                                                              |
            | 18     | VDISCARD      | Toggles the flushing of terminal output.                                                                                          |
            | 30     | IGNPAR        | The ignore parity flag.  The parameter SHOULD be 0 if this flag is FALSE, and 1 if it is TRUE.                                    |
            | 31     | PARMRK        | Mark parity and framing errors.                                                                                                   |
            | 32     | INPCK         | Enable checking of parity errors.                                                                                                 |
            | 33     | ISTRIP        | Strip 8th bit off characters.                                                                                                     |
            | 34     | INLCR         | Map NL into CR on input.                                                                                                          |
            | 35     | IGNCR         | Ignore CR on input.                                                                                                               |
            | 36     | ICRNL         | Map CR to NL on input.                                                                                                            |
            | 37     | IUCLC         | Translate uppercase characters to lowercase.                                                                                      |
            | 38     | IXON          | Enable output flow control.                                                                                                       |
            | 39     | IXANY         | Any char will restart after stop.                                                                                                 |
            | 40     | IXOFF         | Enable input flow control.                                                                                                        |
            | 41     | IMAXBEL       | Ring bell on input queue full.                                                                                                    |
            | 50     | ISIG          | Enable signals INTR, QUIT, [D]SUSP.                                                                                               |
            | 51     | ICANON        | Canonicalize input lines.                                                                                                         |
            | 52     | XCASE         | Enable input and output of uppercase characters by preceding their lowercase equivalents with "\".                                |
            | 53     | ECHO          | Enable echoing.                                                                                                                   |
            | 54     | ECHOE         | Visually erase chars.                                                                                                             |
            | 55     | ECHOK         | Kill character discards current line.                                                                                             |
            | 56     | ECHONL        | Echo NL even if ECHO is off.                                                                                                      |
            | 57     | NOFLSH        | Don't flush after interrupt.                                                                                                      |
            | 58     | TOSTOP        | Stop background jobs from output.                                                                                                 |
            | 59     | IEXTEN        | Enable extensions.                                                                                                                |
            | 60     | ECHOCTL       | Echo control characters as ^(Char).                                                                                               |
            | 61     | ECHOKE        | Visual erase for line kill.                                                                                                       |
            | 62     | PENDIN        | Retype pending input.                                                                                                             |
            | 70     | OPOST         | Enable output processing.                                                                                                         |
            | 71     | OLCUC         | Convert lowercase to uppercase.                                                                                                   |
            | 72     | ONLCR         | Map NL to CR-NL.                                                                                                                  |
            | 73     | OCRNL         | Translate carriage return to newline (output).                                                                                    |
            | 74     | ONOCR         | Translate newline to carriage return-newline (output).                                                                            |
            | 75     | ONLRET        | Newline performs a carriage return (output).                                                                                      |
            | 90     | CS7           | 7 bit mode.                                                                                                                       |
            | 91     | CS8           | 8 bit mode.                                                                                                                       |
            | 92     | PARENB        | Parity enable.                                                                                                                    |
            | 93     | PARODD        | Odd parity, else even.                                                                                                            |
            | 128    | TTY_OP_ISPEED | Specifies the input baud rate in bits per second.                                                                                 |
            | 129    | TTY_OP_OSPEED | Specifies the output baud rate in bits per second.                                                                                |

        3. Future Assignments
            * See original document for details

    6. Names

        0. Names
            * In the following sections, the values for the name spaces are
                textual.
    
        1. Conventions For Names
            * All names registered by the IANA in the following sections
                MUST be printable US-ASCII strings
                * They MUST NOT contain '@' (at-sign), ',' (comma), whitespace,
                    control characters (ASCII codes <= 32), or ASCII code 127 (\<DEL>)

            * Names are case-sensitive, and MUST NOT be longer than 64 characters

            * Names with '@' in them indicate locally extensible names:
                * The IANA will neither register nor control names with the at-sign in them
            
                * They have the format "name@domainname", where:
                    1. the part preceding the at-sign is the name (US-ASCII printable). The character exclusions provided above apply, the only difference being that a single '@' is allowed.
                    
                    2. the part following the at-sign MUST be a valid, fully qualified internet domain name [RFC1034](https://www.rfc-editor.org/rfc/rfc1034) controlled by the person or organization defining the name

                * An example of a locally defined name is "ourcipher-cbc@example.com" (without the double quotes).
            
        2. Future Assignments of Names
            * Requests for assignments of new names MUST be done through the IETF 
                CONSENSUS method, as described in
                [RFC2434](https://www.rfc-editor.org/rfc/rfc2434).

    7. Service Names

        0. Service Names (initial assignments)
            * 'service name' is used to describe a protocol layer

            * Initial assignments are as follows:
                | Service Name   | Reference                         |
                |----------------|-----------------------------------|
                | ssh-userauth   | [SSH-USERAUTH](./SSH-USERAUTH.md) |
                | ssh-connection | [SSH-CONNECT](./SSH-CONNECT.md)   |

    8. Authentication Method Names

        0. Authentication Method Names (initial assignments)
            * The Authentication Method Name is used to describe an authentication 
                method for the "ssh-userauth" service [SSH-USERAUTH](./SSH-USERAUTH.md).
            
            * Initial assignments are as follows:
                | Method Name | Reference                                        |
                |-------------|--------------------------------------------------|
                | publickey   | [[SSH-USERAUTH](./SSH-USERAUTH.md), Section 7]   |
                | password    | [[SSH-USERAUTH](./SSH-USERAUTH.md), Section 8]   |
                | hostbased   | [[SSH-USERAUTH](./SSH-USERAUTH.md), Section 9]   |
                | none        | [[SSH-USERAUTH](./SSH-USERAUTH.md), Section 5.2] |
            
    9. Connection Protocol Assigned Names
        
        0. Connection Protocol Assigned Names
            * Apparent errata in the original document. In this section, it is stated
                that "The following table lists the initial assignments to the Connection
                Protocol Type and Request names". However, no such table is provided.
        
        1. Connection Protocol Channel Types
            * Initial assignments are as follows:
                | Channel Type    | Reference                                        |
                |-----------------|--------------------------------------------------|
                | session         | [[SSH-CONNECT](./SSH-CONNECT.md), Section 6.1]   |
                | x11             | [[SSH-CONNECT](./SSH-CONNECT.md), Section 6.3.2] |
                | forwarded-tcpip | [[SSH-CONNECT](./SSH-CONNECT.md), Section 7.2]   |
                | direct-tcpip    | [[SSH-CONNECT](./SSH-CONNECT.md), Section 7.2]   |
            
        2. Connection Protocol Global Request Names
            | Request Type         | Reference                                      |
            |----------------------|------------------------------------------------|
            | tcpip-forward        | [[SSH-CONNECT](./SSH-CONNECT.md), Section 7.1] |
            | cancel-tcpip-forward | [[SSH-CONNECT](./SSH-CONNECT.md), Section 7.1] |
        
        3. Connection Protocol Channel Request Names
            | Request Type  | Reference                                        |
            |---------------|--------------------------------------------------|
            | pty-req       | [[SSH-CONNECT](./SSH-CONNECT.md), Section 6.2]   |
            | x11-req       | [[SSH-CONNECT](./SSH-CONNECT.md), Section 6.3.1] |
            | env           | [[SSH-CONNECT](./SSH-CONNECT.md), Section 6.4]   |
            | shell         | [[SSH-CONNECT](./SSH-CONNECT.md), Section 6.5]   |
            | exec          | [[SSH-CONNECT](./SSH-CONNECT.md), Section 6.5]   |
            | subsystem     | [[SSH-CONNECT](./SSH-CONNECT.md), Section 6.5]   |
            | window-change | [[SSH-CONNECT](./SSH-CONNECT.md), Section 6.7]   |
            | xon-xoff      | [[SSH-CONNECT](./SSH-CONNECT.md), Section 6.8]   |
            | signal        | [[SSH-CONNECT](./SSH-CONNECT.md), Section 6.9]   |
            | exit-status   | [[SSH-CONNECT](./SSH-CONNECT.md), Section 6.10]  |
            | exit-signal   | [[SSH-CONNECT](./SSH-CONNECT.md), Section 6.10]  |

        4. Initial Assignment of Signal Names
            | Signal | Reference                       |
            |--------|---------------------------------|
            | ABRT   | [SSH-CONNECT](./SSH-CONNECT.md) |
            | ALRM   | [SSH-CONNECT](./SSH-CONNECT.md) |
            | FPE    | [SSH-CONNECT](./SSH-CONNECT.md) |
            | HUP    | [SSH-CONNECT](./SSH-CONNECT.md) |
            | ILL    | [SSH-CONNECT](./SSH-CONNECT.md) |
            | INT    | [SSH-CONNECT](./SSH-CONNECT.md) |
            | KILL   | [SSH-CONNECT](./SSH-CONNECT.md) |
            | PIPE   | [SSH-CONNECT](./SSH-CONNECT.md) |
            | QUIT   | [SSH-CONNECT](./SSH-CONNECT.md) |
            | SEGV   | [SSH-CONNECT](./SSH-CONNECT.md) |
            | TERM   | [SSH-CONNECT](./SSH-CONNECT.md) |
            | USR1   | [SSH-CONNECT](./SSH-CONNECT.md) |
            | USR2   | [SSH-CONNECT](./SSH-CONNECT.md) |

        5. Connection Protocol Subsystem Names
            * There are no initial assignments of the Connection Protocol Subsystem Names.

    10. Key Exchange Method Names
        
        0. Key Exchange Method Names
            * The name "diffie-hellman-group1-sha1" is used for a key exchange method using an Oakley group, as defined in [RFC2409](https://www.rfc-editor.org/rfc/rfc2409)
            
