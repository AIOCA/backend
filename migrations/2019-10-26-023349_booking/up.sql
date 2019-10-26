CREATE TABLE bookings(
    booking_id INTEGER PRIMARY KEY,
    user_id INTEGER,
    FOREIGN KEY(user_id) REFERENCES users(user_id)
)