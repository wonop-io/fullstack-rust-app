CREATE TABLE IF NOT EXISTS
    "auth_passwords_passwords" (
        user_id UUID NOT NULL PRIMARY KEY,
        password VARCHAR(255) NOT NULL,
        FOREIGN KEY (user_id) REFERENCES auth_users(id) ON DELETE CASCADE
    );
