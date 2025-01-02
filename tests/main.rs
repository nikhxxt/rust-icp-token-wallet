#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_balance() {
        let caller = Principal::anonymous();
        initialize_balance(1000);

        let balance = get_balance();
        assert_eq!(balance, 1000);
    }

    #[test]
    fn test_send_tokens() {
        let caller = Principal::anonymous();
        initialize_balance(1000);

        let recipient = Principal::anonymous();
        let response = send_tokens(recipient, 500);
        assert_eq!(response, format!("Successfully sent {} tokens to {}", 500, recipient));

        let sender_balance = get_balance();
        assert_eq!(sender_balance, 500);
    }

    #[test]
    fn test_insufficient_balance() {
        let caller = Principal::anonymous();
        initialize_balance(500);

        let recipient = Principal::anonymous();
        let response = send_tokens(recipient, 1000);
        assert_eq!(response, "Insufficient balance.");
    }
}

