from fastapi import APIRouter, Depends, HTTPException, Query
from sqlalchemy.orm import Session
from typing import List, Optional

from src.crud import UserService
from src.core.database import get_db
from src.schemas import UserResponse, UserCreate, UserUpdate

router = APIRouter(prefix="/users", tags=["users"])


@router.post("/", response_model=UserResponse, status_code=201)
def create_user(user: UserCreate, db: Session = Depends(get_db)):
    """Crea un nuevo usuario"""
    user_service = UserService(db)

    # Verificar si el email ya existe
    if user_service.get_user_by_email(user.email):
        raise HTTPException(status_code=400, detail="Email already registered")

    return user_service.create_user(user)


@router.get("/", response_model=List[UserResponse])
def get_users(
    skip: int = Query(0, ge=0, description="Número de registros a omitir"),
    limit: int = Query(
        100, ge=1, le=100, description="Número máximo de registros a retornar"
    ),
    search: Optional[str] = Query(None, description="Buscar por nombre o email"),
    db: Session = Depends(get_db),
):
    """Obtiene una lista de usuarios con paginación y búsqueda opcional"""
    user_service = UserService(db)
    return user_service.get_users(skip=skip, limit=limit, search=search)


@router.get("/{user_id}", response_model=UserResponse)
def get_user(user_id: int, db: Session = Depends(get_db)):
    """Obtiene un usuario por ID"""
    user_service = UserService(db)
    user = user_service.get_user(user_id)

    if not user:
        raise HTTPException(status_code=404, detail="User not found")

    return user


@router.put("/{user_id}", response_model=UserResponse)
def update_user(user_id: int, user_update: UserUpdate, db: Session = Depends(get_db)):
    """Actualiza un usuario existente"""
    user_service = UserService(db)

    # Verificar si el email ya existe en otro usuario
    if user_update.email:
        existing_user = user_service.get_user_by_email(user_update.email)
        if existing_user and existing_user.id != user_id:
            raise HTTPException(status_code=400, detail="Email already registered")

    user = user_service.update_user(user_id, user_update)
    if not user:
        raise HTTPException(status_code=404, detail="User not found")

    return user


@router.delete("/{user_id}", status_code=204)
def delete_user(user_id: int, db: Session = Depends(get_db)):
    """Elimina un usuario"""
    user_service = UserService(db)

    if not user_service.delete_user(user_id):
        raise HTTPException(status_code=404, detail="User not found")

    return None
