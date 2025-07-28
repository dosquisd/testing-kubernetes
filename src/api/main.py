from fastapi import APIRouter
from src.api.routes.users import router as users_router

import redis
from sqlalchemy import text
from src.core.config import settings
from src.core.database import SessionLocal


router = APIRouter()

# Incluir las rutas
router.include_router(users_router, prefix="/api/v1")


@router.get("/")
def read_root():
    return {
        "message": "User Management API",
        "version": "1.0.0",
        "docs": "/docs",
        "redoc": "/redoc",
    }


@router.get("/health")
def health_check():
    """Endpoint para verificar el estado de la aplicación y sus dependencias"""
    health_status = {"status": "healthy", "database": "unknown", "cache": "unknown"}

    try:
        db = SessionLocal()
        db.execute(text("SELECT 1"))
        db.close()
        health_status["database"] = "connected"
    except Exception as e:
        health_status["database"] = f"error: {str(e)}"
        health_status["status"] = "unhealthy"

    try:
        # Verificar conexión a Redis
        redis_client = redis.from_url(str(settings.REDIS_URI))
        redis_client.ping()
        health_status["cache"] = "connected"
    except Exception as e:
        health_status["cache"] = f"error: {str(e)}"
        health_status["status"] = "unhealthy"

    return health_status
