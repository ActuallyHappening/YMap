DEFINE NAMESPACE production;
USE NS production;

DEFINE DATABASE production;
USE DB production;

DEFINE TABLE user SCHEMAFULL PERMISSIONS FOR select, update, delete WHERE id = $auth.id;

-- DEFINE FIELD username ON user TYPE string;
DEFINE FIELD username ON user TYPE string ASSERT string::len($value) >= 2 AND string::len($value) <= 25;
-- DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value);
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value) AND string::len($value) <= 50;
DEFINE FIELD password ON user TYPE string; # stores a HASH not the actual password

DEFINE INDEX email on user FIELDS email UNIQUE;

DEFINE FUNCTION fn::signin($email: string, $password: string) {
	LET $user = SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(password, $password);
	if !$user {
		THROW "You either provided invalid credentials, or a user with the email " + <string> $email + " doesn't exist.";
	};
	-- This doesn't actually break control flow, but here for clarity
	RETURN $user;
};

DEFINE FUNCTION fn::signup($username: string, $email: string, $password: string) {
	IF string::len($password) < 7 {
		THROW "Password must be at least 7 characters long.";
	};
	IF string::len($email) > 50 {
		THROW "Email must be at most 50 characters long.";
	};
	IF string::len($username) < 2 OR string::len($username) > 25 {
		THROW "Username must be at least 2 characters long and at most 25 characters long.";
	};
	let $user = CREATE user CONTENT {
		username: $username,
		email: $email,
		password: crypto::argon2::generate($password)
	};
	RETURN $user;
};

DEFINE SCOPE end_user
	SESSION 1d
	SIGNIN {
		RETURN fn::signin($email, $password);
	}
	SIGNUP {
		RETURN fn::signup($username, $email, $password);
	};