from dataclasses import dataclass
import datetime
from enum import Enum
from typing import TypedDict

from google.cloud.firestore import GeoPoint
from google.cloud.firestore import DocumentReference


@dataclass
class Tower:
    id: str
    name: str
    location: GeoPoint


class BoxType(Enum):
    BICYCLE = "bicycle"
    STORAGE = "storage"


@dataclass
class Box:
    id: str
    isPowered: bool
    type: BoxType
    level: int
    index: int
    pass


@dataclass
class User:
    id: str


Tags = TypedDict("Tags", {"key": str, "value": str})


@dataclass
class Rental:
    id: str
    box: DocumentReference
    start: datetime.datetime | None
    end: datetime.datetime | None
    tags: Tags


@dataclass
class Key:
    id: str
    start: datetime.datetime | None
    end: datetime.datetime | None
    storeCount: int | None
    retrieveCount: int | None
    pass
