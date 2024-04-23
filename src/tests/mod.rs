#[cfg(test)]
use super::*;
// ssh/
#[test]
fn ssh_test_types() {
    println!("Test");
}
// ssh/kex_groups
#[test]
fn ssh_kex_groups() {
    // DH_GROUP_1
    println!("Testing DiffieHellman w/ DH_GROUP_1:");
    // as server
    let mut dhg1_server = ssh::kex_groups::DiffieHellman::new(&ssh::kex_groups::DH_GROUP_1);
    let (server_private_key, server_public_key) = (dhg1_server.gen_private_key(true), dhg1_server.gen_public_key());
    println!("\tAs Server:");
    println!("\t\tdhg1_server.gen_private_key(is_server=true): \n\t\t\t{}", server_private_key);
    println!("\t\tdhg1_server.gen_public_key(): \n\t\t\t{}", server_public_key);
    // as client
    let mut dhg1_client = ssh::kex_groups::DiffieHellman::new(&ssh::kex_groups::DH_GROUP_1);
    let (cli_private_key, cli_public_key) =  (dhg1_client.gen_private_key(false),  dhg1_client.gen_public_key());
    println!("\n\tAs Client:");
    println!("\t\tdhg1_client.gen_private_key(is_server=false): \n\t\t\t{}", cli_private_key);
    println!("\t\tdhg1_client.gen_public_key(): \n\t\t\t{}", cli_public_key);

    // shared
    let server_shared_secret = dhg1_server.compute_shared_secret(cli_public_key);
    let client_shared_secret = dhg1_client.compute_shared_secret(server_public_key);
    let equality_checks = (
        server_shared_secret == client_shared_secret,
        dhg1_server.validate_shared_secret(&server_shared_secret),
        dhg1_client.validate_shared_secret(&client_shared_secret)
    );
    println!("\n\t Shared operations:");
    println!("\t\t dhg1_server.compute_shared_secret:\n\t\t\t{}", server_shared_secret);
    println!("\t\t dhg1_client.compute_shared_secret:\n\t\t\t{}", client_shared_secret);
    println!("\t\t server computed shared secret == client computed shared secret: {}", equality_checks.0);
    println!("\t\t dhg1_server.validate_shared_secret: {}", equality_checks.1);
    println!("\t\t dhg1_client.validate_shared_secret: {}", equality_checks.2);

}