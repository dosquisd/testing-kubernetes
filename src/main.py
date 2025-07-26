from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from .routes import router as users_router
from sqlalchemy import text
from .database import engine, Base
from .config import settings
import redis

# Crear las tablas de la base de datos
Base.metadata.create_all(bind=engine)

app = FastAPI(
    title="User Management API",
    description="Una API REST sencilla para gestión de usuarios con PostgreSQL y Redis cache",
    version="1.0.0",
)

# Configurar CORS
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # En producción, especificar dominios específicos
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Incluir las rutas
app.include_router(users_router, prefix="/api/v1")


@app.get("/")
def read_root():
    return {
        "message": "User Management API",
        "version": "1.0.0",
        "docs": "/docs",
        "redoc": "/redoc",
    }


@app.get("/health")
def health_check():
    """Endpoint para verificar el estado de la aplicación y sus dependencias"""
    health_status = {"status": "healthy", "database": "unknown", "cache": "unknown"}

    try:
        # Verificar conexión a la base de datos
        from .database import SessionLocal

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


if __name__ == "__main__":
    import uvicorn

    uvicorn.run("src.main:app", host="0.0.0.0", port=8000, reload=settings.DEBUG)
