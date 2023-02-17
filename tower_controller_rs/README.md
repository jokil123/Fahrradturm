Action of tower should be controlled by database messages and button input

**All requests will fail if they are malformed or have permission issues**

# Store

- User requests to store object ->
  - Confirm message + (temporary slot reservation for user)
  - No storage
  - Error
- Wait for user to load bicycle & confirm with button press & wait for robot ->
  - Confirm Sucessful Storage + Database Update + (Remove as Target)
  - Timeout
  - Error

# Retrieve

- User requests to retrieve object ->
  - Confirm Message + Unload object from tower
  - No permission / Object not found
  - Error
- Wait for user to retrieve bike by pressing button ->
  - Confirm Sucessful Retrieval + Database Update + (Remove as Target)
  - Timeout
  - Error

# Tower Struct

The tower struct should keep all necessary data for operation and be synced with the database

# Display

The display should visualize the state of the tower and be in sync with it. It should either work with LEDS or with a GUI application, a common trait will therefore be required

# Some Stuff

- Errors and Confirmations have to be set in multiple places
- Multiple conditions have to be met for a message to pass, depending on the message different conditions must be met
