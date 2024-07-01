DEFINE TABLE user SCHEMAFULL PERMISSIONS FOR select, update, delete WHERE id = $auth.id;

# #[guard(length(2..=25))]
DEFINE FIELD name ON user TYPE string ASSERT string::len($value) >= 2 AND string::len($value) <= 25;
# #[guard(url, length(max = 50))]
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value) AND string::len($value) <= 50;
# #[guard(length(7..=50))]
DEFINE FIELD password ON user TYPE string ASSERT string::len($value) >= 7 AND string::len($value) <= 50;

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