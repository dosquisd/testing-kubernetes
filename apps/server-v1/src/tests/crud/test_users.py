from src.models import User
from src.schemas import UserCreate, UserUpdate

from src.crud import UserService
from src.tests.utils.users import create_random_user
from src.tests.utils.utils import random_email, random_lower_string, random_int


def test_create_user(user_service: UserService) -> User:
    """Helper function to create a random user for testing"""
    name = random_lower_string()
    email = random_email()
    age = random_int(18, 65)
    user_create = UserCreate(email=email, name=name, age=age)
    user = user_service.create_user(user_create)

    assert user.email == email
    assert user.name == name
    assert user.age == age
    assert user.is_active is True
    assert user.created_at is not None
    assert user.updated_at is None


def test_get_user(user_service: UserService) -> None:
    """Test retrieving a user by ID"""
    user = create_random_user(user_service, True)
    retrieved_user = user_service.get_user(user.id)
    assert retrieved_user is not None
    assert retrieved_user.id == user.id
    assert retrieved_user.email == user.email
    assert retrieved_user.name == user.name
    assert retrieved_user.age == user.age
    assert retrieved_user.is_active == user.is_active


def test_get_user_not_found(user_service: UserService) -> None:
    """Test retrieving a non-existent user"""
    user_id = -1
    retrieved_user = user_service.get_user(user_id)
    assert retrieved_user is None


def test_get_user_by_email(user_service: UserService) -> None:
    """Test retrieving a user by email"""
    user = create_random_user(user_service, True)
    retrieved_user = user_service.get_user_by_email(user.email)
    assert retrieved_user is not None
    assert retrieved_user.id == user.id
    assert retrieved_user.email == user.email
    assert retrieved_user.name == user.name
    assert retrieved_user.age == user.age
    assert retrieved_user.is_active == user.is_active


def test_get_user_by_email_not_found(user_service: UserService) -> None:
    """Test retrieving a user by email that does not exist"""
    email = random_email()
    retrieved_user = user_service.get_user_by_email(email)
    assert retrieved_user is None


def test_get_users(user_service: UserService) -> None:
    """Test retrieving a list of users with pagination and search"""
    for _ in range(5):
        create_random_user(user_service, True)

    users = user_service.get_users(skip=0, limit=10)
    assert len(users) >= 5

    # Test search functionality
    search_name = users[0].name
    searched_users = user_service.get_users(skip=0, limit=10, search=search_name)
    assert len(searched_users) > 0
    assert all(search_name in user.name for user in searched_users)


def test_update_user(user_service: UserService) -> None:
    """Test updating an existing user"""
    user = create_random_user(user_service, True)
    new_name = random_lower_string()
    new_email = random_email()
    user_update = UserUpdate(name=new_name, email=new_email)

    updated_user = user_service.update_user(user.id, user_update)
    assert updated_user is not None
    assert updated_user.id == user.id
    assert updated_user.name == new_name
    assert updated_user.email == new_email


def test_update_user_not_found(user_service: UserService) -> None:
    """Test updating a non-existent user"""
    user_id = -1
    user_update = UserUpdate(name=random_lower_string(), email=random_email())
    updated_user = user_service.update_user(user_id, user_update)
    assert updated_user is None
