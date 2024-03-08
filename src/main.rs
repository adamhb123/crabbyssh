mod diffiehellman;
/*
SSH setup flow:
    1. Client initiates connection by contacting server
    2. Server sends its public key
    3. Client & Server negotiate parameters and open secure channel
    4. User login to server host OS


Required/recommended ciphers:
    3des-cbc (required) - Three-key 3DES in CBC mode
    aes128-cbc (recommended) - AES with a 128-bit key
*/
enum SSHDisconnectMessage {
    HostNotAllowedToConnect = 1,
    ProtocolError = 2,
    KeyExchangeFailed = 3,
    Reserved = 4,
    MacError = 5,
    CompressionError = 6,
    ServiceNotAvailable = 7,
    ProtocolVersionNotSupported = 8,
    HostKeyNotVerifiable = 9,
    ConnectionLost = 10,
    ByApplication = 11,
    TooManyConnections = 12,
    AuthCancelledByUser = 13,
    NoMoreAuthMethodsAvailable = 14,
    IllegalUserName = 15
}
struct KeyExchangePacket {
    SSH_MSG_KEXINIT: u8,
    cookie: [u8; 16],
    /*
    kex_algorithms: [&'static str], // name-list type - MUST containe at least one algorithm name, ranked most ot least preferred, 
    server_host_key_algorithms: [&'static str],
    encryption_algorithms_client_to_server: [&'static str],
    encryption_algorithms_server_to_client: [&'static str],
    mac_algorithms_client_to_server: [&'static str],
    mac_algorithms_server_to_client: [&'static str],
    compression_algorithms_client_to_server: [&'static str],
    compression_algorithms_server_to_client: [&'static str],
    languages_client_to_server: [&'static str],
    languages_server_to_client: [&'static str],
    */
    first_kex_packet_follows: bool,
}

#[derive(std::fmt::Debug)]
struct Packet<'a> {
    packet_length: u32, // Lenght of the packet in bytes, EXCLUDING the 'mac' and 'packet_length' (itself) fields
    padding_length: u8, // Length of random_padding (bytes)
    payload: &'a [u8], // Useful contents of packet. If compression has been negotiated, this field is compressed. Initially, compression MUST be 'none'
    random_padding: &'a [u8], // such that the total = packet_length + padding_length + payload + random padding is a multiple of the cipher block size or 8, whichever is larger. There MUST be at least 4 bytes of padding, padding should consist of random bytes. Max amount of padding is 255 bytes.
    mac: &'a [u8] // Message Authentication Code - if message authentication has been negoitated, this field contains the MAC bytes. Initially, MAC algorithm must be none
}
// Mac is computed after key exchange from a shared secret, packet sequence number, and the contents of the packet
// Mac algorithm: hmac-sha1 (required)
// Key exchange method:
//      diffie-hellman-group1-sha1 REQUIRED
//      diffie-hellman-group14-sha1 REQUIRED



impl<'a> Packet<'a> {
    fn new(packet_length: u32, padding_length: u8, payload: &'a [u8], random_padding: &'a [u8], mac: &'a [u8]) -> Packet<'a> {
        Packet { packet_length, padding_length, payload, random_padding, mac }
    }
}

fn main() {
    //let packet = Packet::new(32, 4, &[1,2,3,4,5], &[1,1,1,1], &[]);
    //println!("{:?}", packet);
    diffiehellman::test_random_prime();
}