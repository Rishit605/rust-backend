use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_config::Config;
use crate::model::task::{Task, TaskeState};
use log::error;
use std::str::FromStr;
use std::collections::HashMap;

pub struct DDBRepository {
    client : Client,
    table_name: String
}
