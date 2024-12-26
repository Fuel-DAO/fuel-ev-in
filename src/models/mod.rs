use dotenv_codegen::dotenv;
use leptos::logging;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTransactionRequest {
    pub name: String,
    pub email_address: String,
    pub pan: String,
    pub age: u8,
    pub car_id: u64,
    pub aadhar: u64,
    pub country_code: u16,
    pub mobile_number: String,
    pub principal_jwk: String,
    pub start_time: u64,
    pub end_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReserveCarAPIRequest {
    pub booking_id: u64, pub payment: RazorpayPayment,
}

#[derive( Deserialize, Serialize, Debug, Clone)]
pub struct RazorpayPayment {
  pub payment_link_id: Option<String>,
  pub payment_id: String,
  pub ref_id: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct Data {
    id: u64
}

#[derive(Serialize, Deserialize, Debug)]
struct PaymentLink {
    payment_link: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Error {
    message: String
}

#[derive(Serialize, Deserialize, Debug)]
 struct TransactionResponse<T> {
    status_code: u16,
    data: T
}



impl CreateTransactionRequest {
    pub async fn create_payment_link(&self) -> Result<String, Box<dyn std::error::Error> > {
        let is_dev = dotenv!("BACKEND") == "LOCAL";

        let url = if is_dev {crate::consts::local::OFFCHAIN_URL } else {crate::consts::remote::OFFCHAIN_URL};

        let client = Client::new();
        let url = format!("{url}/api/payment");
        let res = client
        .post(url) // Replace with your API URL
        .json(&self)
        .send()
        .await?;

    if res.status().is_success() {
        let response_body: TransactionResponse<PaymentLink> = res.json::<TransactionResponse<PaymentLink>>().await?;
        Ok(response_body.data.payment_link)
    } else {
        let error_body: String = res.json::<TransactionResponse<Error>>().await?.data.message;
        println!("Failed: {:?}", error_body);
        Err(format!("Request Failed: {:?}", error_body).into())
    }
    }
}


impl ReserveCarAPIRequest {
    pub async fn reserve_car(&self) -> Result<u64, Box<dyn std::error::Error> > {
        let is_dev = dotenv!("BACKEND") == "LOCAL";

        let url = if is_dev {crate::consts::local::OFFCHAIN_URL } else {crate::consts::remote::OFFCHAIN_URL};

        let client = Client::new();
        let url = format!("{url}/api/transactions");
        let res = client
        .post(url) // Replace with your API URL
        .json(&self)
        .send()
        .await?;

    if res.status().is_success() {
        logging::log!("Car reserved");
        let response_body: TransactionResponse<Data> = res.json::<TransactionResponse<Data>>().await?;
        Ok(response_body.data.id)
    } else {
        let error_body: String = res.json::<TransactionResponse<Error>>().await?.data.message;
        println!("Failed: {:?}", error_body);
        Err(format!("Request Failed: {:?}", error_body).into())
    }
    }

}