use crate::{bytes_serializable::BytesSerializable, command::HashableCommand};
use crate::command::CommandPayload;
use crate::error::IggyError;
use crate::identifier::Identifier;
use crate::validatable::Validatable;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// `GetUser` command is used to retrieve the information about a user by unique ID.
/// It has additional payload:
/// - `user_id` - unique user ID (numeric or name).
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct GetUser {
    #[serde(skip)]
    /// Unique user ID (numeric or name).
    pub user_id: Identifier,
}

impl CommandPayload for GetUser {}
impl HashableCommand for GetUser {
    fn hash(&self) -> Option<u32> {
        self.user_id.hash()
    }
}

impl Validatable<IggyError> for GetUser {
    fn validate(&self) -> Result<(), IggyError> {
        Ok(())
    }
}

impl BytesSerializable for GetUser {
    fn as_bytes(&self) -> Bytes {
        self.user_id.as_bytes()
    }

    fn from_bytes(bytes: Bytes) -> Result<GetUser, IggyError> {
        if bytes.len() < 3 {
            return Err(IggyError::InvalidCommand);
        }

        let user_id = Identifier::from_bytes(bytes)?;
        let command = GetUser { user_id };
        command.validate()?;
        Ok(command)
    }
}

impl Display for GetUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_serialized_as_bytes() {
        let command = GetUser {
            user_id: Identifier::numeric(1).unwrap(),
        };

        let bytes = command.as_bytes();
        let user_id = Identifier::from_bytes(bytes.clone()).unwrap();

        assert!(!bytes.is_empty());
        assert_eq!(user_id, command.user_id);
    }

    #[test]
    fn should_be_deserialized_from_bytes() {
        let user_id = Identifier::numeric(1).unwrap();
        let bytes = user_id.as_bytes();
        let command = GetUser::from_bytes(bytes);
        assert!(command.is_ok());

        let command = command.unwrap();
        assert_eq!(command.user_id, user_id);
    }
}
