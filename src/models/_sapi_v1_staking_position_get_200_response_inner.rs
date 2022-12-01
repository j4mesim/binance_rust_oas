/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SapiV1StakingPositionGet200ResponseInner {
    #[serde(rename = "positionId")]
    pub position_id: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "purchaseTime")]
    pub purchase_time: String,
    #[serde(rename = "duration")]
    pub duration: String,
    #[serde(rename = "accrualDays")]
    pub accrual_days: String,
    #[serde(rename = "rewardAsset")]
    pub reward_asset: String,
    #[serde(rename = "APY")]
    pub apy: String,
    #[serde(rename = "rewardAmt")]
    pub reward_amt: String,
    #[serde(rename = "extraRewardAsset")]
    pub extra_reward_asset: String,
    #[serde(rename = "extraRewardAPY")]
    pub extra_reward_apy: String,
    #[serde(rename = "estExtraRewardAmt")]
    pub est_extra_reward_amt: String,
    #[serde(rename = "nextInterestPay")]
    pub next_interest_pay: String,
    #[serde(rename = "nextInterestPayDate")]
    pub next_interest_pay_date: String,
    #[serde(rename = "payInterestPeriod")]
    pub pay_interest_period: String,
    #[serde(rename = "redeemAmountEarly")]
    pub redeem_amount_early: String,
    #[serde(rename = "interestEndDate")]
    pub interest_end_date: String,
    #[serde(rename = "deliverDate")]
    pub deliver_date: String,
    #[serde(rename = "redeemPeriod")]
    pub redeem_period: String,
    #[serde(rename = "redeemingAmt")]
    pub redeeming_amt: String,
    #[serde(rename = "partialAmtDeliverDate")]
    pub partial_amt_deliver_date: String,
    #[serde(rename = "canRedeemEarly")]
    pub can_redeem_early: bool,
    #[serde(rename = "renewable")]
    pub renewable: bool,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "status")]
    pub status: String,
}

impl SapiV1StakingPositionGet200ResponseInner {
    pub fn new(position_id: String, project_id: String, asset: String, amount: String, purchase_time: String, duration: String, accrual_days: String, reward_asset: String, apy: String, reward_amt: String, extra_reward_asset: String, extra_reward_apy: String, est_extra_reward_amt: String, next_interest_pay: String, next_interest_pay_date: String, pay_interest_period: String, redeem_amount_early: String, interest_end_date: String, deliver_date: String, redeem_period: String, redeeming_amt: String, partial_amt_deliver_date: String, can_redeem_early: bool, renewable: bool, r#type: String, status: String) -> SapiV1StakingPositionGet200ResponseInner {
        SapiV1StakingPositionGet200ResponseInner {
            position_id,
            project_id,
            asset,
            amount,
            purchase_time,
            duration,
            accrual_days,
            reward_asset,
            apy,
            reward_amt,
            extra_reward_asset,
            extra_reward_apy,
            est_extra_reward_amt,
            next_interest_pay,
            next_interest_pay_date,
            pay_interest_period,
            redeem_amount_early,
            interest_end_date,
            deliver_date,
            redeem_period,
            redeeming_amt,
            partial_amt_deliver_date,
            can_redeem_early,
            renewable,
            r#type,
            status,
        }
    }
}


