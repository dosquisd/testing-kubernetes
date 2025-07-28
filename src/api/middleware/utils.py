from src.core.config import settings
from questdb.ingress import Sender, Protocol, TimestampNanos


def send_logs_to_questdb(data: dict):
    with Sender(
        protocol=Protocol.Http,
        host=settings.QUESTDB_HOST,
        port=settings.QUESTDB_PORT,
        username=settings.QUESTDB_USER,
        password=settings.QUESTDB_PASSWORD,
    ) as sender:
        sender.row(
            "logs",
            columns=dict(map(lambda x: (x[0], str(x[1])), data.items())),
            at=TimestampNanos.now(),
        )
        sender.flush()
