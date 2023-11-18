/*
  Warnings:

  - Added the required column `neighborhood` to the `places` table without a default value. This is not possible if the table is not empty.

*/
-- DropForeignKey
ALTER TABLE "events" DROP CONSTRAINT "events_category_id_fkey";

-- DropForeignKey
ALTER TABLE "events" DROP CONSTRAINT "events_place_id_fkey";

-- AlterTable
ALTER TABLE "events" ALTER COLUMN "description" DROP NOT NULL;

-- AlterTable
ALTER TABLE "places" ADD COLUMN     "neighborhood" TEXT NOT NULL;

-- AddForeignKey
ALTER TABLE "events" ADD CONSTRAINT "events_category_id_fkey" FOREIGN KEY ("category_id") REFERENCES "categories"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "events" ADD CONSTRAINT "events_place_id_fkey" FOREIGN KEY ("place_id") REFERENCES "places"("id") ON DELETE CASCADE ON UPDATE CASCADE;
