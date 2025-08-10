import random
import string


def random_lower_string() -> str:
    return "".join(random.choices(string.ascii_lowercase, k=32))


def random_email() -> str:
    return f"{random_lower_string()}@{random_lower_string()}.com"


def random_int(min_value: int = 0, max_value: int = 100) -> int:
    return random.randint(min_value, max_value)
