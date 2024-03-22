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
                | SSH_DISCONNECT_TOO_MANY_CONNECTIONS           | 12          |
                | SSH_DISCONNECT_AUTH_CANCELLED_BY_USER         | 13          |
                | SSH_DISCONNECT_NO_MORE_AUTH_METHODS_AVAILABLE | 14          |
                | SSH_DISCONNECT_ILLEGAL_USER_NAME              | 15          |

        3. Future Assignments
            * Disconnection Message 'reason code' values MUST be assigned sequentially

            * Requests for assignments of new Disconnection Message 'reason code' values, and their associated
                Disconnection Message 'description' text, in the range of 0x00000010 through 0xFDFFFFFF, MUST be done
                through the IETF CONSENSUS method, as described in [RFC 2434](https://datatracker.ietf.org/doc/html/rfc2434)
            
            * Disconnection Message 'reason code' values in the range of 0xFE000000 - 0x0xFFFFFFFF are left for PRIVATE USE,
                as described in [RFC 2434](https://datatracker.ietf.org/doc/html/rfc2434)

    3. Channel Connection Failure Reason Codes and Descriptions

        0. Channel Connection Failure Reason Codes and Descriptions
            * Channel Connection Failure 'reason code' is a uint32 value
            * Channel Connection Failure 'description' text is a human- readable message that describes the channel
                connection failure reason
            * These are described in [SSH-CONNECT](./SSH-CONNECT.md)
        
        1. Conventions
            * Protocol packets containing the SSH_MSG_CHANNEL_OPEN_FAILURE message MUST have Connection Failure
                'reason code' values in the range of 0x00000001 - 0xFFFFFFFF
            
        2. Initial Assignments
        