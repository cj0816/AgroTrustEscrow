#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, token};

    fn setup_test_env<'a>() -> (Env, AgroTrustContractClient<'a>, Address, Address, Address, Address, token::AdminClient<'a>) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, AgroTrustContract);
        let client = AgroTrustContractClient::new(&env, &contract_id);

        let buyer = Address::generate(&env);
        let farmer = Address::generate(&env);
        let verifier = Address::generate(&env);

        let token_admin = Address::generate(&env);
        let token_contract = env.register_stellar_asset_contract(token_admin.clone());
        let token_admin_client = token::AdminClient::new(&env, &token_contract);
        let token_client = token::Client::new(&env, &token_contract);

        // Mint tokens to buyer
        token_admin_client.mint(&buyer, &1_000_000);

        (env, client, buyer, farmer, verifier, token_contract, token_admin_client)
    }

    #[test]
    fn test_happy_path_escrow_execution() {
        let (env, client, buyer, farmer, verifier, token_addr, _) = setup_test_env();
        let token_client = token::Client::new(&env, &token_addr);

        client.initialize(&buyer, &farmer, &verifier, &token_addr, &500_000);
        client.fund_escrow();
        client.release_payment();

        assert_eq!(client.get_status(), EscrowStatus::Completed);
        assert_eq!(token_client.balance(&farmer), 500_000);
        assert_eq!(token_client.balance(&buyer), 500_000);
    }

    #[test]
    #[should_panic(expected = "Escrow not funded")]
    fn test_unfunded_release_failure() {
        let (env, client, buyer, farmer, verifier, token_addr, _) = setup_test_env();

        client.initialize(&buyer, &farmer, &verifier, &token_addr, &500_000);
        // Attempt releasing payment without funding
        client.release_payment();
    }

    #[test]
    fn test_state_verification_after_funding() {
        let (env, client, buyer, farmer, verifier, token_addr, _) = setup_test_env();

        client.initialize(&buyer, &farmer, &verifier, &token_addr, &500_000);
        assert_eq!(client.get_status(), EscrowStatus::Created);

        client.fund_escrow();
        assert_eq!(client.get_status(), EscrowStatus::Funded);
    }

    #[test]
    #[should_panic(expected = "Already initialized")]
    fn test_duplicate_initialization_fails() {
        let (env, client, buyer, farmer, verifier, token_addr, _) = setup_test_env();

        client.initialize(&buyer, &farmer, &verifier, &token_addr, &500_000);
        client.initialize(&buyer, &farmer, &verifier, &token_addr, &500_000);
    }

    #[test]
    #[should_panic]
    fn test_unauthorized_verifier_fails() {
        let env = Env::default();
        // Disabling auth mock to test explicit authorization enforcement
        let contract_id = env.register_contract(None, AgroTrustContract);
        let client = AgroTrustContractClient::new(&env, &contract_id);

        let buyer = Address::generate(&env);
        let farmer = Address::generate(&env);
        let verifier = Address::generate(&env);
        let token_addr = Address::generate(&env);

        client.initialize(&buyer, &farmer, &verifier, &token_addr, &100);
        // Calling without mock auth context triggers explicit failure
        client.release_payment();
    }
}