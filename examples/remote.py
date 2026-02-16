"""User authentication module."""

import hashlib

def authenticate(username, password):
    """Authenticate a user with username and password."""
    if not username or not password:
        return False
    hashed = hashlib.sha256(password.encode()).hexdigest()
    return check_credentials(username, hashed)

def check_credentials(username, password_hash):
    """Check credentials against the database using hashed password."""
    user = db.find_user(username)
    if user and user.password_hash == password_hash:
        return True
    return False

def logout(session_id):
    """Log out a user by invalidating their session."""
    db.delete_session(session_id)
    db.audit_log("logout", session_id)
