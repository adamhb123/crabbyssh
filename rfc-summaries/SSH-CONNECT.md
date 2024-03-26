## [SSH-CONNECT](https://datatracker.ietf.org/doc/html/rfc4254) - The Secure Shell (SSH) Connection Protocol ([RFC 4254](https://datatracker.ietf.org/doc/html/rfc4254)) Summary (by section)
This is a summary of "The Secure Shell (SSH) Connection Protocol" ([RFC 4254](https://datatracker.ietf.org/doc/html/rfc4254))

0. Abstract

    * SSH-CONNECT (RFC 4254) details the SSH Connection Protocol, which provides:
        * Interactive login sessions
        * Remote execution of commands
        * Forwarded TCP/IP connections
        * Forwarded X11 connections

    * All of the above channels are multiplexed into a single encrypted tunnel

    * This protocol has been designed to run on top of the SSH Transport Layer ([SSH-TRANS](./SSH-TRANS.md)) and User Authentication ([SSH-USERAUTH](./SSH-USERAUTH.md))

1. Introduction
    * This protocol has been designed to run on top of the SSH Transport Layer ([SSH-TRANS](./SSH-TRANS.md)) and User Authentication ([SSH-USERAUTH](./SSH-USERAUTH.md))

    * The 'service name' for this protocol is "ssh-connection"

    * This document should be read only after reading [SSH-ARCH](./SSH-ARCH.md). Terminology and notation from SSH-ARCH is used freely within this 
        document without reference or further explanation
    
2. Contributors
    * See original document

3. Conventions Used in This Document
    * See original document

4. Global Requests
    * Several kinds of requests can affect the state of the remote end globally, independent of any channels
        * E.g., a request to start TCP/IP forwarding from a specific port
    
    * Both the Client and Server MAY send global requests at any time, and the receiver MUST respond appropriately

    * Global requests use the following format:
        | Type    | Value                         |
        |---------|-------------------------------|
        | byte    | SSH_MSG_GLOBAL_REQUEST        |
        | string  | request name in US-ASCII only |
        | boolean | want reply                    |
        | ...     | request-specific data         |

        * The value of 'request name' follows the DNS extensibility naming convention outlined in [SSH-ARCH](./SSH-ARCH.md)

        * If 'want reply' is TRUE, the recipient of the global request will respond to this method with either:
            * SSH_MSG_REQUEST_SUCCESS
                | Type    | Value                         |
                |---------|-------------------------------|
                | byte    | SSH_MSG_REQUEST_SUCCESS       |
                | ...     | response-specific data        |
                * Usually, 'response-specific data' is non-existent

            * SSH_MSG_REQUEST_FAILURE
                * Sent if the recipient does not recognize or support the request with the message:
                    | Type    | Value                         |
                    |---------|-------------------------------|
                    | byte    | SSH_MSG_REQUEST_FAILURE       |
                    * Only this failure byte is sent
            
            * Generally, these reply messages do not include request type identifiers
                * Thus, in order to make it possible for the originator of the request to identify to which request each reply refers,
                    it is REQUIRED that replies to SSH_MSG_GLOBAL_REQUESTS MUST be sent in the same order as the corresponding request messages.
                
                * For channel requests, replies that relate to the same channel MUST also be replied to in the right order
                    * However, channel requests for distinct channels MAY be replied to out-of-rder
            
5. Channel Mechanism

    0. Channel Mechanism
        * All terminal sessions, forwarded connections, etc... are channels
        * Either side may open a channel
        * Multiple channels are multiplexed into a single connection
        * Channels are identified by numbers at each end
            * The number referring to a channel may be different on each side
            * Requests to open a channel contain the sender's channel number
                * Any other channel-related messages contain the recipient's channel number for the channel
        
        * Channels are flow-controlled
            * No data may be sent to a channel until a message is received to indicate that window space is available

    1. Opening a Channel
        * When either side wishes to open a new channel, it;
            1. Allocates a local number for the channel
            2. Sends the following message to the other side:
                | Type   | Value                              |
                |--------|------------------------------------|
                | byte   | SSH_MSG_CHANNEL_OPEN               |
                | string | channel type in US-ASCII only      |
                | uint32 | sender channel                     |
                | uint32 | initial window size                |
                | uint32 | maximum packet size                |
                | ...    | channel type specific data follows |
        
                * 'channel type' is a name, as described in [SSH-ARCH](./SSH-ARCH.md) and [SSH-NUMBERS](./SSH-NUMBERS.md), with similar
                    extension mechanisms
                * 'sender channel' is a local identifier for the the channel used by the sender of this message
                * 'initial window size' specifies how many bytes of channel data can be sent to the sender of this message without adjusting
                    the window
                * 'maximum packet size' specifies the maximum size of an individual data packet that can be sent to the sender
                    * E.g., one might want to use smaller packets for interactive connections to get better interactive response on slow links
            
        * The remote side then decides whether it can open the channel, responding with either:
            1. SSH_MSG_CHANNEL_OPEN_CONFIRMATION:
                | Type   | Value                              |
                |--------|------------------------------------|
                | byte   | SSH_MSG_CHANNEL_OPEN_CONFIRMATION  |
                | uint32 | recipient channel                  |
                | uint32 | sender channel                     |
                | uint32 | initial window size                |
                | uint32 | maximum packet size                |
                | ...    | channel type specific data follows |
                * 'sender channel' is the channel number allocated by the other side
                * 'recipient channel' is the channel number given in the original open request (also applies to the below message)

            2. SSH_MSG_CHANNEL_OPEN_FAILURE:
                | Type   | Value                                                                                               |
                |--------|-----------------------------------------------------------------------------------------------------|
                | byte   | SSH_MSG_CHANNEL_OPEN_FAILURE                                                                        |
                | uint32 | recipient channel                                                                                   |
                | uint32 | reason code                                                                                         |
                | string | description in ISO-10646 UTF-8 encoding ([RFC 3629](https://datatracker.ietf.org/doc/html/rfc3629)) |
                | string | language tag ([RFC 3066](https://datatracker.ietf.org/doc/html/rfc3066))                            |

                * The Client MAY show the 'description' string to the user. If this is done, the Client software should take the precautions
                    discussed in [SSH-ARCH](./SSH-ARCH.md)

                * If the recipient of the SSH_MSG_CHANNEL_OPEN message does not support the specified 'channel type', then it responds SOLELY with
                    SSH_MSG_CHANNEL_OPEN_FAILURE
                    | Type | Value                        |
                    |------|------------------------------|
                    | byte | SSH_MSG_CHANNEL_OPEN_FAILURE |

                    * TODO: UPDATE THIS SECTION WITH CLARITY - I am unsure if this is what the document is actually saying or not

                * SSH_MSG_CHANNEL_OPEN_FAILURE 'reason code' values are defined as follows:
                    | Symbolic Name                        | Reason Code |
                    |--------------------------------------|-------------|
                    | SSH_OPEN_ADMINISTRATIVELY_PROHIBITED | 1           |
                    | SSH_OPEN_CONNECT_FAILED              | 2           |
                    | SSH_OPEN_UNKNOWN_CHANNEL_TYPE        | 3           |
                    | SSH_OPEN_RESOURCE_SHORTAGE           | 4           |

        * Requests for assignments of new SSH_MSG_CHANNEL_OPEN 'reason code' values, and associated 'description' text, in the range
            of 0x00000005 to 0xFDFFFFFF MUST be done through the IETF CONSENSUS method as described in [RFC 2434](https://datatracker.ietf.org/doc/html/rfc2434)

            * The IANA will not assign Channel Connection Failure 'reason code' values in the range of 0xFE000000 to 0xFFFFFFFF.
                Channel Connection Failure 'reason code' values in this range are left for PRIVATE use, as described in RFC 2434 (see immediately above)
            
                * While is understood that the IANA has no control over this range, it will be split into two parts, administered by the following
                    conventions:
                
                    1. 0xFE000000 to 0xFEFFFFFF
                        * To be used in conjunction with locally assigned channels
                            * E.g., If a channel is proposed with a 'channel type' of "example_session@example.com", but fails, the response
                                will contain either:
                                    1. A 'reason code' assigned by the IANA (as listed above and in the range of 0x00000001 to 0xFDFFFFFF)
                                    2. A 'reason code' locally assigned in the range of 0xFE000000 to 0xFEFFFFFF
                            
                            * If the Server DOES NOT understand the proposed 'channel type', EVEN if it is a locally defined 'channel type', the
                                'reason code' MUST be 0x00000003 (decimal 3) as described above, if the 'reason code' is sent

                            * If the Server DOES understand the proposed 'channel type', but the channel still fails to open, the server SHOULD
                            respond with a locally assigned 'reason code' value consistent with the proposed, local 'channel type'

                        * It is assumed practitioners will first attempt to use the IANA assigned 'reason code' values, and then document their
                            locally assigned 'reason code' values


                    2. 0xFF to ...
                        * No restrictions or suggestions for the range starting with 0xFF... exist
                        * No interoperability is expected for this range
                        * Essentially, this range is for experimentation

    2. Data Transfer
        * 'window size' specifies how many bytes the other party can send before it must wait for the window to be adjusted
            * Both parties use the following message to adjust the window:
                | Type   | Value                         |
                |--------|-------------------------------|
                | byte   | SSH_MSG_CHANNEL_WINDOW_ADJUST |
                | uint32 | recipient channel             |
                | uint32 | bytes to add                  |
            
                * After receiving this message, the recipient MAY send the given number of bytes more than it was previously allowed
                    to send; the window size is incremented
            * Implementations MUST correctly handle window sizes of up to 2<sup>32</sup> - 1 bytes
                * The window size MUST NOT be increased above 2<sup>32</sup> - 1 bytes

        * Data transfer is done with messages of the following type:
            | Type   | Value                |
            |--------|----------------------|
            | byte   | SSH_MSG_CHANNEL_DATA |
            | uint32 | recipient channel    |
            | string | data                 |

            * The maximum amount of data allowed is determined by whichever of the following is SMALLER:
                * Packet size for the channel
                * Current window size

            * 'window size' is decremented by the amount of data sent
            * Both parties MAY ignore all extra data sent after the allowed window is empty
        
        * Implementations are expected to have some limit on the SSH Transport Layer packet size
            * Any limit for received packets MUST be >= 32768, as per [SSH-TRANS](./SSH-TRANS.md)

            * The implementation of the SSH Connection Layer:
                1. MUST NOT advertise a maximum packet size that would result in Transport Layer packets larger than its Transport Layer is
                    willing to receive
                2. MUST NOT generate data packets larger than its Transport Layer is willing to send, EVEN IF the remote end would be willing to
                    accept very large packets
                
        * Some channels can transfer several types of data
            * E.g., STDERR data from interactive sessions
            * Such data can be passed with SSH_MSG_CHANNEL_EXTENDED_DATA messages
                | Type   | Value                         |
                |--------|-------------------------------|
                | byte   | SSH_MSG_CHANNEL_EXTENDED_DATA |
                | uint32 | recipient channel             |
                | uint32 | data_type_code                |
                | string | data                          |
            
                * Data sent with these messages consumes the same window as ordinary data

                * 'data_type_code' currently only has the following type defined:
                    | Symbolic Name            | data_type_code |
                    |--------------------------|----------------|
                    | SSH_EXTENDED_DATA_STDERR | 1              |
                
                    * Extended Channel Data Transfer 'data_type_code' values MUST be assigned sequentially
                    * Requests for assignments of new 'data_type_code' values and their associated 'data' strings MUST be done
                        through the IETF CONSENSUS method as described in [RFC 2434](https://datatracker.ietf.org/doc/html/rfc2434)
                        * The IANA will NOT assign 'data_type_code' values in the range of 0xFE000000 to 0xFFFFFFFF
                            * 'data_type_code' values in this range are left for PRIVATE USE
                        * Actual instructions to the IANA are in [SSH-NUMBERS](./SSH-NUMBERS.md)

    3. Closing a Channel
                
        * When a party will no longer send more data to a channel, it SHOULD send SSH_MSG_CHANNEL_EOF:
            | Type   | Value               |
            |--------|---------------------|
            | byte   | SSH_MSG_CHANNEL_EOF |
            | uint32 | recipient channel   |
        
            * No explicit response is sent to this message
                * However, the application may send EOF to whatever is at the other end of the channel
                    * TODO: This requires clarification - is the \<EOF> control character sent, or is this message (SSH_MSG_CHANNEL_EOF) sent???
                        I am assuming the former, but am unsure
            
            * Note that the channel remains open after this message
                * More data may be sent in the OTHER direction
            
            * This message does not consume window space
                * Thus, it can be sent EVEN IF NO window space is available
            
        * When either party wishes to terminate the channel, it sends SSH_MSG_CHANNEL_CLOSE:
            | Type   | Value                 |
            |--------|-----------------------|
            | byte   | SSH_MSG_CHANNEL_CLOSE |
            | uint32 | recipient channel     |
        
            * Upon receiving this message, a party MUST send back an SSH_MSG_CHANNEL_CLOSE, unless it has already sent this message for
                the channel

            * The channel is considered closed for a party when it has BOTH sent and received SSH_MSG_CHANNEL_CLOSE
                * Once closed, the channel number may be reused
            
            * A party MAY send this message (SSH_MSG_CHANNEL_CLOSE) without having sent or received SSH_MSG_CHANNEL_EOF
            
            * This message does not consume window space
                * Thus, it can be sent EVEN IF NO window space is available

            * It is RECOMMENDED that all data sent before this message be delivered to the actual destination, if possible
        
    4. Channel-Specific Requests
        * Many 'channel type' values have extensions specific to that particular 'channel type'
            * E.g., requesting a pty (pseudo-terminal) for an interactive session

        * All channel-specific requests use the following format: 
            | Type    | Value                                    |
            |---------|------------------------------------------|
            | byte    | SSH_MSG_CHANNEL_REQUEST                  |
            | uint32  | recipient channel                        |
            | string  | request type in US-ASCII characters only |
            | boolean | want reply                               |
            | ...     | type-specific data follows               |

            * If 'want reply' is:
                * FALSE - no response will be sent to the request
                * TRUE - the recipient responds with either:
                    1. SSH_MSG_CHANNEL_SUCCESS
                    2. SSH_MSG_CHANNEL_FAILURE
                        * E.g., if the request is not recognized or is not supported for the channel, this is sent
                    3. request-specific continuation messages
            
           * This message does not consume window space
                * Thus, it can be sent EVEN IF NO window space is available
            
            * The values of 'request type' are local to each channel type

            * The Client is allowed to send further messages without waiting for the response to the request

            * 'request type' names follow the DNS extensibility naming convention outlined in [SSH-ARCH](./SSH-ARCH.md) and 
                [SSH-NUMBERS](./SSH-NUMBERS.md)
            
            * Recipient responses to this message:
                | Type   | Value                   |
                |--------|-------------------------|
                | byte   | SSH_MSG_CHANNEL_SUCCESS |
                | uint32 | recipient channel       |

                | Type   | Value                   |
                |--------|-------------------------|
                | byte   | SSH_MSG_CHANNEL_FAILURE |
                | uint32 | recipient channel       |

                * 'recipient channel' refers to the sender's (i.e., the one who sent the message's) channel number

                * These messages do not consume window space
                    * Thus, they can be sent EVEN IF NO window space is available

6. Interactive Sessions
    0. Interactive Sessions
        * A session is a remote execution of a program
            * The program may be a shell, application, system command, or some built-in subsystem
            * The program may or may not have a tty, and may or may not involve X11 forwarding
            * Multiple sessions can be active simultaneously
    
    1. Opening a Session
        * A session is started by sending an SSH_MSG_CHANNEL_OPEN message:
            | Type   | Value                |
            |--------|----------------------|
            | byte   | SSH_MSG_CHANNEL_OPEN |
            | string | "session"            |
            | uint32 | sender channel       |
            | uint32 | initial window size  |
            | uint32 | maximum packet size  |

            * Client implementations SHOULD reject any session channel open requests to make it more difficult for a corrupt Server to
                attack the Client
        
    2. Requesting a Pseudo-Terminal
        * A pseudo-terminal can be allocated for the session by sending a SSH_MSG_CHANNEL_REQUEST message:
            | Type   | Value                                         |
            |--------|-----------------------------------------------|
            | byte   | SSH_MSG_CHANNEL_REQUEST                       |
            | uint32 | recipient channel                             |
            | string | "pty-req"                                     |
            | string | TERM environment variable value (e.g., vt100) |
            | uint32 | terminal width, characters (e.g., 80)         |
            | uint32 | terminal height, rows (e.g., 24)              |
            | uint32 | terminal width, pixels (e.g., 640)            |
            | uint32 | terminal height, pixels (e.g., 480)           |
            | string | encoded terminal modes                        |
        
            * 'encoded terminal modes' are described in Section 8

            * Dimension parameters are only informational, and:
                * Zero dimension parameters MUST be ignored
                * Character/row dimensions OVERRIDE the pixel dimensions (when nonzero)
                * Pixel dimensions refer to the drawable area of the window

            * The Client SHOULD ignore pty requests

    3. X11 Forwarding
        1. Requesting X11 Forwarding
            * X11 forwarding may be requested for a session by sending a SSH_MSG_CHANNEL_REQUEST message:
                | Type    | Value                       |
                |---------|-----------------------------|
                | byte    | SSH_MSG_CHANNEL_REQUEST     |
                | uint32  | recipient channel           |
                | string  | "x11-req"                   |
                | boolean | want reply                  |
                | boolean | single connection           |
                | string  | x11 authentication protocol |
                | string  | x11 authentication cookie   |
                | uint32  | x11 screen number           |

                * If 'single connection' is TRUE - only a single connection should be forwarded
                    *  No more connections will be forwarded after the first, or after the session channel has been closed
                
                * 'x11 authentication protocol' - the name of the X11 authentication method used
                    * E.g., "MIT-MAGIC-COOKIE-1"

                * 'x11 authentication cookie' MUST be hexadecimal encoded
                    * It is RECOMMENDED that the 'x11 authentication cookie' that is sent be a fake, random cookie, and that the cookie should
                        be checked and replaced by the real cookie when a connection request is received
                
                * X11 connection forwarding should stop when the session channel is closed
                    * However, already opened forwardings should NOT be automatically closed when the session channel is closed

            * The X protocol is documented in [SHEIFLER](https://datatracker.ietf.org/doc/html/rfc4254#ref-SCHEIFLER)

        2. X11 Channels
            * X11 channels are opened with SSH_MSG_CHANNEL_OPEN:
                | Type   | Value                                     |
                |--------|-------------------------------------------|
                | byte   | SSH_MSG_CHANNEL_OPEN                      |
                | string | "x11"                                     |
                | uint32 | sender channel                            |
                | uint32 | initial window size                       |
                | uint32 | maximum packet size                       |
                | string | originator address (e.g., "192.168.7.38") |
                | string | originator port                           |
            
                * The resulting channels are INDEPENDENT of the SESSION
                    * Closing the session channel DOES NOT close the forwarded X11 channels
                
                * The recipient should respond with SSH_MSG_CHANNEL_OPEN_CONFIRMATION or SSH_MSG_CHANNEL_OPEN_FAILURE

            * Implementations MUST reject any X11 channel open requests if they have not requested X11 forwarding

    4. Environment Variable Passing
        * Environment variables (envvars) may be passed to the shell/command to be started later
        * Uncontrolled setting of envvars in a privileged process can be a securit hazard
            * Thus, it is RECOMMENDED that implementations EITHER:
                1. Maintain a list of allowable variable names
                2. Only set environment variables after the Server process has dropped sufficient privileges
        
        * Environment variables may be requested by sending a SSH_MSG_CHANNEL_REQUEST message:
            | Type    | Value                   |
            |---------|-------------------------|
            | byte    | SSH_MSG_CHANNEL_REQUEST |
            | uint32  | recipient channel       |
            | string  | "env"                   |
            | boolean | want reply              |
            | string  | variable name           |
            | string  | variable value          |
    
    5. Starting a Shell or a Command
        * Once the session has been set up, a program is started at the remote end
            * The program can be a shell, application program, or subsystem with a host-independent name
        * Only ONE of these requests can succeed per channel:
            * This message will request that the user's default shell (typically defined in /etc/passwd in UNIX systems) be started at the other end:
                | Type    | Value                   |
                |---------|-------------------------|
                | string  | "shell"                 |
                | uint32  | recipient channel       |
                | byte    | SSH_MSG_CHANNEL_REQUEST |
                | boolean | want reply              |

            * This message will request that the server start the execution of the given command:
                | Type    | Value                   |
                |---------|-------------------------|
                | byte    | SSH_MSG_CHANNEL_REQUEST |
                | uint32  | recipient channel       |
                | string  | "exec"                  |
                | boolean | want reply              |
                | string  | command                 |
                
                * 'command' string may contain a path
                * Normal precautions MUST be taken to prevent the execution of unauthorized commands
            
            * This (last) message executes a predefined subsystem:
                | Type    | Value                   |
                |---------|-------------------------|
                | byte    | SSH_MSG_CHANNEL_REQUEST |
                | uint32  | recipient channel       |
                | string  | "subsystem"             |
                | boolean | want reply              |
                | string  | subsystem name          |

                * It is expected that these will include a general file transfer mechanism, and possibly other features
                    * Implementations MAY also allow configuring more such mechanisms
                
                * It is advisable for the subsystem protocol to have a "magic cookie" at the beginning of the protocol transaction to 
                    distinguish it from arbitrary output generated by shell initialization scripts, etc...
                    * This spurious output from the shell MAY be filtered out, either at the Server or at the Client
                
            * The Server SHOULD NOT halt the execution of the protocol stack when starting a shell or program
            
            * All input and output from these SHOULD be redirected to the channel or to the encrypted tunnel

            * It is RECOMMENDED that the reply to these messages be requested and checked
            
            * The Client SHOULD ignore these messages

            * Subsystem names follow the DNS extensibility naming convention outlined in [SSH-NUMBERS]()

    6. Session Data Transfer
        * Data transfer for a session is done using the SSH_MSG_CHANNEL_DATA and SSH_MSG_CHANNEL_EXTENDED_DATA packets and the window mechanism
        * The extended data type SSH_EXTENDED_DATA_STDERR has been defined for stderr data.
    
    7. Window Dimension Change Message
        * When the window (terminal) size changes on the Client side, it MAY send a message to the other side to inform it of the new dimensions:
           | Type    | Value                   |
           |---------|-------------------------|
           | byte    | SSH_MSG_CHANNEL_REQUEST |
           | uint32  | recipient channel       |
           | string  | "window-change"         |
           | boolean | FALSE                   |
           | uint32  | terminal width, columns |
           | uint32  | terminal height, rows   |
           | uint32  | terminal width, pixels  |
           | uint32  | terminal height, pixels |

           * A response SHOULD NOT be sent to this message
        
    8. Local Flow Control
        * On many systems, it can be determined if a pseudo-terminal is using control-S / control-Q flow control
        
        * When flow control is allowed, it is often desirable to do the flow control at the Client end to speed up responses to user requests
            * This is facilitated by the following notification message, which is used by the Server to inform the Client when it can / cannot 
                perform flow control (control-S / control-Q processing):
                | Type    | Value                   |
                |---------|-------------------------|
                | byte    | SSH_MSG_CHANNEL_REQUEST |
                | uint32  | recipient channel       |
                | string  | "xon-xoff"              |
                | boolean | FALSE                   |
                | boolean | client can do           |
                
                * 'client can do' is TRUE - the client is allowed to perform control-S / control-Q flow control

                * No response is sent to this message

                * The Client MAY ignore this message                

        * Initially, the Server is responsible for flow control
            * Here, again, Client means the side ORIGINATING the session, and Server means the OTHER side

    9. Signals
        * A signal can be delivered to the remote process/service using the following message:
            | Type    | Value                                  |
            |---------|----------------------------------------|
            | byte    | SSH_MSG_CHANNEL_REQUEST                |
            | uint32  | recipient channel                      |
            | string  | "signal"                               |
            | boolean | FALSE                                  |
            | string  | signal name (without the "SIG" prefix) |

            * 'signal name' values will be encoded as discussed in the passage describing SSH_MSG_CHANNEL_REQUEST messages using
                "exit-signal" in the next sub-section

            * Some systems may not implement signals, in which case they SHOULD ignore this message

    10. Returning Exit Status
        * The Client MAY ignore these messages
        * When the command running at the other end terminates, the following message can be sent to return the exit status of the command:
            | Type    | Value                   |
            |---------|-------------------------|
            | byte    | SSH_MSG_CHANNEL_REQUEST |
            | uint32  | recipient channel       |
            | string  | "exit-status"           |
            | boolean | FALSE                   |
            | uint32  | exit_status             |
            * Returning the status is RECOMMENDED
            * No acknowledgement is sent for this message
            * The channel MUST be closed with SSH_MSG_CHANNEL_CLOSE after this message
        
        * The remote command may also terminate violently due to a signal. Such a condition can be indicated by the following message:
            | Type    | Value                                                                                                 |
            |---------|-------------------------------------------------------------------------------------------------------|
            | byte    | SSH_MSG_CHANNEL_REQUEST                                                                               |
            | uint32  | recipient channel                                                                                     |
            | string  | "exit-signal"                                                                                         |
            | boolean | FALSE                                                                                                 |
            | string  | signal name (without the "SIG" prefix)                                                                |
            | boolean | core dumped                                                                                           |
            | string  | error message in ISO-10646 UTF-8 encoding ([RFC 3629](https://datatracker.ietf.org/doc/html/rfc3629)) |
            | string  | language tag ([RFC 3066](https://datatracker.ietf.org/doc/html/rfc3066))                              |
            * A zero 'exit_status' usually means that the command terminated successfully
            * 'signal name' is one of the following (from [POSIX](https://datatracker.ietf.org/doc/html/rfc4254#ref-POSIX)):
                | signal name |
                |-------------|
                | ABRT        |
                | ALRM        |
                | FPE         |
                | HUP         |
                | ILL         |
                | INT         |
                | KILL        |
                | PIPE        |
                | QUIT        |
                | SEGV        |
                | TERM        |
                | USR1        |
                | USR2        |
            
                * Additional 'signal name' values MAY be sent in the format "sig-name@xyz", where 'sig-name' and 'xyz' may be anything an implementer
                    wants (excluding the '@' sign, which is required)
                    * However, it is suggested that if a 'configure' script is used, any non-standard 'signal name' values it finds should be encoded as
                        "SIG@xyz.config.guess", where:
                        * "SIG" is the 'signal name' without the "SIG" prefix
                        * "xyz" is the host type, as determined by "config.guess"
            
            * 'error message' contains an additional textual explanation of the error message
                * It may contain multiple lines separated by \<CR>\<LF> (Carriage Return - Line Feed) pairs
                * The Client software MAY display this message to the user
                    * If this is done, the Client software should takt the precautions discussed in [SSH-ARCH](./SSH-ARCH.md)

7. TCP/IP Port Forwarding
    1. Requesting Port Forwarding
        * A party need not explicitly request forwardings from its own end to the other direction
            * However, if it wishes that connections to a port on the OTHER side be forwarded to the LOCAL side, it MUST EXPLICITLY request this:
                | Type    | Value                             |
                |---------|-----------------------------------|
                | byte    | SSH_MSG_GLOBAL_REQUEST            |
                | string  | "tcpip-forward"                   |
                | boolean | want reply                        |
                | string  | address to bind (e.g., "0.0.0.0") |
                | uint32  | port number to bind               |
                * 'address to bind' - IP address (or domain name) on which connections for forwarding are to be accepted
                    * Some strings used for this have special-case semantics:
                        1. "" means that connections are to be accepted on all protocol families supported by the SSH implementation
                        2. "0.0.0.0" means to listen on all IPv4 addresses
                        3. "::" means to listen on all IPv6 addresses
                        4. "localhost" means to listen on all protocol families supported by the SSH implementation on loopback addresses only
                            (See [RFC 3330](https://datatracker.ietf.org/doc/html/rfc3330) and [RFC 3513](https://datatracker.ietf.org/doc/html/rfc3513))
                        5. "127.0.0.1" indicates listening on the loopback interfaces for IPv4 
                        6. "::1" indicates listening on the loopback interfaces for IPv6

                * 'port number to bind' - port on which connections for forwarding are to be accepted
                    * If a Client BOTH:
                        1. Passes 0 as a 'port number to bind'
                        2. Has 'want reply' as TRUE

                        ...then the Server allocates the next available unprivilege port number and replies with
                        the following message:
                          | Type   | Value                             |
                          |--------|-----------------------------------|
                          | byte   | SSH_MSG_REQUEST_SUCCESS           |
                          | uint32 | port that was bound on the server |

                    * OTHERWISE, there is no response-specific data

                * Note: the Client can still filter connections based on information passed in the open request

                * Implementations should only allow forwarding privileged ports if the user has been authenticated as a priveleged user

                * Port forwarding can be CANCELLED with the following message:
                    | Type    | Value                               |
                    |---------|-------------------------------------|
                    | byte    | SSH_MSG_GLOBAL_REQUEST              |
                    | string  | "cancel-tcpip-forward"              |
                    | boolean | want reply                          |
                    | string  | address_to_bind (e.g., "127.0.0.1") |
                    | uint32  | port number to bind                 |
                    * Note that channel open requests may be received until a reply to this message is received

                * Client implementations SHOULD reject these messages, as they are normally only sent BY the Client
    
    2. TCP/IP Forwarding Channels
        * When a connection comes to a port for which remote forwarding has been requested, a channel is opened
            to forward the port to the other side:
            | Type   | Value                      |
            |--------|----------------------------|
            | byte   | SSH_MSG_CHANNEL_OPEN       |
            | string | "forwarded-tcpip"          |
            | uint32 | sender channel             |
            | uint32 | initial window size        |
            | uint32 | maximum packet size        |
            | string | address that was connected |
            | uint32 | port that was connected    |
            | string | originator IP address      |
            | uint32 | originator port            |
            * Implementations MUST reject these messages unless they have previously requested a remote
            TCP/IP port forwarding with the given port number

        * When a connection comes to a locally forwarded TCP/IP, the following packet is sent to the other side:
            | Type   | Value                 |
            |--------|-----------------------|
            | byte   | SSH_MSG_CHANNEL_OPEN  |
            | string | "direct-tcpip"        |
            | uint32 | sender channel        |
            | uint32 | initial window size   |
            | uint32 | maximum packet size   |
            | string | host to connect       |
            | uint32 | port to connect       |
            | string | originator IP address |
            | uint32 | originator port       |

            * 'host to connect' - specifies the TCP/IP host where the recipient should connect the channel
            * 'port to connect' - specifies the TCP/IP port where the recipient should connect the channel
            * 'originator IP address' - the numeric IP address of the machine from where the connection request originates
            * 'originator port' - the port on the host from where the connection originated

        * Forwarded TCP/IP channels are independent of any sessions
            * Closing a session channel does not in any way imply that forwarded connections should be closed
        
        * Client implementations SHOULD reject direct TCP/IP open requests for security reasons

8. Encoding of Terminal Modes
    * All 'encoded terminal modes' (as passed in a pty request) are encoded into a byte stream
        * It is intended that the coding be portable across different environments
        * The stream consists of opcode-argument pairs wherein the opcode is a byte value
            * Opcodes 1-159 have a single uint32 argument
            * Opcodes 160-255 are not yet defined, and cause parsing to stop (they should only be used after
                any other data)
        * The stream is terminated by opcode TTY_OP_END (0x00)
    
    * The Client SHOULD put any modes it knows about into the byte stream
        * The Server MAY ignore any modes it does not know about
            * This allows some degree of machine-independence, at-least between systems that use a POSIX-like
                tty interface
                * This protocol can support other systems as well, but the Client may need to fill reasonable
                    values for a number of parameters so the Server pty gets set to a reasonable mode (the Server
                    leaves all unspecified mode bits in their default values, and only some combinations
                    make sense)
        
    * The naming of opcode values mostly follows the POSIX terminal mode flags
        * The opcode values are as follows:
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

9. Summary of Message Numbers
    * The following is a summary of messages and their associated message number:
        | Symbolic Name                     | Value |
        |-----------------------------------|-------|
        | SSH_MSG_GLOBAL_REQUEST            | 80    |
        | SSH_MSG_REQUEST_SUCCESS           | 81    |
        | SSH_MSG_REQUEST_FAILURE           | 82    |
        | SSH_MSG_CHANNEL_OPEN              | 90    |
        | SSH_MSG_CHANNEL_OPEN_CONFIRMATION | 91    |
        | SSH_MSG_CHANNEL_OPEN_FAILURE      | 92    |
        | SSH_MSG_CHANNEL_WINDOW_ADJUST     | 93    |
        | SSH_MSG_CHANNEL_DATA              | 94    |
        | SSH_MSG_CHANNEL_EXTENDED_DATA     | 95    |
        | SSH_MSG_CHANNEL_EOF               | 96    |
        | SSH_MSG_CHANNEL_CLOSE             | 97    |
        | SSH_MSG_CHANNEL_REQUEST           | 98    |
        | SSH_MSG_CHANNEL_SUCCESS           | 99    |
        | SSH_MSG_CHANNEL_FAILURE           | 100   |

10. IANA Considerations
    * See original document

11. Security Considerations
    * This protocol is assumed to run on top of a secure, authenticated transport.  User authentication and protection against
    network-level attacks are assumed to be provided by the underlying protocols

    * Full security considerations for this protocol are provided in [SSH-ARCH](./SSH-ARCH.md)

    * Specific to THIS document, it is RECOMMENDED that implementations disable all potentially dangerous features IF the host key has changed
        without notice or explanation
        * E.g., agent forwarding, X11 forwarding, and TCP/IP forwarding

12. References
    * See original document