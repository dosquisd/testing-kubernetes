from sqlalchemy.orm import Session
from sqlalchemy import or_
from typing import List, Optional
from src.models import User
from src.schemas import UserCreate, UserUpdate
from src.core.cache import redis_service


class UserService:
    def __init__(self, db: Session):
        self.db = db

    def get_user(self, user_id: int) -> Optional[User]:
        """Obtiene un usuario por ID con cache"""
        cache_key = f"user:{user_id}"

        # Intentar obtener del cache
        cached_user = redis_service.get(cache_key)
        if cached_user:
            return User(**cached_user)

        # Si no está en cache, obtener de la BD
        user = self.db.query(User).filter(User.id == user_id).first()
        if user:
            # Guardar en cache
            user_dict = {
                "id": user.id,
                "email": user.email,
                "name": user.name,
                "age": user.age,
                "is_active": user.is_active,
                "created_at": user.created_at,
                "updated_at": user.updated_at,
            }
            redis_service.set(cache_key, user_dict)

        return user

    def get_user_by_email(self, email: str) -> Optional[User]:
        """Obtiene un usuario por email"""
        return self.db.query(User).filter(User.email == email).first()

    def get_users(
        self, skip: int = 0, limit: int = 100, search: Optional[str] = None
    ) -> List[User]:
        """Obtiene una lista de usuarios con paginación y búsqueda opcional"""
        cache_key = f"users:skip:{skip}:limit:{limit}:search:{search or 'none'}"

        # Intentar obtener del cache
        cached_users = redis_service.get(cache_key)
        if cached_users:
            return [User(**user_data) for user_data in cached_users]

        # Construir query
        query = self.db.query(User)

        if search:
            query = query.filter(
                or_(User.name.ilike(f"%{search}%"), User.email.ilike(f"%{search}%"))
            )

        users = query.offset(skip).limit(limit).all()

        # Guardar en cache
        users_dict = []
        for user in users:
            users_dict.append(
                {
                    "id": user.id,
                    "email": user.email,
                    "name": user.name,
                    "age": user.age,
                    "is_active": user.is_active,
                    "created_at": user.created_at,
                    "updated_at": user.updated_at,
                }
            )
        redis_service.set(cache_key, users_dict, expire=180)  # Cache por 3 minutos

        return users

    def create_user(self, user: UserCreate) -> User:
        """Crea un nuevo usuario"""
        db_user = User(email=user.email, name=user.name, age=user.age)
        self.db.add(db_user)
        self.db.commit()
        self.db.refresh(db_user)

        # Invalidar cache de listas
        redis_service.delete_pattern("users:*")

        return db_user

    def update_user(self, user_id: int, user_update: UserUpdate) -> Optional[User]:
        """Actualiza un usuario existente"""
        db_user = self.db.query(User).filter(User.id == user_id).first()
        if not db_user:
            return None

        update_data = user_update.model_dump(exclude_unset=True)
        for field, value in update_data.items():
            setattr(db_user, field, value)

        self.db.commit()
        self.db.refresh(db_user)

        # Invalidar cache
        redis_service.delete(f"user:{user_id}")
        redis_service.delete_pattern("users:*")

        return db_user

    def delete_user(self, user_id: int) -> bool:
        """Elimina un usuario"""
        db_user = self.db.query(User).filter(User.id == user_id).first()
        if not db_user:
            return False

        self.db.delete(db_user)
        self.db.commit()

        # Invalidar cache
        redis_service.delete(f"user:{user_id}")
        redis_service.delete_pattern("users:*")

        return True
