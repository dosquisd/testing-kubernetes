from src.core.config import settings
from questdb.ingress import Sender, Protocol, TimestampNanos


def cast_to_questdb_type(value):
    if isinstance(value, (int, float, bool, str)):
        return value
    return str(value)


def send_logs_to_questdb(data: dict):
    with Sender(
        protocol=Protocol.Http,
        host=settings.QUESTDB_HOST,
        port=settings.QUESTDB_PORT,
        username=settings.QUESTDB_USER,
        password=settings.QUESTDB_PASSWORD,
    ) as sender:
        sender.row(
            settings.QUESTDB_DB,
            columns=dict(
                map(lambda x: (x[0], cast_to_questdb_type(x[1])), data.items())
            ),
            at=TimestampNanos.now(),
        )
        sender.flush()
