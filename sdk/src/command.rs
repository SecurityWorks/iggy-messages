use crate::bytes_serializable::BytesSerializable;
use crate::consumer_groups::create_consumer_group::CreateConsumerGroup;
use crate::consumer_groups::delete_consumer_group::DeleteConsumerGroup;
use crate::consumer_groups::get_consumer_group::GetConsumerGroup;
use crate::consumer_groups::get_consumer_groups::GetConsumerGroups;
use crate::consumer_groups::join_consumer_group::JoinConsumerGroup;
use crate::consumer_groups::leave_consumer_group::LeaveConsumerGroup;
use crate::consumer_offsets::get_consumer_offset::GetConsumerOffset;
use crate::consumer_offsets::store_consumer_offset::StoreConsumerOffset;
use crate::error::IggyError;
use crate::messages::poll_messages::PollMessages;
use crate::messages::send_messages::SendMessages;
use crate::partitions::create_partitions::CreatePartitions;
use crate::partitions::delete_partitions::DeletePartitions;
use crate::personal_access_tokens::create_personal_access_token::CreatePersonalAccessToken;
use crate::personal_access_tokens::delete_personal_access_token::DeletePersonalAccessToken;
use crate::personal_access_tokens::get_personal_access_tokens::GetPersonalAccessTokens;
use crate::personal_access_tokens::login_with_personal_access_token::LoginWithPersonalAccessToken;
use crate::streams::create_stream::CreateStream;
use crate::streams::delete_stream::DeleteStream;
use crate::streams::get_stream::GetStream;
use crate::streams::get_streams::GetStreams;
use crate::streams::purge_stream::PurgeStream;
use crate::streams::update_stream::UpdateStream;
use crate::system::get_client::GetClient;
use crate::system::get_clients::GetClients;
use crate::system::get_me::GetMe;
use crate::system::get_stats::GetStats;
use crate::system::ping::Ping;
use crate::topics::create_topic::CreateTopic;
use crate::topics::delete_topic::DeleteTopic;
use crate::topics::get_topic::GetTopic;
use crate::topics::get_topics::GetTopics;
use crate::topics::purge_topic::PurgeTopic;
use crate::topics::update_topic::UpdateTopic;
use crate::users::change_password::ChangePassword;
use crate::users::create_user::CreateUser;
use crate::users::delete_user::DeleteUser;
use crate::users::get_user::GetUser;
use crate::users::get_users::GetUsers;
use crate::users::login_user::LoginUser;
use crate::users::logout_user::LogoutUser;
use crate::users::update_permissions::UpdatePermissions;
use crate::users::update_user::UpdateUser;
use bytes::{BufMut, Bytes, BytesMut};
use std::fmt::{Display, Formatter};
use strum::EnumString;

pub const PING: &str = "ping";
pub const PING_CODE: u32 = 1;
pub const GET_STATS: &str = "stats";
pub const GET_STATS_CODE: u32 = 10;
pub const GET_ME: &str = "me";
pub const GET_ME_CODE: u32 = 20;
pub const GET_CLIENT: &str = "client.get";
pub const GET_CLIENT_CODE: u32 = 21;
pub const GET_CLIENTS: &str = "client.list";
pub const GET_CLIENTS_CODE: u32 = 22;
pub const GET_USER: &str = "user.get";
pub const GET_USER_CODE: u32 = 31;
pub const GET_USERS: &str = "user.list";
pub const GET_USERS_CODE: u32 = 32;
pub const CREATE_USER: &str = "user.create";
pub const CREATE_USER_CODE: u32 = 33;
pub const DELETE_USER: &str = "user.delete";
pub const DELETE_USER_CODE: u32 = 34;
pub const UPDATE_USER: &str = "user.update";
pub const UPDATE_USER_CODE: u32 = 35;
pub const UPDATE_PERMISSIONS: &str = "user.permissions";
pub const UPDATE_PERMISSIONS_CODE: u32 = 36;
pub const CHANGE_PASSWORD: &str = "user.password";
pub const CHANGE_PASSWORD_CODE: u32 = 37;
pub const LOGIN_USER: &str = "user.login";
pub const LOGIN_USER_CODE: u32 = 38;
pub const LOGOUT_USER: &str = "user.logout";
pub const LOGOUT_USER_CODE: u32 = 39;
pub const GET_PERSONAL_ACCESS_TOKENS: &str = "personal_access_token.list";
pub const GET_PERSONAL_ACCESS_TOKENS_CODE: u32 = 41;
pub const CREATE_PERSONAL_ACCESS_TOKEN: &str = "personal_access_token.create";
pub const CREATE_PERSONAL_ACCESS_TOKEN_CODE: u32 = 42;
pub const DELETE_PERSONAL_ACCESS_TOKEN: &str = "personal_access_token.delete";
pub const DELETE_PERSONAL_ACCESS_TOKEN_CODE: u32 = 43;
pub const LOGIN_WITH_PERSONAL_ACCESS_TOKEN: &str = "personal_access_token.login";
pub const LOGIN_WITH_PERSONAL_ACCESS_TOKEN_CODE: u32 = 44;
pub const POLL_MESSAGES: &str = "message.poll";
pub const POLL_MESSAGES_CODE: u32 = 100;
pub const SEND_MESSAGES: &str = "message.send";
pub const SEND_MESSAGES_CODE: u32 = 101;
pub const GET_CONSUMER_OFFSET: &str = "consumer_offset.get";
pub const GET_CONSUMER_OFFSET_CODE: u32 = 120;
pub const STORE_CONSUMER_OFFSET: &str = "consumer_offset.store";
pub const STORE_CONSUMER_OFFSET_CODE: u32 = 121;
pub const GET_STREAM: &str = "stream.get";
pub const GET_STREAM_CODE: u32 = 200;
pub const GET_STREAMS: &str = "stream.list";
pub const GET_STREAMS_CODE: u32 = 201;
pub const CREATE_STREAM: &str = "stream.create";
pub const CREATE_STREAM_CODE: u32 = 202;
pub const DELETE_STREAM: &str = "stream.delete";
pub const DELETE_STREAM_CODE: u32 = 203;
pub const UPDATE_STREAM: &str = "stream.update";
pub const UPDATE_STREAM_CODE: u32 = 204;
pub const PURGE_STREAM: &str = "stream.purge";
pub const PURGE_STREAM_CODE: u32 = 205;
pub const GET_TOPIC: &str = "topic.get";
pub const GET_TOPIC_CODE: u32 = 300;
pub const GET_TOPICS: &str = "topic.list";
pub const GET_TOPICS_CODE: u32 = 301;
pub const CREATE_TOPIC: &str = "topic.create";
pub const CREATE_TOPIC_CODE: u32 = 302;
pub const DELETE_TOPIC: &str = "topic.delete";
pub const DELETE_TOPIC_CODE: u32 = 303;
pub const UPDATE_TOPIC: &str = "topic.update";
pub const UPDATE_TOPIC_CODE: u32 = 304;
pub const PURGE_TOPIC: &str = "topic.purge";
pub const PURGE_TOPIC_CODE: u32 = 305;
pub const CREATE_PARTITIONS: &str = "partition.create";
pub const CREATE_PARTITIONS_CODE: u32 = 402;
pub const DELETE_PARTITIONS: &str = "partition.delete";
pub const DELETE_PARTITIONS_CODE: u32 = 403;
pub const GET_CONSUMER_GROUP: &str = "consumer_group.get";
pub const GET_CONSUMER_GROUP_CODE: u32 = 600;
pub const GET_CONSUMER_GROUPS: &str = "consumer_group.list";
pub const GET_CONSUMER_GROUPS_CODE: u32 = 601;
pub const CREATE_CONSUMER_GROUP: &str = "consumer_group.create";
pub const CREATE_CONSUMER_GROUP_CODE: u32 = 602;
pub const DELETE_CONSUMER_GROUP: &str = "consumer_group.delete";
pub const DELETE_CONSUMER_GROUP_CODE: u32 = 603;
pub const JOIN_CONSUMER_GROUP: &str = "consumer_group.join";
pub const JOIN_CONSUMER_GROUP_CODE: u32 = 604;
pub const LEAVE_CONSUMER_GROUP: &str = "consumer_group.leave";
pub const LEAVE_CONSUMER_GROUP_CODE: u32 = 605;

#[derive(Debug, PartialEq, EnumString)]
pub enum Command {
    Ping(Ping),
    GetStats(GetStats),
    GetMe(GetMe),
    GetClient(GetClient),
    GetClients(GetClients),
    GetUser(GetUser),
    GetUsers(GetUsers),
    CreateUser(CreateUser),
    DeleteUser(DeleteUser),
    UpdateUser(UpdateUser),
    UpdatePermissions(UpdatePermissions),
    ChangePassword(ChangePassword),
    LoginUser(LoginUser),
    LogoutUser(LogoutUser),
    GetPersonalAccessTokens(GetPersonalAccessTokens),
    CreatePersonalAccessToken(CreatePersonalAccessToken),
    DeletePersonalAccessToken(DeletePersonalAccessToken),
    LoginWithPersonalAccessToken(LoginWithPersonalAccessToken),
    SendMessages(SendMessages),
    PollMessages(PollMessages),
    GetConsumerOffset(GetConsumerOffset),
    StoreConsumerOffset(StoreConsumerOffset),
    GetStream(GetStream),
    GetStreams(GetStreams),
    CreateStream(CreateStream),
    DeleteStream(DeleteStream),
    UpdateStream(UpdateStream),
    PurgeStream(PurgeStream),
    GetTopic(GetTopic),
    GetTopics(GetTopics),
    CreateTopic(CreateTopic),
    DeleteTopic(DeleteTopic),
    UpdateTopic(UpdateTopic),
    PurgeTopic(PurgeTopic),
    CreatePartitions(CreatePartitions),
    DeletePartitions(DeletePartitions),
    GetConsumerGroup(GetConsumerGroup),
    GetConsumerGroups(GetConsumerGroups),
    CreateConsumerGroup(CreateConsumerGroup),
    DeleteConsumerGroup(DeleteConsumerGroup),
    JoinConsumerGroup(JoinConsumerGroup),
    LeaveConsumerGroup(LeaveConsumerGroup),
}

/// A trait for all command payloads.
pub trait CommandPayload: BytesSerializable + Display {}

impl BytesSerializable for Command {
    fn as_bytes(&self) -> Bytes {
        match self {
            Command::Ping(payload) => as_bytes(PING_CODE, payload.as_bytes()),
            Command::GetStats(payload) => as_bytes(GET_STATS_CODE, payload.as_bytes()),
            Command::GetMe(payload) => as_bytes(GET_ME_CODE, payload.as_bytes()),
            Command::GetClient(payload) => as_bytes(GET_CLIENT_CODE, payload.as_bytes()),
            Command::GetClients(payload) => as_bytes(GET_CLIENTS_CODE, payload.as_bytes()),
            Command::GetUser(payload) => as_bytes(GET_USER_CODE, payload.as_bytes()),
            Command::GetUsers(payload) => as_bytes(GET_USERS_CODE, payload.as_bytes()),
            Command::CreateUser(payload) => as_bytes(CREATE_USER_CODE, payload.as_bytes()),
            Command::DeleteUser(payload) => as_bytes(DELETE_USER_CODE, payload.as_bytes()),
            Command::UpdateUser(payload) => as_bytes(UPDATE_USER_CODE, payload.as_bytes()),
            Command::UpdatePermissions(payload) => {
                as_bytes(UPDATE_PERMISSIONS_CODE, payload.as_bytes())
            }
            Command::ChangePassword(payload) => as_bytes(CHANGE_PASSWORD_CODE, payload.as_bytes()),
            Command::LoginUser(payload) => as_bytes(LOGIN_USER_CODE, payload.as_bytes()),
            Command::LogoutUser(payload) => as_bytes(LOGOUT_USER_CODE, payload.as_bytes()),
            Command::GetPersonalAccessTokens(payload) => {
                as_bytes(GET_PERSONAL_ACCESS_TOKENS_CODE, payload.as_bytes())
            }
            Command::CreatePersonalAccessToken(payload) => {
                as_bytes(CREATE_PERSONAL_ACCESS_TOKEN_CODE, payload.as_bytes())
            }
            Command::DeletePersonalAccessToken(payload) => {
                as_bytes(DELETE_PERSONAL_ACCESS_TOKEN_CODE, payload.as_bytes())
            }
            Command::LoginWithPersonalAccessToken(payload) => {
                as_bytes(LOGIN_WITH_PERSONAL_ACCESS_TOKEN_CODE, payload.as_bytes())
            }
            Command::SendMessages(payload) => as_bytes(SEND_MESSAGES_CODE, payload.as_bytes()),
            Command::PollMessages(payload) => as_bytes(POLL_MESSAGES_CODE, payload.as_bytes()),
            Command::StoreConsumerOffset(payload) => {
                as_bytes(STORE_CONSUMER_OFFSET_CODE, payload.as_bytes())
            }
            Command::GetConsumerOffset(payload) => {
                as_bytes(GET_CONSUMER_OFFSET_CODE, payload.as_bytes())
            }
            Command::GetStream(payload) => as_bytes(GET_STREAM_CODE, payload.as_bytes()),
            Command::GetStreams(payload) => as_bytes(GET_STREAMS_CODE, payload.as_bytes()),
            Command::CreateStream(payload) => as_bytes(CREATE_STREAM_CODE, payload.as_bytes()),
            Command::DeleteStream(payload) => as_bytes(DELETE_STREAM_CODE, payload.as_bytes()),
            Command::UpdateStream(payload) => as_bytes(UPDATE_STREAM_CODE, payload.as_bytes()),
            Command::PurgeStream(payload) => as_bytes(PURGE_STREAM_CODE, payload.as_bytes()),
            Command::GetTopic(payload) => as_bytes(GET_TOPIC_CODE, payload.as_bytes()),
            Command::GetTopics(payload) => as_bytes(GET_TOPICS_CODE, payload.as_bytes()),
            Command::CreateTopic(payload) => as_bytes(CREATE_TOPIC_CODE, payload.as_bytes()),
            Command::DeleteTopic(payload) => as_bytes(DELETE_TOPIC_CODE, payload.as_bytes()),
            Command::UpdateTopic(payload) => as_bytes(UPDATE_TOPIC_CODE, payload.as_bytes()),
            Command::PurgeTopic(payload) => as_bytes(PURGE_TOPIC_CODE, payload.as_bytes()),
            Command::CreatePartitions(payload) => {
                as_bytes(CREATE_PARTITIONS_CODE, payload.as_bytes())
            }
            Command::DeletePartitions(payload) => {
                as_bytes(DELETE_PARTITIONS_CODE, payload.as_bytes())
            }
            Command::GetConsumerGroup(payload) => {
                as_bytes(GET_CONSUMER_GROUP_CODE, payload.as_bytes())
            }
            Command::GetConsumerGroups(payload) => {
                as_bytes(GET_CONSUMER_GROUPS_CODE, payload.as_bytes())
            }
            Command::CreateConsumerGroup(payload) => {
                as_bytes(CREATE_CONSUMER_GROUP_CODE, payload.as_bytes())
            }
            Command::DeleteConsumerGroup(payload) => {
                as_bytes(DELETE_CONSUMER_GROUP_CODE, payload.as_bytes())
            }
            Command::JoinConsumerGroup(payload) => {
                as_bytes(JOIN_CONSUMER_GROUP_CODE, payload.as_bytes())
            }
            Command::LeaveConsumerGroup(payload) => {
                as_bytes(LEAVE_CONSUMER_GROUP_CODE, payload.as_bytes())
            }
        }
    }

    fn from_bytes(bytes: Bytes) -> Result<Self, IggyError> {
        let command = u32::from_le_bytes(bytes[..4].try_into()?);
        let payload = bytes.slice(4..);
        match command {
            PING_CODE => Ok(Command::Ping(Ping::from_bytes(payload)?)),
            GET_STATS_CODE => Ok(Command::GetStats(GetStats::from_bytes(payload)?)),
            GET_ME_CODE => Ok(Command::GetMe(GetMe::from_bytes(payload)?)),
            GET_CLIENT_CODE => Ok(Command::GetClient(GetClient::from_bytes(payload)?)),
            GET_CLIENTS_CODE => Ok(Command::GetClients(GetClients::from_bytes(payload)?)),
            GET_USER_CODE => Ok(Command::GetUser(GetUser::from_bytes(payload)?)),
            GET_USERS_CODE => Ok(Command::GetUsers(GetUsers::from_bytes(payload)?)),
            CREATE_USER_CODE => Ok(Command::CreateUser(CreateUser::from_bytes(payload)?)),
            DELETE_USER_CODE => Ok(Command::DeleteUser(DeleteUser::from_bytes(payload)?)),
            UPDATE_USER_CODE => Ok(Command::UpdateUser(UpdateUser::from_bytes(payload)?)),
            UPDATE_PERMISSIONS_CODE => Ok(Command::UpdatePermissions(
                UpdatePermissions::from_bytes(payload)?,
            )),
            CHANGE_PASSWORD_CODE => Ok(Command::ChangePassword(ChangePassword::from_bytes(
                payload,
            )?)),
            LOGIN_USER_CODE => Ok(Command::LoginUser(LoginUser::from_bytes(payload)?)),
            LOGOUT_USER_CODE => Ok(Command::LogoutUser(LogoutUser::from_bytes(payload)?)),
            GET_PERSONAL_ACCESS_TOKENS_CODE => Ok(Command::GetPersonalAccessTokens(
                GetPersonalAccessTokens::from_bytes(payload)?,
            )),
            CREATE_PERSONAL_ACCESS_TOKEN_CODE => Ok(Command::CreatePersonalAccessToken(
                CreatePersonalAccessToken::from_bytes(payload)?,
            )),
            DELETE_PERSONAL_ACCESS_TOKEN_CODE => Ok(Command::DeletePersonalAccessToken(
                DeletePersonalAccessToken::from_bytes(payload)?,
            )),
            LOGIN_WITH_PERSONAL_ACCESS_TOKEN_CODE => Ok(Command::LoginWithPersonalAccessToken(
                LoginWithPersonalAccessToken::from_bytes(payload)?,
            )),
            SEND_MESSAGES_CODE => Ok(Command::SendMessages(SendMessages::from_bytes(payload)?)),
            POLL_MESSAGES_CODE => Ok(Command::PollMessages(PollMessages::from_bytes(payload)?)),
            STORE_CONSUMER_OFFSET_CODE => Ok(Command::StoreConsumerOffset(
                StoreConsumerOffset::from_bytes(payload)?,
            )),
            GET_CONSUMER_OFFSET_CODE => Ok(Command::GetConsumerOffset(
                GetConsumerOffset::from_bytes(payload)?,
            )),
            GET_STREAM_CODE => Ok(Command::GetStream(GetStream::from_bytes(payload)?)),
            GET_STREAMS_CODE => Ok(Command::GetStreams(GetStreams::from_bytes(payload)?)),
            CREATE_STREAM_CODE => Ok(Command::CreateStream(CreateStream::from_bytes(payload)?)),
            DELETE_STREAM_CODE => Ok(Command::DeleteStream(DeleteStream::from_bytes(payload)?)),
            UPDATE_STREAM_CODE => Ok(Command::UpdateStream(UpdateStream::from_bytes(payload)?)),
            PURGE_STREAM_CODE => Ok(Command::PurgeStream(PurgeStream::from_bytes(payload)?)),
            GET_TOPIC_CODE => Ok(Command::GetTopic(GetTopic::from_bytes(payload)?)),
            GET_TOPICS_CODE => Ok(Command::GetTopics(GetTopics::from_bytes(payload)?)),
            CREATE_TOPIC_CODE => Ok(Command::CreateTopic(CreateTopic::from_bytes(payload)?)),
            DELETE_TOPIC_CODE => Ok(Command::DeleteTopic(DeleteTopic::from_bytes(payload)?)),
            UPDATE_TOPIC_CODE => Ok(Command::UpdateTopic(UpdateTopic::from_bytes(payload)?)),
            PURGE_TOPIC_CODE => Ok(Command::PurgeTopic(PurgeTopic::from_bytes(payload)?)),
            CREATE_PARTITIONS_CODE => Ok(Command::CreatePartitions(CreatePartitions::from_bytes(
                payload,
            )?)),
            DELETE_PARTITIONS_CODE => Ok(Command::DeletePartitions(DeletePartitions::from_bytes(
                payload,
            )?)),
            GET_CONSUMER_GROUP_CODE => Ok(Command::GetConsumerGroup(GetConsumerGroup::from_bytes(
                payload,
            )?)),
            GET_CONSUMER_GROUPS_CODE => Ok(Command::GetConsumerGroups(
                GetConsumerGroups::from_bytes(payload)?,
            )),
            CREATE_CONSUMER_GROUP_CODE => Ok(Command::CreateConsumerGroup(
                CreateConsumerGroup::from_bytes(payload)?,
            )),
            DELETE_CONSUMER_GROUP_CODE => Ok(Command::DeleteConsumerGroup(
                DeleteConsumerGroup::from_bytes(payload)?,
            )),
            JOIN_CONSUMER_GROUP_CODE => Ok(Command::JoinConsumerGroup(
                JoinConsumerGroup::from_bytes(payload)?,
            )),
            LEAVE_CONSUMER_GROUP_CODE => Ok(Command::LeaveConsumerGroup(
                LeaveConsumerGroup::from_bytes(payload)?,
            )),
            _ => Err(IggyError::InvalidCommand),
        }
    }
}

fn as_bytes(command: u32, payload: Bytes) -> Bytes {
    let mut bytes = BytesMut::with_capacity(4 + payload.len());
    bytes.put_u32_le(command);
    bytes.put_slice(&payload);
    bytes.freeze()
}

impl Display for Command {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Ping(_) => write!(formatter, "{PING}"),
            Command::GetStats(_) => write!(formatter, "{GET_STATS}"),
            Command::GetMe(_) => write!(formatter, "{GET_ME}"),
            Command::GetClient(payload) => write!(formatter, "{GET_CLIENT}|{payload}"),
            Command::GetClients(_) => write!(formatter, "{GET_CLIENTS}"),
            Command::GetUser(payload) => write!(formatter, "{GET_USER}|{payload}"),
            Command::GetUsers(_) => write!(formatter, "{GET_USERS}"),
            Command::CreateUser(payload) => write!(formatter, "{CREATE_USER}|{payload}"),
            Command::DeleteUser(payload) => write!(formatter, "{DELETE_USER}|{payload}"),
            Command::UpdateUser(payload) => write!(formatter, "{UPDATE_USER}|{payload}"),
            Command::UpdatePermissions(payload) => {
                write!(formatter, "{UPDATE_PERMISSIONS}|{payload}")
            }
            Command::ChangePassword(payload) => {
                write!(formatter, "{CHANGE_PASSWORD}|{payload}")
            }
            Command::LoginUser(payload) => write!(formatter, "{LOGIN_USER}|{payload}"),
            Command::LogoutUser(_) => write!(formatter, "{LOGOUT_USER}"),
            Command::GetPersonalAccessTokens(_) => {
                write!(formatter, "{GET_PERSONAL_ACCESS_TOKENS}")
            }
            Command::CreatePersonalAccessToken(payload) => {
                write!(formatter, "{CREATE_PERSONAL_ACCESS_TOKEN}|{payload}")
            }
            Command::DeletePersonalAccessToken(payload) => {
                write!(formatter, "{DELETE_PERSONAL_ACCESS_TOKEN}|{payload}")
            }
            Command::LoginWithPersonalAccessToken(payload) => {
                write!(formatter, "{LOGIN_WITH_PERSONAL_ACCESS_TOKEN}|{payload}")
            }
            Command::GetStream(payload) => write!(formatter, "{GET_STREAM}|{payload}"),
            Command::GetStreams(_) => write!(formatter, "{GET_STREAMS}"),
            Command::CreateStream(payload) => write!(formatter, "{CREATE_STREAM}|{payload}"),
            Command::DeleteStream(payload) => write!(formatter, "{DELETE_STREAM}|{payload}"),
            Command::UpdateStream(payload) => write!(formatter, "{UPDATE_STREAM}|{payload}"),
            Command::PurgeStream(payload) => write!(formatter, "{PURGE_STREAM}|{payload}"),
            Command::GetTopic(payload) => write!(formatter, "{GET_TOPIC}|{payload}"),
            Command::GetTopics(payload) => write!(formatter, "{GET_TOPICS}|{payload}"),
            Command::CreateTopic(payload) => write!(formatter, "{CREATE_TOPIC}|{payload}"),
            Command::DeleteTopic(payload) => write!(formatter, "{DELETE_TOPIC}|{payload}"),
            Command::UpdateTopic(payload) => write!(formatter, "{UPDATE_TOPIC}|{payload}"),
            Command::PurgeTopic(payload) => write!(formatter, "{PURGE_TOPIC}|{payload}"),
            Command::CreatePartitions(payload) => {
                write!(formatter, "{CREATE_PARTITIONS}|{payload}")
            }
            Command::DeletePartitions(payload) => {
                write!(formatter, "{DELETE_PARTITIONS}|{payload}")
            }
            Command::PollMessages(payload) => write!(formatter, "{POLL_MESSAGES}|{payload}"),
            Command::SendMessages(payload) => write!(formatter, "{SEND_MESSAGES}|{payload}"),
            Command::StoreConsumerOffset(payload) => {
                write!(formatter, "{STORE_CONSUMER_OFFSET}|{payload}")
            }
            Command::GetConsumerOffset(payload) => {
                write!(formatter, "{GET_CONSUMER_OFFSET}|{payload}")
            }
            Command::GetConsumerGroup(payload) => {
                write!(formatter, "{GET_CONSUMER_GROUP}|{payload}")
            }
            Command::GetConsumerGroups(payload) => {
                write!(formatter, "{GET_CONSUMER_GROUPS}|{payload}")
            }
            Command::CreateConsumerGroup(payload) => {
                write!(formatter, "{CREATE_CONSUMER_GROUP}|{payload}")
            }
            Command::DeleteConsumerGroup(payload) => {
                write!(formatter, "{DELETE_CONSUMER_GROUP}|{payload}")
            }
            Command::JoinConsumerGroup(payload) => {
                write!(formatter, "{JOIN_CONSUMER_GROUP}|{payload}")
            }
            Command::LeaveConsumerGroup(payload) => {
                write!(formatter, "{LEAVE_CONSUMER_GROUP}|{payload}")
            }
        }
    }
}

pub fn get_name_from_code(code: u32) -> Result<&'static str, IggyError> {
    match code {
        PING_CODE => Ok(PING),
        GET_STATS_CODE => Ok(GET_STATS),
        GET_ME_CODE => Ok(GET_ME),
        GET_CLIENT_CODE => Ok(GET_CLIENT),
        GET_CLIENTS_CODE => Ok(GET_CLIENTS),
        GET_USER_CODE => Ok(GET_USER),
        GET_USERS_CODE => Ok(GET_USERS),
        CREATE_USER_CODE => Ok(CREATE_USER),
        DELETE_USER_CODE => Ok(DELETE_USER),
        UPDATE_USER_CODE => Ok(UPDATE_USER),
        UPDATE_PERMISSIONS_CODE => Ok(UPDATE_PERMISSIONS),
        CHANGE_PASSWORD_CODE => Ok(CHANGE_PASSWORD),
        LOGIN_USER_CODE => Ok(LOGIN_USER),
        LOGOUT_USER_CODE => Ok(LOGOUT_USER),
        GET_PERSONAL_ACCESS_TOKENS_CODE => Ok(GET_PERSONAL_ACCESS_TOKENS),
        CREATE_PERSONAL_ACCESS_TOKEN_CODE => Ok(CREATE_PERSONAL_ACCESS_TOKEN),
        DELETE_PERSONAL_ACCESS_TOKEN_CODE => Ok(DELETE_PERSONAL_ACCESS_TOKEN),
        LOGIN_WITH_PERSONAL_ACCESS_TOKEN_CODE => Ok(LOGIN_WITH_PERSONAL_ACCESS_TOKEN),
        SEND_MESSAGES_CODE => Ok(SEND_MESSAGES),
        POLL_MESSAGES_CODE => Ok(POLL_MESSAGES),
        STORE_CONSUMER_OFFSET_CODE => Ok(STORE_CONSUMER_OFFSET),
        GET_CONSUMER_OFFSET_CODE => Ok(GET_CONSUMER_OFFSET),
        GET_STREAM_CODE => Ok(GET_STREAM),
        GET_STREAMS_CODE => Ok(GET_STREAMS),
        CREATE_STREAM_CODE => Ok(CREATE_STREAM),
        DELETE_STREAM_CODE => Ok(DELETE_STREAM),
        UPDATE_STREAM_CODE => Ok(UPDATE_STREAM),
        PURGE_STREAM_CODE => Ok(PURGE_STREAM),
        GET_TOPIC_CODE => Ok(GET_TOPIC),
        GET_TOPICS_CODE => Ok(GET_TOPICS),
        CREATE_TOPIC_CODE => Ok(CREATE_TOPIC),
        DELETE_TOPIC_CODE => Ok(DELETE_TOPIC),
        UPDATE_TOPIC_CODE => Ok(UPDATE_TOPIC),
        PURGE_TOPIC_CODE => Ok(PURGE_TOPIC),
        CREATE_PARTITIONS_CODE => Ok(CREATE_PARTITIONS),
        DELETE_PARTITIONS_CODE => Ok(DELETE_PARTITIONS),
        GET_CONSUMER_GROUP_CODE => Ok(GET_CONSUMER_GROUP),
        GET_CONSUMER_GROUPS_CODE => Ok(GET_CONSUMER_GROUPS),
        CREATE_CONSUMER_GROUP_CODE => Ok(CREATE_CONSUMER_GROUP),
        DELETE_CONSUMER_GROUP_CODE => Ok(DELETE_CONSUMER_GROUP),
        JOIN_CONSUMER_GROUP_CODE => Ok(JOIN_CONSUMER_GROUP),
        LEAVE_CONSUMER_GROUP_CODE => Ok(LEAVE_CONSUMER_GROUP),
        _ => Err(IggyError::InvalidCommand),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_serialized_as_bytes_and_deserialized_from_bytes() {
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::Ping(Ping::default()),
            PING_CODE,
            &Ping::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::GetStats(GetStats::default()),
            GET_STATS_CODE,
            &GetStats::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::GetMe(GetMe::default()),
            GET_ME_CODE,
            &GetMe::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::GetClient(GetClient::default()),
            GET_CLIENT_CODE,
            &GetClient::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::GetClients(GetClients::default()),
            GET_CLIENTS_CODE,
            &GetClients::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::GetUser(GetUser::default()),
            GET_USER_CODE,
            &GetUser::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::GetUsers(GetUsers::default()),
            GET_USERS_CODE,
            &GetUsers::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::CreateUser(CreateUser::default()),
            CREATE_USER_CODE,
            &CreateUser::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::DeleteUser(DeleteUser::default()),
            DELETE_USER_CODE,
            &DeleteUser::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::UpdateUser(UpdateUser::default()),
            UPDATE_USER_CODE,
            &UpdateUser::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::UpdatePermissions(UpdatePermissions::default()),
            UPDATE_PERMISSIONS_CODE,
            &UpdatePermissions::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::ChangePassword(ChangePassword::default()),
            CHANGE_PASSWORD_CODE,
            &ChangePassword::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::LoginUser(LoginUser::default()),
            LOGIN_USER_CODE,
            &LoginUser::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::LogoutUser(LogoutUser::default()),
            LOGOUT_USER_CODE,
            &LogoutUser::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::GetPersonalAccessTokens(GetPersonalAccessTokens::default()),
            GET_PERSONAL_ACCESS_TOKENS_CODE,
            &GetPersonalAccessTokens::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::CreatePersonalAccessToken(CreatePersonalAccessToken::default()),
            CREATE_PERSONAL_ACCESS_TOKEN_CODE,
            &CreatePersonalAccessToken::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::DeletePersonalAccessToken(DeletePersonalAccessToken::default()),
            DELETE_PERSONAL_ACCESS_TOKEN_CODE,
            &DeletePersonalAccessToken::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::LoginWithPersonalAccessToken(LoginWithPersonalAccessToken::default()),
            LOGIN_WITH_PERSONAL_ACCESS_TOKEN_CODE,
            &LoginWithPersonalAccessToken::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::SendMessages(SendMessages::default()),
            SEND_MESSAGES_CODE,
            &SendMessages::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::PollMessages(PollMessages::default()),
            POLL_MESSAGES_CODE,
            &PollMessages::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::StoreConsumerOffset(StoreConsumerOffset::default()),
            STORE_CONSUMER_OFFSET_CODE,
            &StoreConsumerOffset::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::GetConsumerOffset(GetConsumerOffset::default()),
            GET_CONSUMER_OFFSET_CODE,
            &GetConsumerOffset::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::GetStream(GetStream::default()),
            GET_STREAM_CODE,
            &GetStream::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::GetStreams(GetStreams::default()),
            GET_STREAMS_CODE,
            &GetStreams::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::CreateStream(CreateStream::default()),
            CREATE_STREAM_CODE,
            &CreateStream::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::DeleteStream(DeleteStream::default()),
            DELETE_STREAM_CODE,
            &DeleteStream::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::UpdateStream(UpdateStream::default()),
            UPDATE_STREAM_CODE,
            &UpdateStream::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::PurgeStream(PurgeStream::default()),
            PURGE_STREAM_CODE,
            &PurgeStream::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::GetTopic(GetTopic::default()),
            GET_TOPIC_CODE,
            &GetTopic::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::GetTopics(GetTopics::default()),
            GET_TOPICS_CODE,
            &GetTopics::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::CreateTopic(CreateTopic::default()),
            CREATE_TOPIC_CODE,
            &CreateTopic::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::DeleteTopic(DeleteTopic::default()),
            DELETE_TOPIC_CODE,
            &DeleteTopic::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::UpdateTopic(UpdateTopic::default()),
            UPDATE_TOPIC_CODE,
            &UpdateTopic::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::PurgeTopic(PurgeTopic::default()),
            PURGE_TOPIC_CODE,
            &PurgeTopic::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::CreatePartitions(CreatePartitions::default()),
            CREATE_PARTITIONS_CODE,
            &CreatePartitions::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::DeletePartitions(DeletePartitions::default()),
            DELETE_PARTITIONS_CODE,
            &DeletePartitions::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::GetConsumerGroup(GetConsumerGroup::default()),
            GET_CONSUMER_GROUP_CODE,
            &GetConsumerGroup::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::GetConsumerGroups(GetConsumerGroups::default()),
            GET_CONSUMER_GROUPS_CODE,
            &GetConsumerGroups::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::CreateConsumerGroup(CreateConsumerGroup::default()),
            CREATE_CONSUMER_GROUP_CODE,
            &CreateConsumerGroup::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::DeleteConsumerGroup(DeleteConsumerGroup::default()),
            DELETE_CONSUMER_GROUP_CODE,
            &DeleteConsumerGroup::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::JoinConsumerGroup(JoinConsumerGroup::default()),
            JOIN_CONSUMER_GROUP_CODE,
            &JoinConsumerGroup::default(),
        );
        assert_serialized_as_bytes_and_deserialized_from_bytes(
            &Command::LeaveConsumerGroup(LeaveConsumerGroup::default()),
            LEAVE_CONSUMER_GROUP_CODE,
            &LeaveConsumerGroup::default(),
        );
    }

    fn assert_serialized_as_bytes_and_deserialized_from_bytes(
        command: &Command,
        command_id: u32,
        payload: &dyn CommandPayload,
    ) {
        assert_serialized_as_bytes(command, command_id, payload);
        assert_deserialized_from_bytes(command, command_id, payload);
    }

    fn assert_serialized_as_bytes(
        command: &Command,
        command_id: u32,
        payload: &dyn CommandPayload,
    ) {
        let payload = payload.as_bytes();
        let mut bytes = BytesMut::with_capacity(4 + payload.len());
        bytes.put_u32_le(command_id);
        bytes.put_slice(&payload);
        assert_eq!(command.as_bytes(), bytes);
    }

    fn assert_deserialized_from_bytes(
        command: &Command,
        command_id: u32,
        payload: &dyn CommandPayload,
    ) {
        let payload = payload.as_bytes();
        let mut bytes = BytesMut::with_capacity(4 + payload.len());
        bytes.put_u32_le(command_id);
        bytes.put_slice(&payload);
        let bytes = Bytes::from(bytes);
        assert_eq!(&Command::from_bytes(bytes).unwrap(), command);
    }
}
