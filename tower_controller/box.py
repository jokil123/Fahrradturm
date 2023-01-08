from logistic_state import LogisticState


class Box:
    is_empty: bool
    rented_by: str | None
    logistic_state: LogisticState

    def __init__(self) -> None:
        self. is_empty = True
        self.rented_by = None
        self.logistic_state = LogisticState.STORED
