fn recv(m: db_str, db: Db) {
    let Ok(message) = m.parse() else {
        db.set_err("Invalid message");
        return;
    };

    let Ok(()) = db.check_permissions(message) else {
        db.set_err("Invalid permissions");
        return;
    };

    db.send_confirm();

    match message.m_type {
        "store" => {
            tower.check_free_slot()
            // tower.reserve_slot();
            if err {
                db.set_err("No free slots");
                return;
            }

            tower.store(message).await?;
        }
        "retrieve" => {
            tower.slot_rented_by(message.user);
            if err {
                db.set_err("slot not rented by user");
                return;
            }

            tower.retrieve(message).await?;
        }
    }

    db.send_confirm();
}
