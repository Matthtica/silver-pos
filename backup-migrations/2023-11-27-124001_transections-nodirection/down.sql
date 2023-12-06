-- This file should undo anything in `up.sql`
ALTER TABLE transections ADD COLUMN direction BOOLEAN NOT NULL DEFAULT FALSE;
