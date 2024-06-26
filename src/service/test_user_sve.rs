use mysql::{Result};
use i_dao::{dao, i_mysql, sql};
use std::collections::HashMap;
use std::any::Any;
use std::result::Result::Ok;
use r2d2_mysql::mysql::Transaction;
use r2d2_mysql::{MySqlConnectionManager, r2d2};
use crate::{model::test_user, dao::test_user_dao, service};

pub fn add(m: &mut test_user::TestUser) -> Result<(), Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction | -> Result<(), Box<dyn std::error::Error>>  {
        return dao::add(tx, m);
    };
    return Ok(i_mysql::start_tx(&service::get_data_source_key(), &mut call)?);
}

pub fn add_batch(lst: &mut Vec<&mut test_user::TestUser>) -> Result<(),Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<(), Box<dyn std::error::Error>>  {
        return dao::add_batch(tx, lst);
    };
    return Ok(i_mysql::start_tx(&service::get_data_source_key(), &mut call)?);
}

pub fn update_by_id(m: &mut test_user::TestUser) -> Result<(),Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<(), Box<dyn std::error::Error>>  {
        return dao::update_by_pk(tx, m);
    };
    return Ok(i_mysql::start_tx(&service::get_data_source_key(), &mut call)?);
}

pub fn query_list(params: &HashMap<String, Box<dyn Any>>, condition: &[sql::Condition]) -> Result<Vec<test_user::TestUser>, Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<Vec<test_user::TestUser>, Box<dyn std::error::Error>>  {
        let result = test_user_dao::query_list(tx, params, condition);
        return Ok(result?);
    };
    return Ok(i_mysql::start_tx(&service::get_data_source_key(), &mut call)?);
}

pub fn find_by_id(id: u64) -> Result<Option<test_user::TestUser>, Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<test_user::TestUser>, Box<dyn std::error::Error>>  {
        let result = test_user_dao::find_by_id(tx, id);
        return Ok(result?);
    };
    return Ok(i_mysql::start_tx(&service::get_data_source_key(), &mut call)?);
}

pub fn query_count(params: &HashMap<String, Box<dyn Any>>, condition: &[sql::Condition]) -> Result<u64, Box<dyn std::error::Error>> {
    let mut call = | conn:&mut r2d2::PooledConnection<MySqlConnectionManager> |  -> Result<u64, Box<dyn std::error::Error>>  {
        let result = test_user_dao::query_count(conn, params, condition);
        return Ok(result?);
    };
    return Ok(i_mysql::direct(&service::get_data_source_key(), &mut call)?);
}
