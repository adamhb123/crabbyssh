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

        0. Untitled
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
## Resources
[RFC 4253](https://datatracker.ietf.org/doc/html/rfc4253)