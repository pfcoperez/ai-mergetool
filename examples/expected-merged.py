"""User authentication module."""

import hashlib
import logging

logger = logging.getLogger(__name__)

def authenticate(username, password):
    """Authenticate a user with username and password."""
    if not username or not password:
        logger.warning("Missing username or password")
        return False
    hashed = hashlib.sha256(password.encode()).hexdigest()
    result = check_credentials(username, hashed)
    logger.info(f"Auth attempt for {username}: {'success' if result else 'failure'}")
    return result

def check_credentials(username, password_hash):
    """Check credentials against the database using hashed password."""
    user = db.find_user(username)
    if user and user.password_hash == password_hash:
        return True
    return False

def logout(session_id):
    """Log out a user by invalidating their session."""
    logger.info(f"Logging out session {session_id}")
    db.delete_session(session_id)
    db.audit_log("logout", session_id)
