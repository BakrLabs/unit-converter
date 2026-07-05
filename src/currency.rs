use crate::errors::AppError;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct ExchangeRateResponse {
    rates: HashMap<String, f64>,
}

pub async fn convert(from: &str, to: &str, value: f64) -> Result<f64, AppError> {
    let url = format!("https://api.exchangerate-api.com/v4/latest/{}", from.to_uppercase());
    
    let response = reqwest::get(&url)
        .await
        .map_err(|e| AppError::NetworkError(e.to_string()))?;
        
    let data = response
        .json::<ExchangeRateResponse>()
        .await
        .map_err(|e| AppError::ParsingError(e.to_string()))?;
        
    let rate = data.rates.get(&to.to_uppercase())
        .ok_or_else(|| AppError::InvalidUnit(to.to_string()))?;
        
    Ok(value * rate)
}