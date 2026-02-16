"""User authentication module."""

import logging

logger = logging.getLogger(__name__)

def authenticate(username, password):
    """Authenticate a user with username and password."""
    if not username or not password:
        logger.warning("Missing username or password")
        return False
    result = check_credentials(username, password)
    logger.info(f"Auth attempt for {username}: {'success' if result else 'failure'}")
    return result

def check_credentials(username, password):
    """Check credentials against the database."""
    user = db.find_user(username)
    if user and user.password == password:
        return True
    return False

def logout(session_id):
    """Log out a user by invalidating their session."""
    logger.info(f"Logging out session {session_id}")
    db.delete_session(session_id)
