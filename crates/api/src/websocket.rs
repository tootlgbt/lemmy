use crate::Perform;
use actix_web::web::Data;
use lemmy_api_common::{
  context::LemmyContext,
  utils::local_user_view_from_jwt,
  websocket::{
    handlers::join_rooms::{JoinCommunityRoom, JoinModRoom, JoinPostRoom, JoinUserRoom},
    structs::{
      CommunityJoin,
      CommunityJoinResponse,
      ModJoin,
      ModJoinResponse,
      PostJoin,
      PostJoinResponse,
      UserJoin,
      UserJoinResponse,
    },
  },
};
use lemmy_utils::{error::LemmyError, ConnectionId};

#[async_trait::async_trait(?Send)]
impl Perform for UserJoin {
  type Response = UserJoinResponse;

  #[tracing::instrument(skip(context, websocket_id))]
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<UserJoinResponse, LemmyError> {
    let data: &UserJoin = self;
    let local_user_view = local_user_view_from_jwt(&data.auth, context).await?;

    if let Some(id) = websocket_id {
      context.chat_server().do_send(JoinUserRoom {
        user_id: local_user_view.local_user.id,
        id,
      });
    }

    Ok(UserJoinResponse { joined: true })
  }
}

#[async_trait::async_trait(?Send)]
impl Perform for CommunityJoin {
  type Response = CommunityJoinResponse;

  #[tracing::instrument(skip(context, websocket_id))]
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<CommunityJoinResponse, LemmyError> {
    let data: &CommunityJoin = self;

    if let Some(id) = websocket_id {
      context.chat_server().do_send(JoinCommunityRoom {
        community_id: data.community_id,
        id,
      });
    }

    Ok(CommunityJoinResponse { joined: true })
  }
}

#[async_trait::async_trait(?Send)]
impl Perform for ModJoin {
  type Response = ModJoinResponse;

  #[tracing::instrument(skip(context, websocket_id))]
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<ModJoinResponse, LemmyError> {
    let data: &ModJoin = self;

    if let Some(id) = websocket_id {
      context.chat_server().do_send(JoinModRoom {
        community_id: data.community_id,
        id,
      });
    }

    Ok(ModJoinResponse { joined: true })
  }
}

#[async_trait::async_trait(?Send)]
impl Perform for PostJoin {
  type Response = PostJoinResponse;

  #[tracing::instrument(skip(context, websocket_id))]
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<PostJoinResponse, LemmyError> {
    let data: &PostJoin = self;

    if let Some(id) = websocket_id {
      context.chat_server().do_send(JoinPostRoom {
        post_id: data.post_id,
        id,
      });
    }

    Ok(PostJoinResponse { joined: true })
  }
}
