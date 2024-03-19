## [SSH-USERAUTH](https://datatracker.ietf.org/doc/html/rfc4252) - The Secure Shell (SSH) Protocol Architecture ([RFC 4252](https://datatracker.ietf.org/doc/html/rfc4252)) Summary (by section)
This is a summary of "The Secure Shell (SSH) Authentication Protocol" ([RFC 4252](https://datatracker.ietf.org/doc/html/rfc4252))

0. Abstract
    * SSH-USERAUTH (RFC 4252) details the SSH authentication protocol framework and public key, password, and host-based client authentication methods.
        * Additional authentication methods are described in separate documents.
    * This protocol runs on top of the SSH Transport Layer Protocol [SSH-TRANS](SSH-TRANS.md), and provides a single
    authenticated tunnel for the SSH Connection Protocol [SSH-CONNECT](SSH-CONNECT.md)

1. Introduction
    * This protocol is a general-purpose user authentication protocol, intended to be run over the SSH Tranport Layer
    Protocol [SSH-TRANS](SSH-TRANS.md)
    * This protocol assumes that the underlying protocols provide integrity and confidentialty protection
    * This document should be read only after reading the SSH architecture document [SSH-ARCH](SSH-ARCH.md)
        * Terminology and notation from the architecture document are used freely without reference or further
        explanation
    * The 'service name' for this protocol is 'ssh-userauth'

    * When this protocol starts, it receives the session identifier (exchange hash H from the first key exchange) from
    the lower-level (Transport Layer) protocol
        * The session identifier uniquely identifies this session, and is suitable for signing in order to prove
        ownership of a private key
    * This protocol also needs to know whether the lower-level protocol provides confidentiality protection

2. Contributors
    * See document

3. Conventions Used in This Document
    * See document

4. The Authentication Protocol Framework
    * The Server drives the authentication by telling the client which authentication methods can be used to continue the exchange at any given time
    * The Client has the freedom to try the methods listed by the Server in ANY order
    * This gives the Server complete control over the authentication process if desired, but also gives enough
    flexibility for the Client to use the methods it supports or that are most convenient for the user, when multiple
    methods are offered by the Server

    * Authentication methods are identified by their name as defined in [SSH-ARCH](SSH-ARCH.md)
    * The "none" method is reserved, and MUST NOT be listed as supported
        * However, it MAY be sent by the Client
            * The Server MUST always reject this request, unless the Client is to be granted access without any
            authentication, in which case the Server MUST accept this request
        * The main purpose of sending this request is to get the aforementioned list of supported authentication
        methods from the Server
    
    * The Server SHOULD have a timeout for authentication, disconnecting if the authentication has not been accepted
    within the timeout period
        * The RECOMMENDED timeout period is 10 minutes
    * The implementation SHOULD limit the number of failed authentication attempts a Client may perform in a single
    session
        * The RECOMMENDED limit is 20 attempts
        * If this threshold is exceeded, the Server SHOULD disconnect
    * Additional thoughts regarding authentication timeouts and retries may be found in [ssh-1.2.30](https://datatracker.ietf.org/doc/html/rfc4252#ref-ssh-1.2.30)

5. Authentication Requests
    0. Authentication Requests
        * All authentication requests MUST use the following format
            * Only the first few fields are defined. Remaining fields depend on the authentication method
            * Format:
                | Type   | Value                                                                                           |
                |--------|-------------------------------------------------------------------------------------------------|
                | byte   | SSH_MSG_USERAUTH_REQUEST                                                                        |
                | string | user name in ISO-10646 UTF-8 encoding [RFC 3629](https://datatracker.ietf.org/doc/html/rfc3629) |
                | string | service name in US-ASCII                                                                        |
                | string | method name in US-ASCII                                                                         |
                * 'user name' and 'service name' are repeated in every new authentication attempt, and MAY change
                    * The Server implementation MUST:
                        * Carefully check them in every message
                        * Flush any accumulated authentication states if they change
                            * If the Server is unable to flush an authentication state, it MUST disconnect if the 'user name'
                            or 'service name' changes
                * 'service name' specifies the service to start after authentication
                    * Several different authenticated services may be provided
                    * If the requested service is not available, the Server MAY disconnect immediately or at any later time
                        * Sending a proper disconnect message is RECOMMENDED
                    * In any case, if the service does not exist, authentication MUST NOT be accepted
                
                * If the requested 'user name' does not exist, the server MAY:
                    * Disconnect
                    * Send a bogus list of acceptable authentication 'method name' values, but NEVER accept any
                        * This makes it possible to avoid disclosing information on which accounts exist
                    * In any case, if the 'user name' does not exist, the authentication request MUST NOT be accepted

                * The following 'method name' values are defined:
                    | Method Name | Necessity       |
                    |-------------|-----------------|
                    | "publickey" | REQUIRED        |
                    | "password"  | OPTIONAL        |
                    | "hostbased" | OPTIONAL        |
                    | "none"      | NOT RECOMMENDED |

                    * Additional 'method name' values may be defined as specified in [SSH-ARCH](SSH-ARCH.md) and [SSH-NUMBERS](SSH-NUMBERS.md)

            * While there is usually little point for clients to send requests that the server does not list as acceptable,
            sending such requests is not an error, and the server SHOULD simply reject requests that it does not recognize.

        * An authentication request MAY result in a further exchange of messages
            * All such messages depend on the authentication 'method name' used
            * The Client MAY, at any time, continue with a new SSH_MSG_USERAUTH_REQUEST message
                * In which case the Server MUST abandon the previous authentication attempt and continue with the new one
        
       
    1. Responses to Authentication Requests
        * If the server rejects the authentication request, it MUST respond with the following:
            | Type    | Value                             |
            |---------|-----------------------------------|
            | byte    | SSH_MSG_USERAUTH_FAILURE          |
            | string  | authentications that can continue |
            | boolean | partial success                   |
            * 'authentications that can continue' is a comma-separated name-list of authentication 'method name' values that may
            productively continue the authentication dialog
                * It is RECOMMENDED that servers only include those 'method name' values that are actually useful
                    * However, it is not illegal to include 'method name' values that cannot be used to authenticate the user
                * Already successfully completed authentications SHOULD NOT be included in the name-list, unless they should be
                performed again for some reason
            * 'partial success' MUST be:
                * TRUE if the authentication request to which this is a response was successful
                * FALSE if the request was not successfully processed

        * Upon accepting authentication, the server MUST respond with the following:
            | Type    | Value                             |
            |---------|-----------------------------------|
            | byte    | SSH_MSG_USERAUTH_SUCCESS          |
            
            * Note: this is not sent after each step in a multi-method authentication sequence, but only when authentication is FULLY
            complete
        
        * The Client MAY send several authentication requests without waiting for responses from previous requests
            * The Server MUST process each request completely and acknowledge any failed requests with a SSH_MSG_USERAUTH_FAILURE 
            message before processing the next request

            * A request requiring further methods to be exchanged will be aborted by a subsequent request
                * A Client MUST NOT send a subsequent request if it has not received a response from the server for a previous request

            * A SSH_MSG_USERAUTH_FAILURE message MUST NOT be sent for an aborted method.

        * SSH_MSG_USERAUTH_SUCCESS MUST be sent only once
            * When SSH_MSG_USERAUTH_SUCCESS has been sent, any further authentication requests received after that SHOULD be silently ignored.
        
        * Any non-authentication messages sent by the client after the request that resulted in SSH_MSG_USERAUTH_SUCCESS being sent MUST be
        passed to the service being run on top of this protocol.  Such messages can be identified by their message numbers (see Section 6).

    2. The "none" Authentication Request
        * A Client may request a list of authentication 'method name' values that may continue by using the "none" authentication 'method name'

        *  If no authentication is needed for the user, the server MUST return SSH_MSG_USERAUTH_SUCCESS
            * Otherwise, the server MUST return SSH_MSG_USERAUTH_FAILURE and MAY return with it a list of methods that may continue in its
            'authentications that can continue' value
        
        * This 'method name' MUST NOT be listed as supported by the Server
    
    3. Completion of User Authentication
        * Authentication is complete when the server has responded with SSH_MSG_USERAUTH_SUCCESS
            * All authentication-related messages received after this messages SHOULD be silently ignored
            * After sending SSH_MSG_USERAUTH_SUCCESS, the server starts the requested service
        
    4. Banner Message
        * The Server may send an SSH_MSG_USERAUTH_BANNER message at any time after the Authentication Protocol starts, and before
        authentication is successful
            * The SSH_MSG_USERAUTH_BANNER message is formatted as follows:
                | Type   | Value                                                                                         |
                |--------|-----------------------------------------------------------------------------------------------|
                | byte   | SSH_MSG_USERAUTH_BANNER                                                                       |
                | string | message in ISO-10646 UTF-8 encoding [RFC 3629](https://datatracker.ietf.org/doc/html/rfc3629) |
                | string | language tag                                                                                  |
            
            * 'message' may consist of multiple lines, with line-breaks indicated by \<CR>\<LF> pairs
                * The Client SHOULD display the 'message' on the screen by default
                    * However, the client software may allow the user to explicitly disable the display of banners from the server
            
                * If 'message' is displayed, control character filtering discussed in [SSH-ARCH](SSH-ARCH.md) SHOULD be used to avoid
                attacks by sending terminal control characters
             
6. Authentication Protocol Message Numbers
    * All message numbers used by this protocol are in the range 50-79 (which is part of the range reserved for protocols running on top of
    the SSH Transport Layer Protocol)

    * Message numbers >= 80 are reserved for protocols running after this protocol
        * Thus, receiving one of the before authentication completes is an error
            * The Server must respond by disconnecting, preferably with a proper disconnect message sent to ease troubleshooting
    
    * After successful authentication, such messages are passed to the higher level service

    * General authentication message codes:
        | Code                     | Value |
        |--------------------------|-------|
        | SSH_MSG_USERAUTH_REQUEST | 50    |
        | SSH_MSG_USERAUTH_FAILURE | 51    |
        | SSH_MSG_USERAUTH_SUCCESS | 52    |
        | SSH_MSG_USERAUTH_BANNER  | 53    |

        * The Client sends only SSH_MSG_USERAUTH_REQUEST messages
        * In addition to the above, message numbers in the range 60-79 exist, and are reserved for method-specific messages
            * These messages are only sent by the Server
            * Different authentication methods reuse the same message numbers

7. Public Key Authentication Method: "publickey"
    * The only REQUIRED authentication 'method name'
    * Not all users need to have public keys
    
    * This method:
        * requires the possession of a private key, which serves as authentication
        * Works by the Client sending a signature created with a private key of the user
            * The Server MUST then:
                * Check that the key is a valid authenticator for the user
                * Check that the signature is valid
            * If both of these checks hold, the authentication request MUST be accepted
                * Otherwise, it MUST be rejected
            * The Server MAY require additional authentications after successful authentication
    
    * Private keys are often stored in an encrypted form at the client host, requiring the user to supply a passphrase prior to
    generating the signature
        * Even if the passphrase is not required, the signing operation involves expensive computation
            * To avoid unnecessary processing and user interaction, the following message is provided for querying whether
            authentication using the "publickey" method would be acceptable:
                | Type    | Value                                                                                           |
                |---------|-------------------------------------------------------------------------------------------------|
                | byte    | SSH_MSG_USERAUTH_REQUEST                                                                        |
                | string  | user name in ISO-10646 UTF-8 encoding [RFC 3629](https://datatracker.ietf.org/doc/html/rfc3629) |
                | string  | service name in US-ASCII                                                                        |
                | string  | "publickey"                                                                                     |
                | boolean | FALSE                                                                                           |
                | string  | public key algorithm name                                                                       |
                | string  | public key blob                                                                                 |