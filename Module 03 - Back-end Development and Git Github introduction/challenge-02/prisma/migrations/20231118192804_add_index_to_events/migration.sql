-- CreateIndex
CREATE INDEX "events_name_description_id_category_id_place_id_idx" ON "events"("name", "description", "id", "category_id", "place_id");
