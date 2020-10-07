use crate::json::{BALANCE_ETHER, BALANCE_COMPOUND_ETHER};
use crate::models::backend::balances::Balance as BalanceDto;
use crate::models::service::balances::Balance;
use crate::providers::info::{TokenInfo, TokenType};

#[test]
fn ether_balance() {
    let balance_dto = serde_json::from_str::<BalanceDto>(BALANCE_ETHER).unwrap();

    let expected = Balance {
        token_info: TokenInfo {
            token_type: TokenType::Ether,
            address: "0x0000000000000000000000000000000000000000".to_string(),
            decimals: 0,
            symbol: "ETH".to_string(),
            name: "Ether".to_string(),
            logo_uri: None,
        },
        amount: "7457594371050000001".to_string(),
        fiat_amount: "2523.7991".to_string(),
    };

    let usd_to_fiat = 1.0;
    let actual = balance_dto.to_balance(usd_to_fiat);

    assert_eq!(actual, expected);
}

#[test]
fn erc20_token_balance_usd_balance() {
    let balance_dto = serde_json::from_str::<BalanceDto>(BALANCE_COMPOUND_ETHER).unwrap();

    let expected = Balance {
        token_info: TokenInfo {
            token_type: TokenType::Erc20,
            address: "0xd6801a1DfFCd0a410336Ef88DeF4320D6DF1883e".to_string(),
            decimals: 8,
            symbol: "cETH".to_string(),
            name: "Compound Ether 📈".to_string(),
            logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xd6801a1DfFCd0a410336Ef88DeF4320D6DF1883e.png".to_string()),
        },
        amount: "5002".to_string(),
        fiat_amount: "0.0014".to_string(),
    };

    let usd_to_fiat = 1.0;
    let actual = balance_dto.to_balance(usd_to_fiat);

    assert_eq!(actual, expected);
}

#[test]
fn erc20_token_balance_fiat_is_twice_usd() {
    let balance_dto = serde_json::from_str::<BalanceDto>(BALANCE_COMPOUND_ETHER).unwrap();

    let expected = Balance {
        token_info: TokenInfo {
            token_type: TokenType::Erc20,
            address: "0xd6801a1DfFCd0a410336Ef88DeF4320D6DF1883e".to_string(),
            decimals: 8,
            symbol: "cETH".to_string(),
            name: "Compound Ether 📈".to_string(),
            logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xd6801a1DfFCd0a410336Ef88DeF4320D6DF1883e.png".to_string()),
        },
        amount: "5002".to_string(),
        fiat_amount: "0.0028".to_string(),
    };

    let usd_to_fiat = 2.0;
    let actual = balance_dto.to_balance(usd_to_fiat);

    assert_eq!(actual, expected);
}