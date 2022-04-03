/*
 Copyright (c) 2022 ParallelChain Lab
 This program is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.
 This program is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.
 You should have received a copy of the GNU General Public License
 along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use smart_contract::{
    contract_init,
    sdk_method_bindgen,
    Transaction
};

use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};

// REPLACE ARGUMENT STRUCT WITH YOUR OWN IF ANY
#[sdk_method_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct MatchaShop {
    pub client: String,
    pub temp: u32,
    pub sugar: bool,
}


impl MatchaShop {
    
    fn new(tx: &Transaction<MatchaShop>, client: &String, temp: u32, sugar: bool) -> Self {
        let matcha_shop = MatchaShop {
            client: client.to_owned(),
            temp,
            sugar,
        };

        tx.set_matcha_shop(matcha_shop.client.as_bytes(), &matcha_shop);

        matcha_shop

    }

    fn ready(&self, tx: &Transaction<MatchaShop>) {

        match tx.get_matcha_shop(self.client.as_bytes()) {
            Some(matcha) => {
                let topic: String = "Shop Message".to_string();
                let value =
                    format!(
                        "Hello {} your matcha is ready at {}ºC with{} sugar",
                        self.client,
                        matcha.temp,
                        if matcha.sugar { "" } else { "out" },
                    );

                    
                    tx.emit_event(topic.as_bytes(), value.as_bytes());
                
                
                
            },
            None => {
                let topic: String = "Sorry".to_string();
                let value: String = format!("Matcha Shop is closed, come back tomorrow");
                tx.emit_event(topic.as_bytes(), value.as_bytes());
            }       
        };

    }
}


// The `contract_init` macro is required to convert the smart contract code
// from idiomatic rust to a contract that is readable and executable in
// ParallelChain Mainnet Fullnode.
#[contract_init]
pub fn contract(tx: Transaction<MatchaShop>) -> Result<String> {
    let matcha_specification = &tx.arguments;
    let new_matcha = MatchaShop::new(
        &tx, 
        &matcha_specification.client, 
        matcha_specification.temp, 
        matcha_specification.sugar
    );

    Ok(format!("Welcome to the shop {}", &new_matcha.client))

}