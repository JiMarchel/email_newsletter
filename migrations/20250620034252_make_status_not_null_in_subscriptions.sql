-- We wrap the whole migration in a transaction to make sure
BEGIN;
    UPDATE subscriptions
        SET status = 'confirmed'
        WHERE status IS NULL;

    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;
