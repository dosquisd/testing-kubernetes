from collections.abc import Generator

import pytest
from sqlalchemy.orm import Session
from fastapi.testclient import TestClient

from src.main import app
from src.crud import UserService
from src.core.database import engine


@pytest.fixture(scope="session", autouse=True)
def db() -> Generator[Session, None, None]:
    with Session(bind=engine) as db:
        try:
            yield db
        finally:
            db.close()


@pytest.fixture(scope="module")
def client() -> Generator[TestClient, None, None]:
    with TestClient(app) as client:
        yield client


@pytest.fixture(scope="module")
def user_service(db: Session) -> UserService:
    return UserService(db)
