/*
  Warnings:

  - You are about to drop the column `pub_id` on the `instance` table. All the data in the column will be lost.

*/
-- CreateTable
CREATE TABLE "location" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "pub_id" BLOB NOT NULL,
    "node_id" INTEGER NOT NULL,
    "name" TEXT,
    "local_path" TEXT,
    "is_online" BOOLEAN NOT NULL DEFAULT true,
    "is_archived" BOOLEAN NOT NULL DEFAULT false,
    "date_created" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "location_node_id_fkey" FOREIGN KEY ("node_id") REFERENCES "node" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_instance" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "node_id" INTEGER NOT NULL,
    "name" TEXT,
    "mc_path" TEXT,
    "mc_version" TEXT,
    "mc_platform" INTEGER,
    "shared" BLOB
);
INSERT INTO "new_instance" ("id", "mc_path", "mc_platform", "mc_version", "name", "node_id", "shared") SELECT "id", "mc_path", "mc_platform", "mc_version", "name", "node_id", "shared" FROM "instance";
DROP TABLE "instance";
ALTER TABLE "new_instance" RENAME TO "instance";
CREATE UNIQUE INDEX "instance_node_id_name_key" ON "instance"("node_id", "name");
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;

-- CreateIndex
CREATE UNIQUE INDEX "location_pub_id_key" ON "location"("pub_id");
