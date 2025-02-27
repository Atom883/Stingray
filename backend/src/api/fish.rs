use std::cmp::min;

use crate::{
    StingrayState,
    auth::get_user_id_from_request,
    domain::{
        entities::{Feed, UserData},
        repositories::{
            session_repository::SessionRepository, user_data_repository::UserDataRepository,
            user_repository::UserRepository,
        },
        transaction_manager::TransactionManager,
    },
    response::AppResult,
};
use anyhow::Context;
use axum::{Json, extract::State, http::HeaderMap, response::IntoResponse};
use fxhash::FxHashMap;

pub async fn eat_fish<
    Conn: Send,
    Txm: TransactionManager<Conn>,
    SR: SessionRepository<Conn>,
    UR: UserRepository<Conn>,
    UDR: UserDataRepository<Conn>,
>(
    State(StingrayState {
        txm,
        session_repository,
        user_data_repository,
        ..
    }): State<StingrayState<Conn, Txm, SR, UR, UDR>>,
    headers: HeaderMap,
    Json(feed): Json<char>,
) -> AppResult<impl IntoResponse> {
    let user_id = get_user_id_from_request(txm.clone(), session_repository, headers).await?;

    let UserData {
        user_id,
        mut a_state,
        mut feeds,
    } = txm
        .run(|ctx| user_data_repository.get_by_user_id(&user_id, ctx))
        .await?
        .context("user_data not found")?;

    let feed: Feed = feed.try_into()?;

    let feed_cnt = feeds
        .get_mut(&feed)
        .ok_or_else(|| anyhow::anyhow!("feed not found"))?;
    *feed_cnt -= 1;
    if *feed_cnt == 0 {
        feeds.remove(&feed);
    }

    a_state.hp = if feed.is_a() {
        a_state.max_hp
    } else {
        min(a_state.hp + 1, a_state.max_hp)
    };

    let user_data = UserData {
        user_id,
        a_state,
        feeds,
    };

    txm.run(|ctx| user_data_repository.update(user_data.clone(), ctx))
        .await?;

    Ok(Json(user_data))
}

pub async fn add_fish<
    Conn: Send,
    Txm: TransactionManager<Conn>,
    SR: SessionRepository<Conn>,
    UR: UserRepository<Conn>,
    UDR: UserDataRepository<Conn>,
>(
    State(StingrayState {
        txm,
        session_repository,
        user_data_repository,
        ..
    }): State<StingrayState<Conn, Txm, SR, UR, UDR>>,
    headers: HeaderMap,
    Json(adding_feeds): Json<FxHashMap<char, u64>>,
) -> AppResult<impl IntoResponse> {
    let user_id = get_user_id_from_request(txm.clone(), session_repository, headers).await?;

    let UserData {
        user_id,
        a_state,
        mut feeds,
    } = txm
        .run(|ctx| user_data_repository.get_by_user_id(&user_id, ctx))
        .await?
        .context("user_data not found")?;

    let adding_feeds = adding_feeds
        .clone()
        .into_iter()
        .map(|(k, v)| Ok((k.try_into()?, v)))
        .collect::<Result<FxHashMap<Feed, u64>, anyhow::Error>>()?;

    for (k, v) in adding_feeds {
        *feeds.entry(k).or_insert(0) += v;
    }

    let user_data = UserData {
        user_id,
        a_state,
        feeds,
    };
    txm.run(|ctx| user_data_repository.update(user_data.clone(), ctx))
        .await?;

    Ok(Json(user_data))
}
