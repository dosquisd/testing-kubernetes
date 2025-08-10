from fastapi.testclient import TestClient

from src.crud import UserService
from src.tests.utils.users import create_random_user


def test_create_user(client: TestClient, user_service: UserService) -> None:
    """Test to create a new user"""
    user_data = create_random_user(user_service, False)
    response = client.post("/api/v1/users/", json=user_data.model_dump())

    assert response.status_code == 201
    data = response.json()
    assert data["email"] == user_data.email
    assert data["name"] == user_data.name
    assert data["age"] == user_data.age
    assert data["is_active"] is True


def test_read_user(client: TestClient, user_service: UserService) -> None:
    """Test to retrieve a user by ID"""
    user = create_random_user(user_service, True)
    response = client.get(f"/api/v1/users/{user.id}")

    assert response.status_code == 200
    data = response.json()
    assert data["id"] == user.id
    assert data["email"] == user.email
    assert data["name"] == user.name
    assert data["age"] == user.age
    assert data["is_active"] is True


def test_read_user_not_found(client: TestClient) -> None:
    """Test to retrieve a user that does not exist"""
    response = client.get("/api/v1/users/-1")
    assert response.status_code == 404
    assert response.json()["detail"] == "User not found"


def test_get_users(client: TestClient, user_service: UserService) -> None:
    """Test to retrieve a list of users"""
    # Create at least 5 users
    for _ in range(5):
        create_random_user(user_service, True)

    response = client.get("/api/v1/users/")
    assert response.status_code == 200
    data = response.json()
    assert isinstance(data, list)
    assert len(data) >= 5  # Make sure at least 5 users are returned


def test_update_user(client: TestClient, user_service: UserService) -> None:
    """Test to update an existing user"""
    user = create_random_user(user_service, True)
    update_data = {"name": "Updated Name", "age": 30}

    response = client.put(f"/api/v1/users/{user.id}", json=update_data)
    assert response.status_code == 200
    data = response.json()
    assert data["id"] == user.id
    assert data["name"] == update_data["name"]
    assert data["age"] == update_data["age"]


def test_update_user_not_found(client: TestClient) -> None:
    """Test to update a user that does not exist"""
    update_data = {"name": "Updated Name", "age": 30}
    response = client.put("/api/v1/users/-1", json=update_data)
    assert response.status_code == 404
    assert response.json()["detail"] == "User not found"


def test_delete_user(client: TestClient, user_service: UserService) -> None:
    """Test to delete an existing user"""
    user = create_random_user(user_service, True)

    response = client.delete(f"/api/v1/users/{user.id}")
    assert response.status_code == 204

    # Verify the user is actually deleted
    response = client.get(f"/api/v1/users/{user.id}")
    assert response.status_code == 404


def test_delete_user_not_found(client: TestClient) -> None:
    """Test to delete a user that does not exist"""
    response = client.delete("/api/v1/users/-1")
    assert response.status_code == 404
    assert response.json()["detail"] == "User not found"
