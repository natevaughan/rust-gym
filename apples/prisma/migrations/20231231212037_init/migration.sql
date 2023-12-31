-- CreateTable
CREATE TABLE "Apple" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "varietyName" TEXT NOT NULL,
    "color" TEXT NOT NULL DEFAULT 'MIXED'
);

-- CreateTable
CREATE TABLE "AppleCount" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "count" INTEGER NOT NULL DEFAULT 0,
    "appleId" TEXT NOT NULL,
    CONSTRAINT "AppleCount_appleId_fkey" FOREIGN KEY ("appleId") REFERENCES "Apple" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
