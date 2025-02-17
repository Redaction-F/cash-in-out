-- Add down migration script here
DELETE FROM cash_record WHERE id=10000;
DELETE FROM cash_record WHERE id=10001;
DELETE FROM cash_record WHERE id=10002;