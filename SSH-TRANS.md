## [SSH-TRANS](https://datatracker.ietf.org/doc/html/rfc4253) - The Secure Shell (SSH) Transport Layer Protocol ([RFC 4253](https://datatracker.ietf.org/doc/html/rfc4253)) Summary (by section)
This is a summary of "The Secure Shell (SSH) Transport Layer Protocol" ([RFC 4253](https://datatracker.ietf.org/doc/html/rfc4253))

0. Abstract
    * SSH-TRANS (RFC 4253) details the SSH transport layer protocol, which (usually) runs on top of TCP/IP

1. Introduction
    * Host-based authentication, user authentication not performed
    * Simple and flexible to allow parameter negotiation. The following are all negotiated:
        * Key exchange method
        * Public key algorithm
        * Symmetric encryption algorithm
        * Message authentication algorithm
        * Hash algorithm
    * 2 round-trips expected for full key exchange, server authentication, service request,
    and acceptance notification of service request
    * 3 round-trips is the worst-case for the above
2. Contributors
    * See document
3. Conventions
    * See document
4. Connection Setup
    
    0. Connection Setup
        * SSH works over any 8-bit clean, binary-transparent transport
        * Underlying transport SHOULD protect against transmission errors
        * Errors cause SSH connection to terminate

    1. Use Over TCP/IP
        * Server listens for connections on port 22 (official IANA assignment)

    2. Protocol Version Exchange
        * After connection established, both sides send an identification string (which we will call "ID string" for brevity)
        * Older, undocumented versions of SSH may exclude the \<CR>
        character, and thus any implementation wanting to support said
        versions will want to support the exclusion of \<CR> in processing
        * The null character MUST NOT be sent during exchange
        * The maximum length of the ID string is 255 characters,
        including \<CR> and \<LF>
        * The part of the ID string preceding \<CR> and \<LF> is used in the
        Diffie-Hellman key exchange (see section 8)
        * The Server MAY send other lines of data prior to the ID string.
            * Each line SHOULD end with \<CR>\<LF>
            * Such lines MUST NOT begin with "SSH-"
            * Such lines SHOULD be encoded in ISO-10646 UTF-8 (see [RFC 3629](https://datatracker.ietf.org/doc/html/rfc3629))
            * Clients MUST be able to process such lines
            * Such lines MAY be ignored, or displayed to the Client
                * If displayed, control character filtering (see [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4251)) SHOULD be used. This allows TCP-wrappers to display an error message prior to disconnecting
        * The ID string is structured as follows:
            > SSH-{protoversion}-{softwareversion}\<SP>{comments}\<CR>\<LF>
            
            \<SP> = Space (ASCII 32)

            \<CR> = Carriage Return (ASCII 13)

            \<LF> = Line Feed (ASCII 10) 

            {protoversion} - to be replaced with the SSH protocol version
            (i.e., "2.0" as per this RFC)
            
            {softwareversion} - to be replaced with your software name,version; primarily used to trigger compatibility extensions and
            to indicate implementation capabilities

            {comments} - OPTIONAL; to be replaced with additional information that may be useful
            in solving user problems 

            <b>NOTE:</b> {protoversion} and {softwareversion} MUST consist
            ONLY OF printable US-ASCII characters, with the exception of
            whitespace characters and the minus sign (-), which are also
            valid.
            

        * An example of the above ID string might be:
            > SSH-2.0-crabbySSH_1.0.0\<SP>Hamburgers\<CR>\<LF>

        * Key exchange begins immediately after sending this identifier.
        Thus, all packets following the ID string SHALL use the binary packet protocol, described in Section 6.
    
5. Compatibility with Old SSH Versions
    
    0. Compatibility with Old SSH Versions
        * See document

    1. Old Client, New Server
        * Servers MAY support a configurable compatibility flag enabling 
        compatibility with old versions.
            * When this flag is enabled, the server SHOULD identify its
            {protoversion} as "1.99"
            * Clients using protocol 2.0 MUST be able to identify this as identical to "2.0".
            * In this mode, the server SHOULD NOT send the \<CR> character (ASCII 13) after the ID string
            * In this mode, the server SHOULD NOT send any further data
            after sending its ID string until it has received the Client's
            ID string. Once received, the Server can use the Client's ID 
            string to determine whether the client is using an old protocol,
            and can then revert to the old protocol if required.
        * If compatibility with old clients is NOT required, the server MAY send its initial key
        exchange data immediately after the ID string
    
    2. New Client, Old Server
        * Since the new Client MAY immediately send additional data after its ID string, the old
        protocol may already be corrupt when the Client learns the server is old. When this happens,
        the Client SHOULD close the connection to the server and reconnect using the old protocol.
    3. Packet Size and Overhead
        * This section addresses packet size bloat concerns. See document for more details.

6. Binary Packet Protocol

    0. Binary Packet Protocol
        * Each packet is in the following format:
        ```
        uint32    packet_length
        byte      padding_length
        byte[n1]  payload; n1 = packet_length - padding_length - 1
        byte[n2]  random_padding; n2 = padding_length
        byte[m]   mac (Message Authentication Code - MAC); m = mac_length
        ```
        Where TYPE[n] indicates an array of TYPE having length n (e.g., above, byte[n1] is an array of bytes having length n1)

        packet_length - Length of packet in bytes, excluding 'mac' and 'packet_length' (i.e., itself)

        padding_length - Length of 'random_padding' in bytes

        payload - Useful contents of the packet. If compression has been negotiated, this field is compressed. Initially, compression MUST be "none"

        random_padding - Arbitrary-length padding, such that the total length of the concatenation of
        packet_length, padding_length, payload, and random_padding is a multiple of the cipher block size
        or 8, whichever is larger. There MUST be at least 4 bytes of padding. The padding SHOULD consist
        of random bytes. The maximum amount of padding is 255 bytes.

        mac - "Message Authentication Code". If message authentication has been negotiated, this field
        contains the MAC bytes. Initially, the MAC algorithm MUST be "none"

        <b>Notes:</b>
        * The requirement described in the description of random_padding MUST be enforced
        * 'packet_length' is also encrypted; Processing it requires special care when sending and
        receiving packets
        * Insertion of variable amounts of 'random_padding' may help thwart traffic analysis
        * Minimum size of a packet is 16 (or the cipher block size, whichever is larger) bytes (plus 
        'mac'). Implementations SHOULD decrypt the length after receiving the first 8 (or cipher
        block size, whichever is larger) bytes of a packet.
    
    1. Maximum Packet Length
        * All implementations MUST be able to process packets with:
            * an uncompressed payload length
            of <= 32768 bytes
            * a total packet size of <= 35000 bytes (including 'packet_length', 
            'padding_length', 'payload', 'random_padding', and 'mac')
                * 35000 is an arbitrary choice larger than the uncompressed length above
            * Larger packets SHOULD be supported
                * However, it SHOULD be checked that the packet length is reasonable
                in order to avoid denial of service and/or buffer-overflow attacks.
    2. Compression
        * If compression has been negotiated:
            * 'payload' field will be compressed using the negotiated algorithm.
            * 'packet_length' and 'mac' fields will be computed from the compressed payload
            * Encryption will be done AFTER compression
        * Compression MAY be stateful, depending on the method.
        * Compression MUST be independent for each direction
            * Independent algorithm choice MUST be allowed for each direction.
        * In practice, it is RECOMMENDED that the compression method be the same in both
        directions.
        * Defined compression methods:
            | Name | Necessity | Description                                                                                                                                          |
            |------|-----------|------------------------------------------------------------------------------------------------------------------------------------------------------|
            | zlib | OPTIONAL  | ZLIB (LZ77) compression; See [RFC 1950](https://datatracker.ietf.org/doc/html/rfc1950) and [RFC 1951](https://datatracker.ietf.org/doc/html/rfc1951) |
            | none | REQUIRED  | no compression                                                                                                                                       |
    
    3. Encryption
        * Encryption algorithm and a key are negotiated during key exchange
        * When encryption is in effect, the following fields MUST
        be encrypted with the given algorithm:
            * packet_length
            * padding_length
            * payload
            * padding
        * Encrypted data in all packets sent in one direction
        SHOULD be considered a single data stream. E.g.,
        initialization vectors SHOULD be passed from the end of
        one packet to the beginning of the next packet
        * All ciphers SHOULD use keys with an effective key length >= 128 bits
        * The ciphers in each direction MUST run independently of each other.
        * Implementations MUST allow the algorithm for each
        direction to be independently selected.
            * However, in practice it is RECOMMENDED that the same
            algorithm be used in both directions.
        * The following algorithms are defined:
                | Name           | Necessity   | Description                                                                |
                |----------------|-------------|----------------------------------------------------------------------------|
                | 3des-cbc       | REQUIRED    | three-key 3DES in CBC mode                                                 |
                | blowfish-cbc   | OPTIONAL    | Blowfish in CBC mode                                                       |
                | twofish256-cbc | OPTIONAL    | Twofish in CBC mode,  with a 256-bit key                                   |
                | twofish-cbc    | OPTIONAL    | alias for "twofish256-cbc" (this is being retained for historical reasons) |
                | twofish192-cbc | OPTIONAL    | Twofish with a 192-bit key                                                 |
                | twofish128-cbc | OPTIONAL    | Twofish with a 128-bit key                                                 |
                | aes256-cbc     | OPTIONAL    | AES in CBC mode, with a 256-bit key                                        |
                | aes192-cbc     | OPTIONAL    | AES with a 192-bit key                                                     |
                | aes128-cbc     | RECOMMENDED | AES with a 128-bit key                                                     |
                | serpent256-cbc | OPTIONAL    | Serpent in CBC mode, with a 256-bit key                                    |
                | serpent192-cbc | OPTIONAL    | Serpent with a 192-bit key                                                 |
                | serpent128-cbc | OPTIONAL    | Serpent with a 128-bit key                                                 |
                | arcfour        | OPTIONAL    | the ARCFOUR stream cipher with a 128-bit key                               |
                | idea-cbc       | OPTIONAL    | IDEA in CBC mode                                                           |
                | cast128-cbc    | OPTIONAL    | CAST-128 in CBC mode                                                       |
                | none           | OPTIONAL    | no encryption - NOT RECOMMENDED                                            |

                Notes on algorithms:
            1. "3des-cbc" - a three-key triple-DES (encrypt-decrypt-encrypt) algorithm:
                * Block cipher with 8-byte blocks
                * The first 8 bytes of the key are used for the first encryption
                * The next 8 bytes for the decryption
                * The following 8 bytes for the final encryption
                * The above require 24 bytes of key data, of which 168 bits are
                actually used
                * To implement CBC mode, outer chaining MUST be used (only one initialization vector)
                * See [FIPS-46-3](https://datatracker.ietf.org/doc/html/rfc4253#ref-FIPS-46-3)
                * Since this algorithm only has an effective key length of 112 bits,
                it does not meet the specifications that SSH encryption algorithms 
                should use keys of 128-bits or more. Nonetheless, this algorithm is
                still REQUIRED for historical reasons. In the future, another
                algorithm with better strength may make 3des-cbc deprecated.

            2. "blowfish-cbc" - Blowfish in CBC mode:
                * 128-bit keys
                * Block cipher with 8-byte blocks
            
            3. "twofish-cbc" AKA "twofish256-cbc" - Twofish in CBC mode:
                * 256-bit keys
                * Block-cipher with 16-byte blocks
                * See [TWOFISH](https://datatracker.ietf.org/doc/html/rfc4253#ref-TWOFISH)
            4. "twofish192-cbc":
                * Same as above, but with 192-bit key
            5. "twofish128-cbc":
                * Same as above, but with 128-bit key
            6. "aes256-cbc" - AES in CBC mode
                * 256-bit key
                * See [FIPS-197](https://datatracker.ietf.org/doc/html/rfc4253#ref-FIPS-197)
            7. "aes192-cbc":
                * Same as above, but with 192-bit key
            8. "aes128-cbc":
                * Same as above, but with 128-bit key
            9. "serpent256-cbc" - Serpent in CBC mode:
                * 256-bit key as described in the Serpent AES submission
            10. "serpent192-cbc":
                * Same as above, but with 192-bit key
            11. "serpent128-cbc":
                * Same as above, but with 128-bit 
            12. "arcfour" - Arcfour stream cipher
                * 128-bit keys
                * Believed to be compatible with RC4 cipher
                * USE WITH CAUTION - has problems with weak keys
            13. "idea-cbc" - IDEA Cipher in CBC mode
            14. "cast128-cbc" - Cast-128 Cipher in CBC mode
                * 128-bit key
            15. "none" - no encryption to be done
                * NOT RECOMMENDED - no confidentiality protection
                * Some functionality (e.g., password authentication) may
                be disabled for security reasons if this cipher is chosen.
            
            <b>NOTE:</b> Additional methods may be defined as specified in
            [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4251) and [SSH-NUMBERS](https://datatracker.ietf.org/doc/html/rfc4250)

    4. Data Integrity
        * Each packet includes a MAC computed from a shared securet, packet sequence 
        number, and the contents of the packet, used for data integrity protection.
        * The message authentication algorithm and key are negotiated during key exchange.
        * Initially, no MAC algorithm will be in effect, and its length MUST be zero.
        * After key exchange, the 'mac' for the chosen MAC algorithm is computed before encryption from the concatenation of packet data, e.g.:
            > mac = MAC(key, sequence_number || unencrypted_packet)
            * Where:
                * || indicates concatenation    
                * unencrypted_packet is:
                    * the length fields
                    * 'payload'
                    * 'random_padding'
                * sequence_number is:
                    * implicit packet sequence number represented as uint32
                    * initialized to zero for the first packet
                    * incremented after every packet - regardless of whether
                    encryption or MAC is in use
                    * never reset, even if keys/algorithms are renegotiated
                    * wraps to zero after every 2^32 packets
                    * is not included in the packet sent over the wire
        * MAC algorithms for each direction MUST run independently, and MUST
        alllow choosing the algorithm independently for both directions. In practice, however, it is RECOMMENDED that the same algorithm be used in both directions.
        * The value of 'mac' resulting from the MAC algorithm MUST be transmitted
        without unencryption as the LAST part of the packet. The number of 'mac' bytes
        is dependent on the chosen algorithm
        * The following MAC algorithms are defined:
            | Name         | Necessity   | Description                                                      |
            |--------------|-------------|------------------------------------------------------------------|
            | hmac-sha1    | REQUIRED    | HMAC-SHA1 (digest length = key length = 20)                      |
            | hmac-sha1-96 | RECOMMENDED | first 96 bits of HMAC-SHA1 (digest length = 12, key length = 20) |
            | hmac-md5     | OPTIONAL    | HMAC-MD5 (digest length = key length = 16)                       |
            | hmac-md5-96  | OPTIONAL    | first 96 bits of HMAC-MD5 (digest length = 12, key length = 16)  |
            | none         | OPTIONAL    | no MAC; NOT RECOMMENDED                                          |
            * "hmac-*" algorithms described in [RFC 2104](https://datatracker.ietf.org/doc/html/rfc2104)
            * "*-n" MACs (e.g., hmac-sha1-96, hmac-md5-96) use only the first n bits of the resulting value
            * SHA-1 is decribed in [FIPS-180-2](https://datatracker.ietf.org/doc/html/rfc4253#ref-FIPS-180-2)
            * MD5 is described in [RFC 1321](https://datatracker.ietf.org/doc/html/rfc1321)
            * Additional methods may be defined as specified in [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4251) and in [SSH-NUMBERS](https://datatracker.ietf.org/doc/html/rfc4250)

    5. Key Exchange Methods
        * Specifies how one-time session keys are generated for encryption and 
        authentication, and how the server authentication is done.
        * Two REQUIRED key exchange methods are defined:
            | Name                        | Necessity | Description   |
            |-----------------------------|-----------|---------------|
            | diffie-hellman-group1-sha1  | REQUIRED  | See Section 8 |
            | diffie-hellman-group14-sha1 | REQUIRED  | See Section 8 |
            * Additional methods may be defined as specified in [SSH-NUMBERS](https://datatracker.ietf.org/doc/html/rfc4250)
            * The name "diffie-hellman-group1-sha1" is used for a key exchange method
            using an Oakley group as defined in [RFC 2409](https://datatracker.ietf.org/doc/html/rfc2409)
            * SSH maintains its own group identifier space that is logically
            distinct from Oakley [RFC 2412](https://datatracker.ietf.org/doc/html/rfc2412) and IKE. See document for more details.
            * Group names should be treated as opaque identifiers, and should
            not be assumed to have any relationship between groups used by SSH
            and those defined for IKE.
    6. Public Key Algorithms
        * SSH has been designed to operate with almost any public key format, encoding,
        and algorithm (signature and/or encryption)
        * Public key type are defined by:
            1. Key Format
                * How the key is encoded and how certificates are represented. Key blobs
                MAY contain certificates in addition to keys.
            2. Signature and/or Encryption Algorithms
                * Some key types may not support both signing and encryption
                * Key usage may be restricted by policy statements (e.g., in certificates). In this case, different key types SHOULD be defined for
                the different policy alternatives
            3. Encoding of Signatures and/or Encrypted data
                * Includes but is not limited to:
                    * padding
                    * byte order
                    * data formats
        * The following public key and/or certificate formats are defined:
            | Name         | Necessity   | Algorithm Type | Description                    |
            |--------------|-------------|----------------|--------------------------------|
            | ssh-dss      | REQUIRED    | sign           | Raw DSS Key                    |
            | ssh-rsa      | RECOMMENDED | sign           | Raw RSA Key                    |
            | pgp-sign-rsa | OPTIONAL    | sign           | OpenPGP certificates (RSA key) |
            | pgp-sign-dss | OPTIONAL    | sign           | OpenPGP certificates (DSS key) |
            * Additional key types may be defined as specified in [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4251) and [SSH-NUMBERS](https://datatracker.ietf.org/doc/html/rfc4250)
            * Key type MUST always be explicitly known (from algorithm negotiation or another source). It is not normally included in the key blob.
        * Certificates and public keys are encoded as follows:
            | Type    | Value                                 |
            |---------|---------------------------------------------|
            | string  | certificate or public key format identifier |
            | byte[n] | key/certificate data                        |
            * Public key is REQUIRED - used for authentication
            * Certificate part may be a zero-length string 
                * Certificate sequence contained in the certificate blob can be used
                to provide authorization
            * Public key / certificate formats that do not explicitly a signature format
            MUST use the public key / certificate format identifier as the signature identifier.
            * Signatures are encoded as follows:
                | Type    | Value                                                                     |
                |---------|---------------------------------------------------------------------------------|
                | string  | signature format identifier (as specified by the public key/certificate format) |
                | byte[n] | signature blob in format specific encoding.                                     |
            * The following key formats use the "mpint" type, which is defined as follows:
                * multiple precision integer
                * in two's complement format
                * stored as a string
                * 8 bits per byte
                * MSB first
                * Negative numbers have 1 at the MSBfo the first byte of the data partition
                * If the MSB would be set as 1 for a positive number, the number MUST be
                preceeded by a zero byte.
                * Unnecessary leading bytes with values 0 or 255 MUST NOT be included
                * The value zero MUST be stored as a string with zero bytes of data
                * By convention, a nuber that is used in modular computations in Z_n
                SHOULD be represented in the range 0 <= x < n
                
            * The 'ssh-dss' key format has the folowing specific encoding:
                | Type   | Value |
                |--------|-------------|
                | string | "ssh-dss"   |
                | mpint  | g           |
                | mpint  | p           |
                | mpint  | q           |
                | mpint  | y           |
                * Here, 'p', 'q', 'g', and 'y' parameters form the signature key blob.
                * Signing and verifying using this key format is done according to the DSS [FIPS-186-2](https://datatracker.ietf.org/doc/html/rfc4253#ref-FIPS-186-2) using the SHA-1 hash [FIPS-180-2](https://datatracker.ietf.org/doc/html/rfc4253#ref-FIPS-180-2)
                * The resulting signature is encoded as follows:
                    | Type   | Description        |
                    |--------|--------------------|
                    | string | "ssh-dss"          |
                    | string | dss_signature_blog |
                    * The value for 'dss_signature_blob' is encoded as a string containing r, followed by s (two 160-bit integers without
                        lengths or padding, unsigned, and in network-byte order)
            * The 'ssh-rsa' key format has the following specific encoding:
                | Type   | Value |
                |--------|-------------|
                | string | "ssh-rsa"   |
                | mpint  | e           |
                | mpint  | n           |
                * 'e' and 'n' form the signature key blob.
                * Signing and verifying using this key format is preferred according to
                the RSASSA-PKCS1-v1_5 scheme in [RFC3447](https://datatracker.ietf.org/doc/html/rfc3447) using the SHA-1 hash.
                * The resulting signature is encoded as follows:
                    | Type   | Value        |
                    |--------|--------------------|
                    | string | "ssh-rsa"          |
                    | string | rsa_signature_blob |
                    * The value for 'rsa_signature_blob" is encoded as a string
                    containing s (an integer, without lengths or padding, unsigned, and
                    in network-byte order)
            * The 'pgp-sign-rsa' method indicates the certificates, public key, and
            signature are in OpenPGP compatible binary format [RFC 2440](https://datatracker.ietf.org/doc/html/rfc2440). This method indicates that the key is an RSA-key.
            * The 'pgp-sign-dss' is as above, but indicates that the key is a DSS-key.
7. Key Exchange

    0. Key Exchange
        * Key exchange begins by each side sending name-lists of supported algorithms.
        Each side has a preferred algorithm in each category. It is expected that most
        implementations will use the same preferred algorithm at any given time.
        * Each side MAY guess which algorithm the other side is using, and MAY send
        an initial key exchange packet according to the algorithm if appropriate for the preferred method.
            * This guess is considered wrong if:
                1. the key algorithm and/org host-key algorithm is guessed wrong 
                (Server & Client have differing preferred algorithms)
                
                OR

                2. any of the other algorithms cannot be agreed upon (see Section 7.1)

            * Otherwise, the guess is considered correct, and the "optimistically" sent
            packet MUST be handled as the first key exchange packet.
            * However, if the guess was wrong, the "optimistically" sent packets MUST be
            ignored, and the appropriate side MUST send the correct initial packet.
        * A key exchange method uses:
            1. Explicit Server Authentication - if the key exchange messages include a
            signature or other proof of the server's authenticity.
            2. Implicit Server Authentication - if, in order to prove its authenticity, the
            server also has to prove that it knows the shared secret 'K' by sending a
            message and corresponding MAC that the Client can verify.
            * The key exchange method specified by this document uses Explicit Server
            Authentication.
                * However, key exchange methods with Implicit Server Authentication MAY be
                used with this protocol. After key exchange with Implicit Server
                Authentication, the client MUST wait for a response to its service request message before sending any further data.
    
    1. Algorithm Negotiation
        * Key exchange begins by each side sending the following packet:
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
        * Each of the algorithm name--lists MUST be a comma-separated list of algorithm
        names (see Algorithm Naming in [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4251) and additional information in [SSH-NUMBERS](https://datatracker.ietf.org/doc/html/rfc4250)). Each supported
        (allowed) algorithm MUST be listed in order of preference, from most to least.
        * The first algorithm in each name-list MUST be the preferred (guessed) algorithm
        * Each name-list MUST contain at least one algorithm name
        * Packet parameter definitions:
            * cookie
                * MUST be a random value generated by the sender.
                * Makes it impossible for either side to fully determine the keys
                and session identifier.
            * kex_algorithms
                * Key exchange algorithms were defined above.
                * First algorithm MUST be the PREFERRED (guessed) algorithm
                * If both sides make the same guess, that algorithm MUST be used.
                    * Otherwise, the following algorithm MUST be used to choose
                    a key exchange method:
                        1. Iterate over the Client's kex_algorithms
                        2. Choose the first algorithm that satisfies the following
                        conditions:
                            * The server also supports the algorithm.
                            * If the algorithm requires an encryption-capable host key,
                            there is an encryption-capable algorithm on the Server's
                            server_host_key_algorithms that is also supported by the
                            Client.
                            * If the algorithm requires a signature-capable host key,
                            there is a signature-capable algorithm on the Server's
                            server_host_key_algorithms that is also supported by the
                            Client.
                        * If no algorithm satisfying all of these conditions can be
                        found, the connection fails, and both sides MUST disconnect.

            * server_host_key_algorithms
                * A name-list of algorithms supported for the Server host key.
                * Server lists the algorithms for which it has host keys.
                * Client lists the algorithms that it is willing to accept.
                * There MAY be multiple host keys for a host, possibly with different
                algorithms
                * Some host keys may not support both signatures and encryption, and
                thus not all host keys are valid for all key exchange methods. This can
                be discerned from the algorithm.
                * Algorithm selection depends on whether the chosen key exchange
                algorithm requires a signature or an encryption-capable host key.
                    * It MUST be possible to determine this from the public key algorithm
                    name. The first algorithm on the client's name-list that satisfies
                    the requirements and is also supported by the server MUST be chosen.
                    * If no such algorithm is found, both sides MUST disconnect.
            
            * encryption_algorithms
                * A name-list of acceptable symmetric encryption algorithms, AKA ciphers,
                in order of preference.
                * The chosen encryption algorithm to each direction MUST be the first
                algorithm on the Client's name-lsit that is ALSO on the Server's
                name-list.
                    * If no such algorithm is found, both sides MUST disconnect.
                * "none" must be explicitly listed if it is to be acceptable
                * See Section 6.3 for encryption algorithm names

            * mac_algorithms
                * A name-list of acceptable MAC algorithms, in order of preference
                * Client's preference MUST take priorty. I.e., The chosen MAC algorithm 
                MUST be the first algorithm on the Client's name-list that is also
                on the Server's name-list 
                * If there is no such algorithm, both sides MUST disconnect
                * "none" must be explicitly listed if it is to be acceptable
                * See Section 6.4 for MAC algorithm names
            
            * compression_algorithms
                * A name-list of compression algorithms, in order of preference
                * Client's preference MUST take priority. I.e., The chosen compression 
                algorithm MUST be the first algorithm on the Client's name-list that is
                also on the Server's name-list
                * If there is no such algorithm, both sides MUST disconnect
                * See section 6.2 for compression algorithm names
            
            * languages
                * A name-list of language tags, in order of preference
                    * See [RFC 3066](https://datatracker.ietf.org/doc/html/rfc3066)
                    for language identification tags
                * Both parties MAY ignore this name-list
                * SHOULD be empty if there are no language preferences, as defined
                in Section 5 of [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4251)
                * SHOULD NOT be present unless they are known to be needed by the
                sending party
            
            * first_kex_packet_follows
                * Indicates whether a guessed key exchange packet follows
                * TRUE if a guessed packet will be sent
                * FALSE if a guessed packet will NOT be sent
                * Each party will know whether their guess was right after
                receiving the SSH_MSG_KEXINIT packet
                * If the other party's guess was wrong, and this field was TRUE,
                then the next packet MUST be silently ignored
                    * Both sides MUST then act as determined by the negotiated
                    key exchange method
                * If the other party's guess was right, key exchange MSUT continue using
                the guessed packet.

        * After the SSH_MSG_KEXINIT message exchange:
            * The key exchange algorithm is run
            * This may involve several packet exchanges, as specified by the key
            exchange method

        * Once a party has sent a SSH_MSG_KEXINIT message for key exchange
        (or re-exchange), until it has sent a SSH_MSG_NEWKEYS message (see Section 7.3)
        it MUST NOT send any messages other than:
            * Transport Layer Generic Messages (1 to 19)
                * Excluding SSH_MSG_SERVICE_REQUEST and SSH_MSG_SERVICE_ACCEPT, which
                MUST NOT be sent
            * Algorithm Negotiation Messages (20 to 29)
                * Excluding SSH_MSG_KEXINIT messages, which MUST NOT be sent 
            * Specific Key Exchange Method Messages (30 to 49)
        
        * Unrecognized messages should be responded to as defined in Section 11

        * Note that during a key re-exchange, after sending each party MUST be
        prepared to process an arbitrary number of messages that may be in-flight
        before receiving a SSH_MSG_KEXINIT messages from the other party

    2. Output from Key Exchange
        * Key exchange produces two values:
            1. K - a shared secret

            2. H - an exchange hash
                * The exchange hash, H, from the first key exchange is additionally
                used as the session identifier - a unique identifier for this connection
                    * It is used by authentication methods as part of the data that is
                    signed as a proof of possession of a private key
                    * Once computed, the session identifier is not changed, even if
                    keys are later re-exchanged

        * Each key exchange method specifies a hash function used in the key exchange
            * The SAME hash algorithm MUST be used in key derivation

        * Here, the hash algorithm used will be called HASH
            * Encryption keys MUST be computed as HASH, of a known value and K,
            as follows:
                1. Initial IV (initialization vector) Client to Server:
                    * HASH(K || H || "A" || session_id)
                        * K encoded as mpint
                        * H is the exchange hash
                        * "A" (ASCII 65) encoded as byte
                        * session_id encoded as raw data

                2. Initial IV Server to Client
                    * HASH(K || H || "B" || session_id)
                        * All encoded as described above ("B" replacing "A")
                
                3. Encryption key Client to Server
                   * HASH(K || H || "C" || session_id)
                        * All encoded as described above ("C" replacing "B")
                
                4. Encryption key Server to Client
                    * HASH(K || H || "D" || session_id)
                        * All encoded as described above ("D" replacing "C")

                5. Integrity key Client to Server
                    * HASH(K || H || "E" || session_id)
                        * All encoded as described above ("E" replacing "D")

                6. Integrity key Server to Client
                    * HASH(K || H || "F" || session_id)
                        * All encoded as described above ("F" replacing "E")
                
                * Key data MUST be taken from the beginning of the HASH(...) output
                    * As many bytes as needed are taken from the beginning of the hash
                    value
                    * If the key length needed is longer than the output of the HASH(...):
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

                        * This process will lose entropy if the amount of entropy in K
                        is larger than the internal state size of HASH

    3. Taking Keys Into Use
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

8. Diffie-Hellman Key Exchange

    0. Diffie-Hellman Key Exchange
        * The Diffie-Hellman (DH) key exchange provides a shared secret that cannot be
        determined by either party alone
        * The key exchange is combined with a signature with the host key to provide host
        authentication
        * This key exchange method provides Explicit Server Authentication as defined in
        Section 7.
        * The following steps are used to exchange a key, where:
            * C - Client
            * S - Server
            * p - large safe prime
            * g - generator for a subgroup of GF(p)
                * GF(p) - Galois field of p 
                * See [finite field arithmetic](https://en.wikipedia.org/wiki/Finite_field_arithmetic)
            * q - order of the subgroup
            * V_S - Identification string of S (Server)
            * V_C - Identification string of C (Client)
            * K_S - Public host key of S (Server)
            * I_C - C's (Client's) SSH_MSG_KEXINIT message
            * I_S - S's (Server's) SSH_MSG_KEXINIT message
            
                Note: Both I_C and I_S have been exchanged before this part begins

            The steps are as follows:
            
            1. C generates random number x (where 1 < x < q):
                1. computes e = g<sup>x</sup> mod p
                2. C sends e to S
            2. S generates random number y (where 0 < y < q):
                1. Computes f = g<sup>y</sup> mod p
                2. S receives e
                3. S computes:
                    * K = e<sup>y</sup> mod p
                    * H = HASH(V_C || V_S || I_C || I_S || K_S || e || f || k)
                        * Elements encoded according to their types; see below
                    * signature s on H with its private host key
                4. S sends (K_S || f || s) to C

                * The signing operation may involve a second hashing operation
            3. C verifies that K_S really is the hsot key for S (e.g., using
            certificates or a local database).
                * C is also allowed to accept hte key without verification
                    * However, doing so renders the protocol insecure against active
                    attacks
                1. C computes:
                    * K = f<sup>x</sup> mod p
                    * H = HASH(V_C || V_S || I_C || I_S || K_S || e || f || K )
                
                2. C verifies signature s on H

            * Values of 'e' or 'f' not in range [1, p-1] MUST NOT be sent or accepted by
            either side.
                * If this condition is violated, the key exchange fails

        * This is implemented with the following messages:
            * The hash algorithm for computing the exchange hash is defined by the method name, and is called HASH
            * The public key algorithm for signing is negotiated with the SSH_MSG_KEXINIT
            messages
        
            1. First, C sends:
                | Type  | Value              |
                |-------|--------------------|
                | byte  | SSH_MSG_KEXDH_INIT |
                | mpint | e                  |
            
            2. S then responds with:
                | Type   | Value                                         |
                |--------|-----------------------------------------------|
                | byte   | SSH_MSG_KEXDH_REPLY                           |
                | string | server public host key and certificates (K_S) |
                | mpint  | f                                             |
                | string | signature of H                                |
                
            3. The hash H is computed as the HASH hash of the concatenation of:
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

                * This value, H, is called the exchange hash
                    * It is used to authenticate the key exchange
                    * It SHOULD be kept secret
                
                * The signature algorithm MUST be applied over H, not the original
                data
                    * Most signature algorithms include hashing and additional padding
                    (e.g., "ssh-dss" specifies SHA-1 hashing).
                        * In this case, the data is first hashed with HASH to compute H,
                        and then H is hashed with SHA-1 as part of the signing operation
    
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
    
    * To reiterate, both of the above methods MUST be supported by implementations

9. Key Re-Exchange
    * The key re-exchange process is as follows:
        1. Send an SSH_MSG_KEXINIT when not already doing a key exchange, as described
        in Section 7.1
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

    * Key exchange does not affect the protocols that lie above the SSH transport layer

10. Service Request
    * After the key exchange, the Client requests a service
    * The service is identified by a name
    * The format of names and procedures for defining new names are defined in [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4251)
    and [SSH-NUMBERS](https://datatracker.ietf.org/doc/html/rfc4250)
    * The following names are currently reserved:
        * ssh-userauth
        * ssh-connection
    
    * A local service should use the PRIVATE USE syntax of "servicename@domain
        | Type   | Value                   |
        |--------|-------------------------|
        | byte   | SSH_MSG_SERVICE_REQUEST |
        | string | service name            |

    * If the Server rejects the service request, it SHOULD send an appropriate
    SSH_MSG_DISCONNECT message and MUST disconnect
    
    * When the service starts, it may have access to the session identifier generated
    during the key exchange

    * If the Server supports the service, and permits the Client to use it, it MUST
    respond with:
        | Type   | Value                  |
        |--------|------------------------|
        | byte   | SSH_MSG_SERVICE_ACCEPT |
        | string | service name           |
    
    * Message numbers used by services should be in the area reserved for them
        * See [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4251)
        and [SSH-NUMBERS](https://datatracker.ietf.org/doc/html/rfc4250)
    
        The transport level will continue to process its own messages
    
    * Note: After a key exchange with Implicit Server Authentication, the Client MUST
    wait for a response to its service request message before sending any further data

11. Additional Messages
    
    Either party may send any of the following messages at any time:

    1. Disconnection Message
        * Causes immediate termination of the connection
        * All implementations MUST be able to process this message, and SHOULD be able
        to transmit this message
        * The sender MUST NOT send or receive any data after this message
        * The recipient MUST NOT accept any data after receiving this message
        * Message format:
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

            * 'reason code' gives the reason in a machine-readble form
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
                * Requests for assignments of new 'reason code' values (and associated 'description' text) in the range
                of 0x00000010 to 0xFDFFFFFF MUST be done through the IETF CONSENSUS method, as described in
                [RFC 2434](https://datatracker.ietf.org/doc/html/rfc2434).
                * Disconnection Message 'reason code' values in the range of 0xFE000000 through 0xFFFFFFFF are reserved
                for PRIVATE USE
                
    2. Ignored Data Message
        * Message format:
            | Type   | Value          |
            |--------|----------------|
            | byte   | SSH_MSG_IGNORE |
            | string | data           |
        * All implementations MUST understand (and ignore) this message at any time (after receiving the
        identification string).
        * No implementation is required to send them.
        * This message can be used as an additional protection measure against advanced traffic analysis techniques
        

    3. Debug Message
        * All implementations MUST understand this message, but are allowed to ignore it.
        * This message is used to transmit information that may help debugging
        * Message format:
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

    4. Reserved Messages
        * An implementation MUST respond to all unrecognized messages with an SSH_MSG_UNIMPLEMENTED message in the order
        in which the messages were received
        * Such messages MUST be otherwise ignored
        * Message format:
            | Type   | Value                                      |
            |--------|--------------------------------------------|
            | byte   | SSH_MSG_UNIMPLEMENTED                      |
            | uint32 | packet sequence number of rejected message |
        
12. Summary of Message Numbers
    * The following is a summary of messages, along with their associated message number:
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
    * Numbers 30 through 49 are used for kex packets - different kex methods may reuse message numbers in this range


13. IANA Considerations
    * The summarized document is part of a set, including:
        * [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4251)
        * [SSH-USERAUTH](https://datatracker.ietf.org/doc/html/rfc4252)
        * [SSH-CONNECT](https://datatracker.ietf.org/doc/html/rfc4254)
        * [SSH-NUMBERS](https://datatracker.ietf.org/doc/html/rfc4250)
        * [SSH-TRANS](https://datatracker.ietf.org/doc/html/rfc4253) (the document summarized here)
    * IANA considerations for the SSH protocol as defined in the above set are detailed
    in [SSH-NUMBERS](https://datatracker.ietf.org/doc/html/rfc4250)

14. Security considerations for this protocol are provided in [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4251)

15. References
    * See the summarized document [RFC 4253](https://datatracker.ietf.org/doc/html/rfc4253) for references

