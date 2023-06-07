-- Add migration script here

ALTER TABLE billable_aggregate ALTER min SET NOT NULL;
ALTER TABLE billable_aggregate ALTER max SET NOT NULL;
ALTER TABLE billable_aggregate ALTER avg SET NOT NULL;
ALTER TABLE billable_aggregate ALTER count SET NOT NULL;
ALTER TABLE billable_aggregate ALTER sum SET NOT NULL;