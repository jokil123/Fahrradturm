class QueueMessage:
    pass


class ConsoleInput(QueueMessage):
    def __init__(self, input: str):
        self.input = input


# class FirebaseUpdate(QueueMessage):
#     def __init__(self, snapshot, changes, read_time):
#         self.snapshot = snapshot
#         self.changes = changes
#         self.read_time = read_time


class StoreBicycle(QueueMessage):
    def __init__(self, user):
        self.user = user


class RetrieveBicycle(QueueMessage):
    def __init__(self, rental, box):
        self.rental = rental
        self.box = box
