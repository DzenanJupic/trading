use crate::error::*;
use std::time;
use async_trait::asny_trait;

#[asyn_trait]
trait Api {
    type SESSION;
    
    async fn new_session(&mut self) -> Result<(), Error>;
    async fn close_session(&mut self) -> Result<(), Error>;
    async fn active_session(&self) -> bool;
    async fn session(&self) -> &Self::SESSION;

    async fn get_latency(&self) -> f32;
}

#[asyn_trait]
trait Broker {
    type API;
    type ACCOUNT;
    type DERIVATIVE;
    type POSITION;
    type PRICE;
    type ORDER;

    async fn get_accounts(&self) -> Vec<Self::ACCOUNT>;
    async fn available_money(&self, account: Self::ACCOUNT) -> f32;

    async fn get_price(&self, derivative: Self::DERIVATIVE) -> Self::PRICE;
    async fn price_in_range(&self, derivative: Self::DERIVATIVE, price: Self::PRICE, plus_minus: f32) -> bool;

    async fn sma(&self, derivative: Self::DERIVATIVE, time_frame: time::Duration, resolution: time::Duration) -> Vec<f32>;
    async fn ema(&self, derivative: Self::DERIVATIVE, time_frame: time::Duration, resolution: time::Duration) -> Vec<f32>;

    async fn buy(&mut self, order: Self::ORDER) -> Result<(), Error>;
    async fn sell(&mut self, position: Self::POSITION) -> Result<(), Error>;
    async fn current_positions(&self) -> Vec<Self::POSITION>;
    async fn position_is_active(&self, position: Self::POSITION) -> bool;
}
