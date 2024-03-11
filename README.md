# CrabbySSH
Rust implementation of SSH
## Method
Current restrictions:
* Password-based user authentication only
* 
1. Key exchange with Diffie-Hellman
    * Agree on modulus and base number
        * modulus is prime and >= 2048 bits, in range  [ 2<sup>(n-1)</sup>, 2<sup>n</sup> ) | n >= 2048 as per [this](https://crypto.stackexchange.com/questions/19263/generation-of-n-bit-prime-numbers-what-is-the-actual-range)
    * Client & Server separately choose a number, and calculate a value
    * Client & Server exchange their respective calculated values
    * Client & Server calculate using the result received from the other 
    * Client & Server calculate the shared secret key (which should be the same value)

2.  User authentication
    * For now, password-based authentication will be used. Asymmetric key authentication may be
    implemented in the future

3. 

## [RFC 4253](https://datatracker.ietf.org/doc/html/rfc4253) Summary (by section)
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
    
    0. Untitled
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
                * If displayed, control character filtering (see [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4253#ref-SSH-ARCH)) SHOULD be used. This allows TCP-wrappers to display an error message prior to disconnecting
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
    
    0. Untitled
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

    0. Untitled
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
            | none | REQUIRED  | no compression                                                                                                                                       |
            | zlib | OPTIONAL  | ZLIB (LZ77) compression; See [RFC 1950](https://datatracker.ietf.org/doc/html/rfc1950) and [RFC 1951](https://datatracker.ietf.org/doc/html/rfc1951) |
    
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
            [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4253#ref-SSH-ARCH) and [SSH-NUMBERS](https://datatracker.ietf.org/doc/html/rfc4253#ref-SSH-NUMBERS)

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
            * Additional methods may be defined as specified in [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4253#ref-SSH-ARCH) and in [SSH-NUMBERS](https://datatracker.ietf.org/doc/html/rfc4253#ref-SSH-NUMBERS)

    5. Key Exchange Methods
        * Specifies how one-time session keys are generated for encryption and 
        authentication, and how the server authentication is done.
        * Two REQUIRED key exchange methods are defined:
            | Name                        | Necessity | Description   |
            |-----------------------------|-----------|---------------|
            | diffie-hellman-group1-sha1  | REQUIRED  | See Section 8 |
            | diffie-hellman-group14-sha1 | REQUIRED  | See Section 8 |
            * Additional methods may be defined as specified in [SSH-NUMBERS](https://datatracker.ietf.org/doc/html/rfc4253#ref-SSH-NUMBERS)
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
            * Additional key types may be defined as specified in [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4253#ref-SSH-ARCH) and [SSH-NUMBERS](https://datatracker.ietf.org/doc/html/rfc4253#ref-SSH-NUMBERS)
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

    0. Untitled
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
            | Type      | Description                             |
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
        names (see Algorithm Naming in [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4253#ref-SSH-ARCH) and additional information in [SSH-NUMBERS](https://datatracker.ietf.org/doc/html/rfc4253#ref-SSH-NUMBERS)). Each supported
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
                * Note that "none" must be explicitly listed if it is to be available.
                * See Section 6.3 for defined algorithm names.

            * mac_algorithms
        
                

## Resources
[RFC 4253](https://datatracker.ietf.org/doc/html/rfc4253)