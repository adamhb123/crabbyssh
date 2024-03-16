## [SSH-ARCH](https://datatracker.ietf.org/doc/html/rfc4251) - The Secure Shell (SSH) Protocol Architecture ([RFC 4251](https://datatracker.ietf.org/doc/html/rfc4253)) Summary (by section)
This is a summary of "The Secure Shell (SSH) Transport Layer Protocol" ([RFC 4251](https://datatracker.ietf.org/doc/html/rfc4251))

SSH-ARCH (RFC 4253) details the architecture of the SSH protocol, as well as the 
notation and terminology used in SSH protocol documents

1. Introduction

    Secure Shell (SSH) consists of three major components:
    
    1. The Transport Layer Protocol (SSH-TRANS)
        * Provides server authentication, confidentiality, integrity, and 
        (optionally) compression
        * Is typically run over TCP/IP, but may be used on top of any other
        reliable data stream
    
    2. The User Authentication Protocol (SSH-USERAUTH)
        * Authenticates the Client-side user to the Server
        * Runs over the Transport Layer Protocol (SSH-TRANS)
    
    3. The Connection Protocol (SSH-CONNECT)
        * Multiplexes the encrypted tunnel into several logical chnanels
        * Runs over the User Authentication Protocol (SSH-USERAUTH)
    
    The Client sends a service request once a secure Transport Layer Connection has
    been established.
        
    A second service request is sent after authentication is complete.

    The Connection Protocol provides channels that can be used for a variety
    of purposes. Standard methods are provided for setting up secure interactive shell sessions and for forwarding (AKA, "tunneling") arbitrary TCP/IP ports and
    X11 connections

2. Contributors
    * See document

3. Conventions
    * See document

4. Architecture

    1. Host Keys
        * Each Server host SHOULD have a host key.
        * Hosts MAY have multiple host keys using multiple different algorithms
        * Multiple hosts MAY share the same host key
        * If a host has keys at all, it MUST have at least one key that uses each
        REQUIRED public key algorithm

        The Server host key is used during key exchange to verify that the client is
        really "talking" to the correct server. Thus, the client MUST have a priori
        knowledge ofthe Server's public host key.

        Two different trust models can be used:
        1. Local Database
            * The Client has a local database
                * This local database associates each host name (as typed by the user)
                with the corresponding public host key
            * Pros:
                * This method requires no centrally administered infrastructure, and no third-party coordination
            * Cons:
                * Maintaining the database of name-to-key associations may be burdensome to maintain.
        2. Certification Authority (CA) Verification
            * Host name-to-key association is certified by a trusted CA
                * Client knows only the CA root key, and can verify the validity of all
                host keys certified by accepted CAs
            * Pros: 
                * No local database needs to be maintained
                * Client is (ideally) only required to securely store a single CA key.
            * Cons:
                * Requires centrally administered infrastructure
                * Requires significant trust in the central infrastructure
        
        This protocol provides the option that the server name - host key association
        be unchecked when connecting to the host for the first time. This allowance:
        * Allows communication without prior communication of host keys or
        certification
        * Maintains protection against passive listening, but exposes the connection 
        to active man-in-the-middle attacks 
        * SHOULD NOT be normally allowed by default, but is acceptable for first-time
        communication

        Implementations SHOULD try to check host keys. An example strategy is:
        1. Accept a host key without checking ONLY for the first-time a host is
        connected
        2. Save the key in the Client's local database
        3. Compare against the saved key on all future connections to that host

        Implementations MAY provide additional host key verification methods, e.g.,
        a hexadecimal fingerprint derived from the SHA-1 hash (FIPS-180-2) of the
        public key.

        Implementations SHOULD provide an option not to accept unverifiable host keys.

    2. Extensibility
        * See document
    
    3. Policy Issues
        
        This protocol allows full negotation of encryption, integrity, key 
        exchange, compression, and public key algorithms and formats.
        
        The following policy issues SHOULD be addressed in the configuration
        mechanisms of each implementation:
        1. Encryption, integrity, and compression algorithms, separately for
        each direction
            * The policy MUST specify which is the preferred algorithm (e.g., the 
            first algorithm listed in each category)
        
        2. Public key algorithms and exchange method to be used for host
        authentication
            * The existence of trusted host keys for different public key
            algorithms also affects this choice
        
        3. The Server's required authentication methods for each user
            * The Server's policy MAY require multiple authentication for some
            or all users
            * Required algorithms MAY depend on the location from where the
            user is trying to gain access
        
        4. Operations that the user is allowed to perform using the connection
        protocol
            * The policy SHOULD NOT allow the server to start sessions or run
            commands on the client machine
            * The policy MUST NOT allow connections to the authentication agent
            unless such connections has been requested
            * Other issues generally relate to local policy - many of which may
            involve traversing or bypassing firewalls, and are interrelated with
            the local security policy
        
    4. Security Properties
        
        SSH attempts to maintain a balance between ease-of-deployment (and use),
        and security. Thus:
        * All encryption, integrity, and public key algorithms used are well-known,
        well-established algorithms
        * All algorithms use cryptographically sound key sizes that are believed to
        provide protection against even the strongest cryptanalytic attacks
        * All algorithms are negotiated. If it so happens to be the case that an
        algorithm is broken, one can easily switch to some other algorithm without
        modifying the base protocol
        * Some concessions were made for ease-of-deployment
            * Most obviously in the case of server host key verification (verifying that the server host key really belongs to the desired host).
                * The protocol allows the verification to be ignored, but this is
                NOT RECOMMENDED. This is allowed as it improves usability in the
                short-term - at least until widespread Internet public key
                infrastructures emerge.
    
    5. Localization and Character Set Support
        * Generally, SSH does not directly pass text that would be displayed to
        the user.
            * However, in cases where such data may be passed:
                * The character set for the data MUST be explicitly specified.
                * ISO-10646 UTF-8 ([RFC 3629](https://datatracker.ietf.org/doc/html/rfc3629)) encoding is generally used
                * When applicable, a field is also provided for a language tag
                ([RFC3066](https://datatracker.ietf.org/doc/html/rfc3066))
            * Note that there are no provisions for directly specifying the 
            character set or encoding for the interactive session.
                * Thus, the character set for the interactive (i.e. terminal)
                session is considered out-of-scope as a Client local issue
        * Internal names used to identify algorithms or protocols are normally
        never displayed to users, and MUST be in US-ASCII
        * Client and Server user names are constrained by what the Server is
        prepared to accept.
            * They may, on occasion, be displayed in logs, reports, etc..., 
            and MUST be encoded using ISO-10646 UTF-8
                * If another encoding is required, the Server must map user
                names to accepted user names
                    * Straight bit-wise binary comparison is RECOMMENDED
        * Textual messages are rarely transmitted due to localization reasons.
            * Textual messages that are transmitted typically relate to errors,
            debugging information, or some externally configured data.
                * Other types of textual messages SHOULD be configurable
            * For data that is normally displayed, there SHOULD be a way to
            fetch a localized message instead of the transmitted message using
            a numerical code.
    
5. Data Type Representations Used in the SSH Protocols

    This section describes the data types used throughout the SSH protocol
    documents.

    Data Types:
    * byte
        * Represents an arbitrary 8-bit value (octet)
        * Fixed length data may be written as byte[n], where n is the number of
        bytes in the array

    * boolean
        * Stored as a single byte
        * ```0 == FALSE``` ; ```1 == TRUE```
        * All non-zero values MUST be interpreted as TRUE (truthy interpretation)
            * However, applications MUST NOT store values other than 0 (FALSE) or 1 (TRUE)

    * uint32
        * 32-bit unsigned integer
        * Stored as four bytes in order of decreasing significance (i.e., network
        byte order; AKA big-endian)
            * E.g., the value ```699921578 (0x29b7f4aa)``` is stored as 
                ```29 b7 f4 aa```

    * uint64
        * 64-bit unsigned integer
        * Stored as eight bytes in order of decreasing significance (i.e., network
        byte order; AKA big-endian)

    * string
        * Arbitrary length binary string
        * Allowed to contain arbitrary binary data, including null characters and 8-bit characters
        * Stored as a uint32 containing:
            1. Length of string (number of bytes that follow)
            2. Zero (= empty string) or more bytes that are the value of the string
        * Terminating null characters are NOT used
            * i.e., strings are NOT null-terminated
        * US-ASCII is used for internal names
        * ISO-10646 UTF-8 is used for text that may be displayed to the user
        * The terminating null character, again, SHOULD NOT be stored in the string
        * E.g.:
            * Note: below, spaces are added to separate bytes for visual clarity.
            They are not included in the actual data (unless a space is intended,
            which is represented as 0x20).
            * the string "testing now" is represented as
            ```00 00 00 0B t e s t i n g 20 n o w``` where:
                * The first 3 bytes are padding
                * The 4th byte is the length (0x0B = 11), in bytes of the string,
                whose values immediately follow
                * Bytes 5-15 are each string characters (0x20 = 32 = the space character)
            * In raw hex:
                ```00 00 00 0B 74 65 73 74 69 6E 67 20 6E 6F 77``` (which is 
                actually ```0000000B74657374696E67206E6F77```, when displayed
                without visual aid)
            * In decimal:
                ```3545049591888788221162581879```
            * In binary:
                ```10110111010001100101011100110111010001101001011011100110011100100000011011100110111101110111```
    
    * mpint
        <b>TODO:"If the most significant bit would be set for
      a positive number, the number MUST be preceded by a zero byte." - clarify what this means</b> 
        * Multiple precision integer in two's complement format
            * As per two's complement:
                * Most significant bit = 0 indicates a positive number
                * Most significant bit = 1 indicates a negative number
        * Stored as a string (see above)
        * 8 bits per byte
        * MSB first (i.e., network-byte order; AKA Big-Endian)
        * The value zero MUST be stored as a string with zero bytes of data
        * Unnecessary leading bytes with the value 0 or 255 MUST NOT be included
        * As per convention, a number used in modular computations in Z_n SHOULD
        be represented in the range 0 <= x < n

        * Examples:
            | Value (hex)     | Representation (hex)                |
            |-----------------|-------------------------------------|
            | 0               | 00 00 00 00                         |
            | 9a378f9b2e332a7 | 00 00 00 08 09 a3 78 f9 b2 e3 32 a7 |
            | 80              | 00 00 00 02 00 80                   |
            | -1234           | 00 00 00 02 ed cc                   |
            | -deadbeef       | 00 00 05 ff 21 52 41 11             |

            <b>TODO: Further explanation of the above examples</b>

    * name-list
    	* String containing a comma-separated list of names
    	* Represented as a uint32 containing its length (number of bytes that follow)
    	* A name MUST have non-zero length
    	* All elements contained MUST be in US-ASCII
    	* Order of elements (names) ma or may not be significant
    	* Terminating null characters MUST NOT be used
    	* 
   		* Examples:
            | Value (hex)     | Representation (hex)                |
            |-----------------|-------------------------------------|
            | (), the empty name-list       | 00 00 00 00   |
            | ("zlib")        | 00 00 00 04 7a 6c 69 62  |
            | ("zlib,none")        |00 00 00 09 7a 6c 69 62 2c 6e 6f 6e 65|
            
6. Algorithm and Method Naming
	* Hash, encryption, integrity, and key exchange algorithms or methods are referred to by name
	* Some standard algorithms and methods MUST be supported by all implementations
	* All algorithm and method identifiers MUST be printable US-ASCII, non-empty strings no longer than 64 characters
	* Names MUST be case-sensitive
	
    Two formats exist for algorithm and method names:
    	1. Names that do not contain an "@" sign are reserved to be assigned by IETF CONSENSUS
    		* E.g., "3des-cbc"; "sha-1"; "hmac-sha1"; "zlib"
    		* Names of this format are only valid if they are first registered with the IANA
    		* Registered names MUST NOT contain an "@" sign, comma (","), whitespace, control characters (ASCII <= 32), or \<DEL> (ASCII 127)
    		* Names are case-sensitive, and MUST NOT be longer than 64 characters
    	2. Additional algorithms or methods may be defined in the format name@domainname
    		* E.g., "ourcipher-cbc@example.com"
    		* These names MUST be printable US-ASCII strings, and MUST NOT contain a comma (","), whitespace, control characters (ASCII <= 32), or \<DEL> (ASCII 127)
			* These names MUST only have a single "@" sign
			* The part following "@" MUST be a valid, fully-qualified domain name, controlled by the person or organization defining the name
			* Names are case-sensitive, and MUST NOT be longer than 64 characters
	
7. Message Numbers
	* SSH packets have message numbers in the range 1-255. These numbers have been allocated as follows:
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

8. IANA Considerations
	* See document

9. Security Considerations

	0. Untitled

      Transport Layer Protocol:

      * Provides a confidential channel over an insecure network
      * Performs:
        * Server host authentication
        * Key exchange
        * Encryption
        * Integrity protection
      * Derives a unique session-id, which may be used by higher-level protocols

      User Authentication Protocol:

      * Provides a suite of mechanisms used to authenticate the client user to the server
      * Mechanisms specified in the Authentication Protocol use the session-id provided by the Transport Layer Protocol and/or depend on the security and integrity guarantees of the Transport Layer Protocol

      Connection Protocol:

      * Specifies:
          * A mechanism to multiplex multiple streams (channels) of data over the confidential and authenticated transport
          * Channels for accessing an interactive shell
          * Channels for proxy-forwarding various external protocols over the secure transport (including arbitrary TCP/IP protocols)
          * Channels for accessing secure subsystems on the server host
    
	1. Pseudo-Random Number Generation
		* This protocol binds each session key to the session by including random, session-specific data in the hash used to produce session keys
		* Special care should be taken to ensure all random numbers are of good quality
		* If the random data here (e.g., Diffie-Hellman parameters) are pseudo-random, then the pseudo-random number generator should be cryptographically secure (i.e., its next output not easily guessed even when knowing all previous outputs)
			* Proper entropy must be added to the pseudo-random number generator as well
		* [RFC 4086](https://datatracker.ietf.org/doc/html/rfc4086) offers suggestions for sources of random numbers and entropy
		* In the case that the amount of entropy available to a given Client or Server is less than required, one must either resort to pseudo-random number generation regardless of insufficient entropy or refuse to run the protocol. The latter is preferable.
	
	2. Control Character Filtering
		* When displaying text to a user, such as error or debug messages, the Client software SHOULD replace any control characters (excluding tab, \<CR>, and newline) with safe sequences to avoid attacks by sending terminal control characters
	
	3. Transport
		1. Confidentiality
			* The Rogaway CBC mode attack may be mitigated through the insertion of packets containing SSH_MSG_IGNORE
			* See document for details
		2. Data Integrity
			* This protocol allows the Data Integrity mechanism to be disabled, but implementers SHOULD be wary of exposing this feature for any purpose other than debugging
				* Users and administrators SHOULD be explicitly warned anytime that the "none" MAC is enabled
			* So long as the "none" MAC is not used, this protocol provides data integrity
			* Since MACs use a 32-bit sequence number, they might start to leak information after 2<sup>32</sup> packets have been set
				* Following the rekeying recommendations should prevent this attack
					* The Transport Layer Protocol (SSH-TRANS) recommends rekeying after 1 GB of data, and the smalleseet possible packet is 16 bytes. Therefore, rekeying SHOULD happen after 2<sup>28</sup> at the very most

		3. Replay
			* The use of a MAC other than "none" provides integrity and authentication
			* Additionally, the Transport Layer Protocol provides a unique session identifier that can be used by higher-level protocols to bind data to a given session and prevent replay of data from prior sessions
				* For example, the User Authentication Protocol (SSH-USERAUTH) uses this to prevent replay of signatures from previous sessions
				* Since public key authentication exchanges are cryptographically bound to the session (i.e., to the initial key exchange), they cannot be successfully replayed in other sessions
				* Note that the session-id can be made ppublic without harming the security of the protocol
			* Two sessions with the same session-id (hash of key exchanges) can have their packets used to replay against the other. The chance of this is minimal with modern cryptographic methods
			* See document for discussion of replay detection using monotonically increasing sequence numbers as input to the MAC or HMAC
		
        4. Man-in-the-middle (MITM)
        	* This protocol makes no assumptions or provisions for an infrastructure or means for distributing the public keys of hosts
        	* If the protocol is used without first verifying the Server host key - Server host name association (such as when connecting to a Server for the first time), then MITM attacks are possible
        	* "In summary, the use of this protocol without a reliable association of the binding between a host and its host keys is inherently insecure and is NOT RECOMMENDED.  However, it may be necessary in non-security-critical environments, and will still provide protection against passive attacks."
        	* See document for further details on MITM attacks
					



