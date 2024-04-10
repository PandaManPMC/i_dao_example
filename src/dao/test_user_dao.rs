use log::{debug, warn};
use std::collections::HashMap;
use std::any::Any;
use r2d2_mysql::mysql::{Transaction, Value, Row};
use r2d2_mysql::mysql::prelude::Queryable;
use crate::model::test_user;
use i_dao::{sql};

pub fn query_list(tx: &mut Transaction, condition_params: &HashMap<String, Box<dyn Any>>, condition: &[sql:: Condition]) -> Result<Vec<test_user::TestUser>, Box<dyn std::error::Error>> {
    let mut query_sql = format!("SELECT {} FROM {}", test_user::get_field_sql(""), test_user::TABLE_NAME);
    let mut params: Vec<Value> = vec![];

    let (mut where_sql,page_index,page_size,mut order_by_sql_field,order_by_sql_type) = sql::pot_base_condition(&mut params, &condition);

    for (key, val) in condition_params.iter() {
        let (i_key, operator) = sql::get_real_key_operator(key.to_string());
        if "" != where_sql {
            where_sql = format!(" {} AND {} {} ?", where_sql, i_key, operator)
        } else {
            where_sql = format!(" {} {} ?", i_key, operator)
        }

        if !sql::pot_params_condition(&mut params, &val) {
            warn!("test_user_dao::query_list::pot_params_condition - {} 参数装入失败", key)
        }
    }

    if "" != where_sql{
        query_sql = format!("{} WHERE {}", query_sql, where_sql);
    }
    if "" == order_by_sql_field {
        order_by_sql_field = "id".to_string();
    }
    query_sql = format!(" {} ORDER BY {} {}", query_sql, order_by_sql_field, order_by_sql_type);
    query_sql = format!("{} LIMIT {},{}", query_sql, (page_index-1) * page_size, page_size);

    debug!("test_user_dao::query_list::query_sql={}", query_sql);
    let result = tx.exec_map(
        query_sql,  params ,|row: Row| test_user::pot(row, 0)
    );

    if result.is_err() {
        warn!("b_d::test_user_dao::query_list 失败！ res={:?}", result);
        return Ok(result?);
    }

    return Ok(result?);
}

pub fn find_by_id(tx: &mut Transaction, id: u64) -> Result<Option<test_user::TestUser>, Box<dyn std::error::Error>> {
    let query_sql = format!("SELECT {} FROM {} WHERE {} = ? LIMIT 0,1", test_user::get_field_sql("") ,test_user::TABLE_NAME, test_user::FIELDS[0]);
    let result = tx.exec_map(
        query_sql, (id,),|row: Row| test_user::pot(row, 0)
    );
    if result.is_err() {
        warn!("b_d::test_user_dao::find_by_id 失败！ res={:?}", result);
        return match result {
            Err(e) => {
                Err(Box::new(e))
            },
            Ok(_) => {
                unimplemented!()
            },
        };
    }

    let mut lst = result.unwrap();
    if 0 == lst.len() {
        return Ok(None);
    }

    let one:Option<test_user::TestUser> = lst.pop();
    return Ok(one);
}