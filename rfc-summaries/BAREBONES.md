# BAREBONES - Minimal Synthesis of the SSH RFC Collection
This document aims to synthesize the information from each of the SSH RFC documents into one unified piece in order to ease the development of CrabbySSH.

It includes the bare minimum information required to implement the SSH protocol.

Note that CrabbySSH is not (currently) concerned with working with any
non-2.0 SSH implementations, and so those considerations are excluded

## SSH-ARCH
1. Chosen trust model: Local Database
    * Client-side local database associates host name with public host key

2. Data types:
    | Identifier | Description |
    |-|-|
    | byte | 8-bit value |
    | boolean | 8-bit value; 0 == FALSE, 1 == TRUE; values != 0 interpreted as TRUE |
    | uint32 | 32-bit unsigned integer; four bytes, big-endian; E.g., the value `699921578 (0x29b7f4aa)` is stored as `29 b7 f4 aa` |
    | uint64 | 64-bit unsigned integer; eight bytes, big-endian |
    | string | Arbitrary length, contains arbitrary data; terminating null characters NOT used; US-ASCII for internal names; UTF-8 used for text that may be displayed; E.g., the string "testing now" is represented as `00 00 00 0B t e s t i n g 20 n o w` |
    | mpint | Multiple precision integer in 2's complement; Stored as `string`; 8 bits per byte; Big-endian; 0 stored as `string` with zero bytes of data; Unnecessary leading bytes with the value 0 or 255 MUST NOT be included; See original document for examples |
    | name-list | string comma-separated list of names; represented by a uint32 containing its length (number of bytes that follow) followed by a comma-separated list of zero or more names; each name MUST have a non-zero length, MUST NOT contain a comma; each name in list is US-ASCII |

3. Message Numbers
    * SSH message numbers are in the range 1-255, allocated as follows:
        * Transport Layer Protocol:
            | Range | Description |
            | ----- | ----------- |
            | 1-19  | Transport layer generic (e.g., disconnect; ignore; debug; etc... |
            | 20-29 | Algorithm negotiation |
            | 30-49 | Key exchange method specific (numbers can be reused for different authentication methods) |
            
        * User Authentication Protocol:
            | Range | Description |
            | ----- | ----------- |
            | 50-59 | User authentication generic |
            | 60-79 | User authentication method specific (numbers can be reused for different authentication methods) |
            
        * Connection Protocol:   
            | Range | Description |
            | ----- | ----------- |
            | 80-89 | User authentication generic |
            | 90-127 | Channel related messages |
        
        * Reserved for client protocols:
            | Range | Description |
            | ----- | ----------- |
            | 128-191 | Reserved |
        * Local extensions:
            | Range | Description |
            | ----- | ----------- |
            | 192-255 | Local extensions |

4. Control Character Filtering
    * When displaying text to a user, such as error or debug messages, the Client software SHOULD replace any control characters (excluding tab, <CR>, and newline) with safe sequences to avoid attacks by sending terminal control characters

## SSH-TRANS
1. Connection Setup
    * Server listens on port 22
    
    1. Establish TCP connection
    
    2. Send ID string to partner
        * Null character MUST NOT be sent
        
        * Max length of ID string is 255 characters INCLUDING \<CR>\<LF>
        
        * Server MAY send other lines of data prior to ID string
            * See [SSH-ARCH](./SSH-ARCH.md) for details
        
        * ID string example:
            `SSH-2.0-CrabbySSH_1.0.0<SP>Optional comments go here<CR><LF>`

        * Key exchange begins immediately after this identifier is sent.
        All packets following the ID string SHALL use the binary packet 
        protocol described in section 6.

2. Key Exchange
    * CrabbySSH uses Explicit Server Authentication
    
    1. Each side sends the folowing SSH_MSG_KEXINIT packet:
        | Type      | Value                                   |
        |-----------|-----------------------------------------|
        | byte      | SSH_MSG_KEXINIT                         |
        | byte[16]  | cookie (random bytes)                   |
        | name-list | kex_algorithms                          |
        | name-list | server_host_key_algorithms              |
        | name-list | encryption_algorithms_client_to_server  |
        | name-list | encryption_algorithms_server_to_client  |
        | name-list | mac_algorithms_client_to_server         |
        | name-list | mac_algorithms_server_to_client         |
        | name-list | compression_algorithms_client_to_server |
        | name-list | compression_algorithms_server_to_client |
        | name-list | languages_client_to_server              |
        | name-list | languages_server_to_client              |
        | boolean   | first_kex_packet_follows                |
        | uint32    | 0   (reserved for future extension)     |
        * The first algorithm in each name-list MUST be the preferred (guessed) algorithm

    
    2. Run Key Exchange Algorithm
        * This may involve SEVERAL packet exchanges, as specified by the key exchange method.

        1. The key exchange algorithm produces:
            * K - shared secret
            * H - exchange hash
                * Used as the SESSION IDENTIFIER (unique id for this connection)
                * Once computed, the SESSION IDENTIFIER never changes, even if key re-exchange occurs
            
        2. The key exchange algorithm's HASH function is used to compute encryption keys:
            | Step | Description                                         | HASH function call                      |
            |------|-----------------------------------------------------|-----------------------------------------|
            | 1    | Initial IV (initialization vector) Client to Server | HASH(K \|\| H \|\| "A" \|\| session_id) |
            | 2    | Initial IV Server to Client                         | HASH(K \|\| H \|\| "B" \|\| session_id) |
            | 3    | Encryption key Client to Server                     | HASH(K \|\| H \|\| "C" \|\| session_id) |
            | 4    | Encryption key Server to Client                     | HASH(K \|\| H \|\| "D" \|\| session_id) |
            | 5    | Integrity key Client to Server                      | HASH(K \|\| H \|\| "E" \|\| session_id) |
            | 6    | Integrity key Server to Client                      | HASH(K \|\| H \|\| "F" \|\| session_id) |
            * \|\| indicates concatenation
            * K encoded as mpint
            * H is the exchange hash
            * "A" (ASCII 65) encoded as byte
            * session_id encoded as raw data

            Notes:
            * Key data MUST be taken from the beginning of the HASH(...) output
                
                * As many bytes as needed are taken from the beginning of the hash
                value

                * If the key length needed is longer than the output of the HASH(...) :
                    * The Key is extended by the computed HASH(...) of the concatenation of K, H, and the entire key so far, and then appending the resulting bytes (as many as HASH generates) to the
                    key.
                    
                    * The above process is repeated until enough key material is
                    available; the key is taken from the beginning of this value.
                    
                    * In other words:
                        > K1 = HASH(K || H || X || session_id)   (X is e.g., "A")

                        > K2 = HASH(K || H || K1)
                        
                        > K3 = HASH(K || H || K1 || K2)

                        > Kn = HASH(K || H || K1 || K2 || K3 || ... || K<sub>n-1</sub>)

                        > key = K1 || K2 || K3 || ... || K<sub>n</sub>

    3. Ending Key Exchange
        * Key exchange ends by each side sending an SSH_MSG_NEWKEYS message
            * This message is sent with the old keys and algorithms
            * All methods sent after this message MUST use the new keys and algorithms
            * When this message is received, the new keys and algorithms MUST be used for
            receiving
            * This message ensures that a party is able to respond with an
            SSH_MSG_DISCONNECT message that the other party can understand if something 
            goes wrong with the key exchange

            | Type | Value |
            | ---- | --------------- |
            | byte | SSH_MSG_NEWKEYS |

    4. Diffie-Hellman Key Exchange Algorithm
        * Provides a shared secret that cannot be determined by either party alone
        * Combined with a signature made with the host key to provide host authentication

        Identifiers for the DH steps are as follows:
        | identifier | description                                           |
        |------------|-------------------------------------------------------|
        | C          | Client                                                |
        | S          | Server                                                |
        | p          | large safe prime                                      |
        | g          | generator for a subgroup of GF(p) (Galois field of p) |
        | q          | order of the subgroup                                 |
        | V_S        | ID string of S                                        |
        | V_C        | ID string of C                                        |
        | K_S        | Public host key of S                                  |
        | I_C        | C's SSH_MSG_KEXINIT message                           |
        | I_S        | S's SSH_MSG_KEXINIT message                           |
        
        * Note: Both I_C and I_S have been exchanged before this part begins
            
        DH Steps:

        1. Client (C):
            * generate random number x, where 1 < x < q
            * e = g<sup>x</sup> mod p
            * C sends e to S
        
        2. Server (S):
            * generate random number y, where 0 < y < q
            * f = g<sup>y</sup> mod p
            * receive e from C
            * K = e<sup>y</sup> mod p
            * H = HASH(V_C || V_S || I_C || I_S || K_S || e || f || k)
            * S sends (K_S || f || s) to C
            * <b>NOTE:</b> signing operation may involve a 2nd hashing operation

        3. C verifies K_S is really the host key for S using Local Database
            * <b>NOTE: </b>C may accept key without verification, if first-time connecting
            * K = f<sup>x</sup> mod p
            * H = HASH(V_C || V_S || I_C || I_S || K_S || e || f || K )
            * C verifies signature s on H

        * <b>NOTE:</b>Values of 'e' or 'f' MUST BE in range [1, p-1]
            * If violated, key exchange fails

        The following messages are used in implementation:
        1. C sends:
            | Type  | Value              |
            |-------|--------------------|
            | byte  | SSH_MSG_KEXDH_INIT |
            | mpint | e                  |
        
        2. S responds:
            | Type   | Value                                         |
            |--------|-----------------------------------------------|
            | byte   | SSH_MSG_KEXDH_REPLY                           |
            | string | server public host key and certificates (K_S) |
            | mpint  | f                                             |
            | string | signature of H                                |

        3. Hash H is computed as the HASH hash of the concatenation of:
            | Type   | Value                                        |
            |--------|----------------------------------------------|
            | string | V_C, Client's ID string, excluding CR and LF |
            | string | V_S, Server's ID string, excluding CR and LF |
            | string | I_C, the payload of Client's SSH_MSG_KEXINIT |
            | string | I_S, the payload of Server's SSH_MSG_KEXINIT |
            | string | K_S, the host key                            |
            | mpint  | e, exchange value sent by Client             |
            | mpint  | f, exchange value sent by Server             |
            | mpint  | k, the shared secret                         |

            * I.e., H = HASH(V_C \|\| V_S \|\| I_C \|\| I_S \|\| K_S \|\| e \|\| f \|\| k)

            * This value, H, is called the exchange hash
                * It is used to authenticate the key exchange
                * It SHOULD be kept secret
            
            * The signature algorithm MUST be applied over H, not the original
            data
                * Most signature algorithms include hashing and additional padding
                (e.g., "ssh-dss" specifies SHA-1 hashing).
                    * In this case, the data is first hashed with HASH to compute H,
                    and then H is hashed with SHA-1 as part of the signing operation
        
        * Two key exchange methods are REQUIRED:

            1. diffie-hellman-group1-sha1 (REQUIRED)
                * Specifies the Diffie-Hellman key exchange with:
                    * SHA-1 as HASH
                    * Oakley Group 2 (1024-bit MODP group; see [RFC2409](https://datatracker.ietf.org/doc/html/rfc2409))
                    
                * Note that although the method includes "group1" in the name, it uses
                Oakley Group 2
        
            2. diffie-hellman-group14-sha1 (REQUIRED)
                * Specifies the Diffie-Hellman key exchange with:
                    * SHA-1 as HASH
                    * Oakley Group 14 (2048-bit MODP group; see [RFC3526](https://datatracker.ietf.org/doc/html/rfc3526))
        
    5. Key Re-Exchange
        * Process:
            1. Send an SSH_MSG_KEXINIT when not already doing a key exchange

            2. When this message is received, a party MUST respond with its own
    SSH_MSG_KEXINIT message
                * EXCEPT when the received SSH_MSG_KEXINIT was already a reply
            
        * Either party MAY initiate the re-exchange, but roles MUST NOT be changed
            * I.e., the Server remains the Server, and the Client remains the Client
        
        * Key re-exchange is performed using whatever encryption was in effect when the
        exchange was started
            * Encryption, compression, and MAC methods are not changed before a new
            SSH_MSG_NEWKEYS is sent after the key exchange (as in the initial key exchange)
            * Re-exchange is processed identifcally to the initial key exchange
                * EXCEPT the session identifier, which will remain unchanged
        * Some or all of the algorithms MAY be changed DURING the re-exchange
            * Host keys can also change
            * All keys and initialization vectors are recomputed after the exchange
            * Compression and encryption contexts are reset
        
        * It is RECOMMENDED that keys be changed after each GIGABYTE of transmitted data
        OR after each HOUR of connection time - whichever comes sooner.
            * However, since the re-exchange is a public key operation, it requires a fair
            amount of processing power and should not be performed too often
        
        * More application data may be sent after the SSH_MSG_NEWKEYS packet has been sent

3. Service Request (following key exchange):
    1. Client Requests Service:
        | Type   | Value                   |
        |--------|-------------------------|
        | byte   | SSH_MSG_SERVICE_REQUEST |
        | string | service name            |
        * Reserved service names:
            * ssh-userauth
            * ssh-connection
        
        * Local services use the PRIVATE USE syntax of "servicename@domain"

    2. Server Responds to Client 
        * If accepted:
            | Type   | Value                  |
            |--------|------------------------|
            | byte   | SSH_MSG_SERVICE_ACCEPT |
            | string | service name           |

        * If rejected:
            * Server sends an appropriate SSH_MSG_DISCONNECT message (see below) and disconnects

4. Additional Messages
    * May be sent by either party at any time
    1. Disconnection Message (SSH_MSG_DISCONNECT)
        | Type   | Value                                                                                               |
        |--------|-----------------------------------------------------------------------------------------------------|
        | byte   | SSH_MSG_DISCONNECT                                                                                  |
        | uint32 | reason code                                                                                         |
        | string | description in ISO-10646 UTF-8 encoding ([RFC 3629](https://datatracker.ietf.org/doc/html/rfc3629)) |
        | string | language tag ([RFC 3066](https://datatracker.ietf.org/doc/html/rfc3066))                            |
        * 'description' string gives the reason for disconnect in a human-readable form
            * If the 'description' string is displayed, the control character filtering discussed in
            [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4251) should be used to
            avoid attacks by sending terminal control characters
        
        * 'reason code' gives the reason in a machine-readable form
            * 'reason code' can have one of the values defined in the table below:
                | Symbolic Name                                 | 'reason code' Value |
                |-----------------------------------------------|---------------------|
                | SSH_DISCONNECT_HOST_NOT_ALLOWED_TO_CONNECT    | 1                   |
                | SSH_DISCONNECT_PROTOCOL_ERROR                 | 2                   |
                | SSH_DISCONNECT_KEY_EXCHANGE_FAILED            | 3                   |
                | SSH_DISCONNECT_RESERVED                       | 4                   |
                | SSH_DISCONNECT_MAC_ERROR                      | 5                   |
                | SSH_DISCONNECT_COMPRESSION_ERROR              | 6                   |
                | SSH_DISCONNECT_SERVICE_NOT_AVAILABLE          | 7                   |
                | SSH_DISCONNECT_PROTOCOL_VERSION_NOT_SUPPORTED | 8                   |
                | SSH_DISCONNECT_HOST_KEY_NOT_VERIFIABLE        | 9                   |
                | SSH_DISCONNECT_CONNECTION_LOST                | 10                  |
                | SSH_DISCONNECT_BY_APPLICATION                 | 11                  |
                | SSH_DISCONNECT_TOO_MANY_CONNECTIONS           | 12                  |
                | SSH_DISCONNECT_AUTH_CANCELLED_BY_USER         | 13                  |
                | SSH_DISCONNECT_NO_MORE_AUTH_METHODS_AVAILABLE | 14                  |
                | SSH_DISCONNECT_ILLEGAL_USER_NAME              | 15                  |
        * Causes immediate termination of the connection
        * All implementations MUST be able to process this message, and SHOULD be able
        to transmit this message
        * The sender MUST NOT send or receive any data after this message
        * The recipient MUST NOT accept any data after receiving this message
    2. Ignored Data Message (SSH_MSG_IGNORE)
        | Type   | Value          |
        |--------|----------------|
        | byte   | SSH_MSG_IGNORE |
        | string | data           |
        * All implementations MUST understand (and ignore) this message at any time (after receiving the
        identification string).
        * No implementation is required to send them.
    
    3. Debug Message (SSH_MSG_DEBUG)
        | Type    | Value                                                                                           |
        |---------|-------------------------------------------------------------------------------------------------|
        | byte    | SSH_MSG_DEBUG                                                                                   |
        | boolean | always_display                                                                                  |
        | string  | message in ISO-10646 UTF-8 encoding ([RFC 3629](https://datatracker.ietf.org/doc/html/rfc3629)) |
        | string  | language tag ([RFC 3066](https://datatracker.ietf.org/doc/html/rfc3066))                        |
        * 'always_display':
            * If TRUE, the message SHOULD be displayed
            * If FALSE, it SHOULD NOT be displayed, unless debugging information has been explcitly requested by the
            user
        * 'message'
            * Does not need to contain a newline
            * Is permitted to consist of multiple lines separated by \<CR>\<LF> pairs
            * If displayed, the terminal control characte filtering discussed in
            [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4251) should be used to avoid
            attacks by sending terminal control characters

    4. Reserved Messages (SSH_MSG_UNIMPLEMENTED)
        | Type   | Value                                      |
        |--------|--------------------------------------------|
        | byte   | SSH_MSG_UNIMPLEMENTED                      |
        | uint32 | packet sequence number of rejected message |
        * An implementation MUST respond to all unrecognized messages with an SSH_MSG_UNIMPLEMENTED message in the order
        in which the messages were received
        * Such messages MUST be otherwise ignored
            
5. Summary of Message Numbers
    | Symbolic Name           | Value |
    |-------------------------|-------|
    | SSH_MSG_DISCONNECT      | 1     |
    | SSH_MSG_IGNORE          | 2     |
    | SSH_MSG_UNIMPLEMENTED   | 3     |
    | SSH_MSG_DEBUG           | 4     |
    | SSH_MSG_SERVICE_REQUEST | 5     |
    | SSH_MSG_SERVICE_ACCEPT  | 6     |
    | SSH_MSG_KEXINIT         | 20    |
    | SSH_MSG_NEWKEYS         | 21    |

    * Numbers 30 - 49 are used for kex packets - different kex methods may reuse message numbers in this range

## SSH-USERAUTH
## SSH-CONNECT
## SSH-NUMBERS