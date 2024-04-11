use env_logger::Env;
use log::{debug, info, trace, warn};
use r2d2_mysql::mysql::OptsBuilder;
use std::time::{SystemTime, Duration};
use std::collections::HashMap;
use i_dao::{i_mysql, sql};
use crate::model::test_user::TestUser;
use std::any::Any;

mod model;
mod dao;
mod service;



fn test_init() {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    debug!("Hello, world!");

    service::set_date_source_key(String::from("mysql_db1"));
    debug!("{:?}", service::get_data_source_key());

    let opts = OptsBuilder::new()
        .ip_or_hostname(Some("localhost"))
        .user(Some("root"))
        .pass(Some("123456"))
        .db_name(Some("test_rs"))
        .tcp_port(3306)
        .tcp_connect_timeout(Some(Duration::from_secs(30)));

    i_mysql::init(service::get_data_source_key(), opts, 200, 5);
    let conn = i_mysql::get_conn(&service::get_data_source_key());
    trace!("{:?}", conn);
}

fn main() {
    test_init();
    test_add();
    test_add_batch();
    test_find();
    test_update();
    test_query_list();
}

fn test_add() {
    // let mut t1 = model::test_user::TestUser::new("xcy".to_string(), 1);

    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let mut t1 = TestUser::new(format!("xcy 0409 {}", now), 1);
    debug!("{:?}", t1);

    let add_res = service::test_user_sve::add(&mut t1);
    debug!("{:?}", add_res);
    if add_res.is_err() {
        debug!("调用 service 方法出现错误");
    }
    info!("{:?}", t1);
}

fn test_find() {
    debug!("----------- test_find --------------------");
    let id: u64 = 71;

    let res = service::test_user_sve::find_by_id(id);
    debug!("{:?}", res);
    if res.is_err(){
        warn!("test_find 出现异常 {:?}", res);
        return;
    }

    let t = res.unwrap();
    debug!("{:?}", t);
    match t {
        Some(user) => {
            debug!("找到用户 user={:?}", user);
        },
        None => {
            debug!("未找到用户id={}", id);
        }
    }
}

fn test_query_list(){
    let mut params:HashMap<String, Box<dyn Any>> = HashMap::new();
    // params.insert(String::from(format!("{}state", foundation::dao::GT)), Box::new(1));
    params.insert(String::from(format!("{}state", sql::GT_EQ)), Box::new(1));

    // params.insert(String::from("user_name"), Box::new(String::from("XINYI_Doge")));
    params.insert(String::from(format!("{}id", sql::GT)), Box::new(7u64));

    let page_index = sql::Condition::PageIndex(1);
    let page_size = sql::Condition::PageSize(3);
    let asc = sql::Condition::OrderByAESOrDESC(1);

    let bc = [page_index, page_size, asc, ];

    let result = service::test_user_sve::query_list(&params, &bc);
    if result.is_err(){
        warn!("出现异常 {:?}", result);
        return;
    }
    let res = result.unwrap();
    for i in &res {
        debug!(
                "id = {}, created_at = {}, updated_at = {}, user_name = {}, state = {}",
                i.id, i.created_at, i.updated_at, i.user_name, i.state
            );
    }

    let result = service::test_user_sve::query_count(&params, &bc);
    info!("查询 query_count 数量={:?}", result)
}

fn test_update() {
    debug!("----------- test_update --------------------");
    let id: u64 = 71;

    let res = service::test_user_sve::find_by_id(id);
    debug!("{:?}", res);
    if res.is_err(){
        warn!("test_find 出现异常 {:?}", res);
        return;
    }

    let t = res.unwrap();
    debug!("{:?}", t);
    match t {
        Some(mut user) => {
            debug!("找到用户 user={:?}", user);
            // user.user_name = String::from("XINYI_Doge");
            user.state = 2;
            let u_res = service::test_user_sve::update_by_id(&mut user);
            if u_res.is_err() {
                warn!("出现异常 {:?}", u_res);
                return;
            }

            debug!("更新后的 user={:?}", user)
        },
        None => {
            debug!("未找到用户id={}", id);
        }
    }
}

fn test_add_batch(){
    let mut lst: Vec<&mut TestUser> = Vec::new();
    let mut binding = TestUser::new("xcy 0409 01 5".to_string(), 1);
    lst.push(&mut binding);
    let mut binding2 = TestUser::new("xcy 0409 03 5".to_string(), 1);
    lst.push(&mut binding2);
    let mut binding3 = TestUser::new("xcy 0409 02 5".to_string(), 1);
    lst.push(&mut binding3);
    let res = service::test_user_sve::add_batch(&mut lst);
    debug!("{:?}", res);
    debug!("{:?}", lst);
}