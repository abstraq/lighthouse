use thiserror::Error;

#[derive(Error, Debug)]
pub enum PermissionsError {
	/// Represents when the user is not permitted to perform an action.
	#[error("You are not allowed perform this action.")]
	Restricted,

	/// Represents when the user is missing one or more permissions in the
	/// current context.
	#[error("To perform this action, you must have the following permissions: {expected_permissions}.")]
	UserMissingPermissions { expected_permissions: String },

	/// Represents when the bot is missing one or more permissions in the
	/// current context.
	#[error("To perform this action, lighthouse must have the following permissions: {expected_permissions}.")]
	BotMissingPermissions { expected_permissions: String },
}
