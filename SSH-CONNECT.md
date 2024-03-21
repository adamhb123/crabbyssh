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
                
