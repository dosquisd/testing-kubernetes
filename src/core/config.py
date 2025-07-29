import secrets
from dotenv import load_dotenv

from typing import Optional
from pydantic_core import MultiHostUrl
from pydantic import PostgresDsn, RedisDsn, computed_field
from pydantic_settings import BaseSettings, SettingsConfigDict

load_dotenv()


class Settings(BaseSettings):
    model_config: SettingsConfigDict = SettingsConfigDict(
        env_file=".env",
        extra="ignore",
        env_ignore_empty=True,
        env_file_encoding="utf-8",
    )

    # PostgreSQL configuration
    POSTGRES_USER: str
    POSTGRES_PASSWORD: str
    POSTGRES_HOST: str = "localhost"
    POSTGRES_PORT: int = 5432
    POSTGRES_DB: str = "api_test"

    @computed_field
    @property
    def POSTGRES_URI(self) -> PostgresDsn:
        return MultiHostUrl.build(
            scheme="postgresql+psycopg2",
            username=self.POSTGRES_USER,
            password=self.POSTGRES_PASSWORD,
            host=self.POSTGRES_HOST,
            port=self.POSTGRES_PORT,
            path=self.POSTGRES_DB,
        )

    # Redis configuration
    REDIS_HOST: str = "localhost"
    REDIS_PORT: int = 6379
    REDIS_PASSWORD: str

    @computed_field
    @property
    def REDIS_URI(self) -> RedisDsn:
        return MultiHostUrl.build(
            scheme="redis",
            host=self.REDIS_HOST,
            port=self.REDIS_PORT,
            password=self.REDIS_PASSWORD,
        )

    # QuestDB configuration
    QUESTDB_HOST: str = "localhost"
    QUESTDB_PORT: int = 9000
    QUESTDB_USER: Optional[str] = None
    QUESTDB_PASSWORD: Optional[str] = None
    QUESTDB_PG_PORT: int = 8812
    QUESTDB_DB: str = "logs"

    # Server configuration
    SECRET_KEY: str = secrets.token_urlsafe(32)
    DEBUG: bool = False


settings = Settings()
