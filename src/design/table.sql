
drop index uni_user_name on test_user;

drop table if exists test_user;

/*==============================================================*/
/* Table: test_user                                             */
/*==============================================================*/
create table test_user
(
    id                   bigint unsigned not null auto_increment comment 'search会员编号',
    created_at           bigint unsigned not null comment '创建时间',
    updated_at           bigint unsigned not null comment '最后更新',
    user_name            varchar(20) not null default '' comment 'search用户名',
    state                tinyint unsigned default 1 comment 'thing状态:1@正常;2@冻结;3@锁定',
    primary key (id)
);

alter table test_user comment '会员';

/*==============================================================*/
/* Index: uni_user_name                                         */
/*==============================================================*/
create unique index uni_user_name on test_user
    (
     user_name
        );
