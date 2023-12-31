/*
  Warnings:

  - You are about to drop the `AppleCount` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropTable
PRAGMA foreign_keys=off;
DROP TABLE "AppleCount";
PRAGMA foreign_keys=on;

-- CreateTable
CREATE TABLE "Inventory" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "count" INTEGER NOT NULL DEFAULT 0,
    "appleId" TEXT NOT NULL,
    CONSTRAINT "Inventory_appleId_fkey" FOREIGN KEY ("appleId") REFERENCES "Apple" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
