"""User authentication module."""

def authenticate(username, password):
    """Authenticate a user with username and password."""
    if not username or not password:
        return False
    return check_credentials(username, password)

def check_credentials(username, password):
    """Check credentials against the database."""
    user = db.find_user(username)
    if user and user.password == password:
        return True
    return False

def logout(session_id):
    """Log out a user by invalidating their session."""
    db.delete_session(session_id)
