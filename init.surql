DEFINE NAMESPACE production;
USE NS production;

DEFINE DATABASE production;
USE DB production;

-- DEFINE TABLE user SCHEMAFULL PERMISSIONS FOR select, update, delete WHERE id = $auth.id;
DEFINE TABLE user SCHEMAFULL;

DEFINE FIELD username ON user TYPE string ASSERT string::len($value) >= 2 AND string::len($value) <= 25;
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value) AND string::len($value) <= 50;
DEFINE FIELD password ON user TYPE string; # stores an HASH not the actual password

DEFINE INDEX email on user FIELDS email UNIQUE;

DEFINE SCOPE user
	SESSION 1d
	SIGNIN (
		SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(password, $password)
	)
	SIGNUP (
		CREATE user CONTENT {
			username: $username,
			email: $email,
			password: crypto::argon2::generate($password)
		}
	);