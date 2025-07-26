import redis
import json
from typing import Optional, Any
from .config import settings


class RedisService:
    def __init__(self):
        self.redis_client = redis.from_url(str(settings.REDIS_URI), decode_responses=True)

    def get(self, key: str) -> Optional[Any]:
        """Obtiene un valor del cache"""
        try:
            value = self.redis_client.get(key)
            if value:
                return json.loads(value)
            return None
        except Exception as e:
            print(f"Error obteniendo del cache: {e}")
            return None

    def set(self, key: str, value: Any, expire: int = 300) -> bool:
        """Guarda un valor en el cache con expiración en segundos (default: 5 minutos)"""
        try:
            json_value = json.dumps(value, default=str)
            return self.redis_client.setex(key, expire, json_value)
        except Exception as e:
            print(f"Error guardando en cache: {e}")
            return False

    def delete(self, key: str) -> bool:
        """Elimina una clave del cache"""
        try:
            return bool(self.redis_client.delete(key))
        except Exception as e:
            print(f"Error eliminando del cache: {e}")
            return False

    def delete_pattern(self, pattern: str) -> int:
        """Elimina todas las claves que coincidan con el patrón"""
        try:
            keys = self.redis_client.keys(pattern)
            if keys:
                return self.redis_client.delete(*keys)
            return 0
        except Exception as e:
            print(f"Error eliminando patrón del cache: {e}")
            return 0


# Instancia global del servicio Redis
redis_service = RedisService()
