from fastapi import FastAPI
from src.api.main import router as router
from fastapi.middleware.cors import CORSMiddleware
from src.api.middleware.logs import LogsMiddleware

from src.core.config import settings
from src.core.database import engine, Base


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
app.include_router(router)

# Incluir el middleware de logs
app.add_middleware(LogsMiddleware)


if __name__ == "__main__":
    import uvicorn

    uvicorn.run("src.main:app", host="0.0.0.0", port=8000, reload=settings.DEBUG)
