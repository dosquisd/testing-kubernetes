# TODO: Find a way to detect if the request and response data have binaries
from datetime import datetime
from time import perf_counter
from fastapi import Request, Response

from starlette.background import BackgroundTask
from starlette.middleware.base import BaseHTTPMiddleware
from src.api.middleware.utils import send_logs_to_questdb


class LogsMiddleware(BaseHTTPMiddleware):
    async def dispatch(self, request: Request, call_next):
        """Endpoint para registrar solicitudes entrantes"""
        scope = request.scope
        req_params = {
            "method": scope["method"],
            "req_headers": dict(map(lambda x: (x[0], x[1]), scope["headers"])),
            "http_version": scope["http_version"],
            "path": scope["path"],
            "scheme": scope["scheme"],
            "type": scope["type"],
            "path_params": scope["path_params"] if "path_params" in scope else {},
            "query_string": scope["query_string"],
            "server": scope["server"],
            "client": scope["client"],
            "body": await request.body() if False else b"",  # This meanwhile
        }

        created_at = datetime.now().isoformat()
        start_time = perf_counter()
        response: Response = await call_next(request)
        end_time = perf_counter()

        res_params = {
            "status_code": response.status_code,
            "process_time": end_time - start_time,
            "created_at": created_at,
            "res_headers": dict(map(lambda x: (x[0], x[1]), response.headers.items())),
            "response": response.body if False else b"",  # This meanwhile
        }

        results = {**req_params, **res_params}
        response.background = BackgroundTask(send_logs_to_questdb, results)

        return response
