class QueueMessage:
    pass


class ConsoleInput(QueueMessage):
    def __init__(self, input: str):
        self.input = input


class FirebaseUpdate(QueueMessage):
    def __init__(self, doc_id: str, data: dict):
        self.doc_id = doc_id
        self.data = data
