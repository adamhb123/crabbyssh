## BAREBONES - Minimal Synthesis of the SSH RFC Collection
This document aims to synthesize the information from each of the SSH RFC documents into one unified piece in order to ease the development of CrabbySSH.

It includes the bare minimum information required to implement the SSH protocol.

Note that CrabbySSH is not (currently) concerned with working with any
non-2.0 SSH implementations, and so those considerations are excluded

1. SSH-ARCH
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

2. SSH-TRANS
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

    
