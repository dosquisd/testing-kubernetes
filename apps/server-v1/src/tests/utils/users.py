from src.crud import UserService
from src.models import User
from src.schemas import UserCreate
from src.tests.utils.utils import random_email, random_lower_string, random_int


def create_random_user(
    user_service: UserService, submit_db: bool = False
) -> UserCreate | User:
    """Crea un usuario aleatorio para pruebas"""
    user = UserCreate(
        email=random_email(),
        name=random_lower_string(),
        age=random_int(18, 65),
    )
    if submit_db:
        return user_service.create_user(user)
    return user
