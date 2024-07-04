DEFINE NAMESPACE production;
USE NS production;

DEFINE DATABASE production;
USE DB production;

DEFINE TABLE user SCHEMAFULL PERMISSIONS FOR select, update, delete WHERE id = $auth.id;

-- DEFINE FIELD username ON user TYPE string;
DEFINE FIELD username ON user TYPE string ASSERT string::len($value) >= 2 AND string::len($value) <= 25;
-- DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value);
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value) AND string::len($value) <= 50;
DEFINE FIELD password ON user TYPE string; # stores an HASH not the actual password

DEFINE INDEX email on user FIELDS email UNIQUE;

DEFINE SCOPE end_user
	SESSION 1d
	SIGNIN {
		LET $email_input = type::string($email);
		LET $password_input = type::string($password);
		LET $user = SELECT * FROM user WHERE email = $email_input AND crypto::argon2::compare(password, $password_input);
		if !$user {
			THROW "You either provided invalid credentials, or a user with the email " + <string> $email + " doesn't exist.";
		};
	}
	SIGNUP {
		LET $password_input = type::string($password);
		IF string::len($password_input) < 7 {
			THROW "Password must be at least 7 characters long.";
		};
		LET $email_input = type::string($email);
		IF string::len($email_input) > 50 {
			THROW "Email must be at most 50 characters long.";
		};
		LET $username_input = type::string($username);
		IF string::len($username_input) < 2 OR string::len($username_input) > 25 {
			THROW "Username must be at least 2 characters long and at most 25 characters long.";
		};
		CREATE user CONTENT {
			username: $username_input,
			email: $email_input,
			password: crypto::argon2::generate($password_input)
		}
	};