use std::thread;

use reqwest::{Result};
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::log;
use rocket::serde::json::to_string;
use rocket::tokio;
use bambangshop::{REQWEST_CLIENT, compose_error_response, Error};
use crate::model::notification::Notification;
use crate::repository::product::ProductRepository;
use crate::repository::subscriber::{self, SubscriberRepostiory};
use crate::service::notification::NotificationService;

use super::product::Product;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber{
    pub url: String,
    pub name: String,
}

impl Subscriber{
    #[tokio::main]
    pub async fn update(&self, payload: Notification){
        REQWEST_CLIENT
            .post(&self.url)
            .header("Content-Type", "JSON")
            .body(to_string(&payload).unwrap())
            .send().await.ok();
        log::warn_!("Sent {} notification of: [{}] {}, to: {}",
            payload.status, payload.product_type, payload.product_title, self.url);
    }
}