DEFINE TABLE user SCHEMAFULL PERMISSIONS FOR select, update, delete WHERE id = $auth.id;

# name must be at least 2 characters long
DEFINE FIELD name ON user TYPE string ASSERT string::len($value) >= 2;
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value);
# password must be at least 7 characters long
DEFINE FIELD password ON user TYPE string ASSERT string::len($value) >= 7;

DEFINE INDEX email on user FIELDS email UNIQUE;

DEFINE SCOPE user SESSION 1d SIGNIN (
	SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(password, $password)
	SIGNUP (
		CREATE user CONTENT {
			name: $name,
			email: $email,
			password: crypto::argon2::generate($password)
		}
	)
)