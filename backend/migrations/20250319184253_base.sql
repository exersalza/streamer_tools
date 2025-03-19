-- --------------------------------------------------------
-- Host:                         E:\development\rust\streamer_tools\backend\db.sqlite
-- Server version:               3.48.0
-- Server OS:                    
-- HeidiSQL Version:             12.10.0.7000
-- --------------------------------------------------------

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET NAMES  */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;


-- Dumping structure for table db.subathon_data
CREATE TABLE IF NOT EXISTS "subathon_data" (
	"id" VARCHAR(50) NOT NULL DEFAULT NULL, "total_donos" INTEGER NULL, "total_subs" INTEGER NULL, "total_bits" INTEGER NULL,
	PRIMARY KEY ("id")
);

-- Data exporting was unselected.

-- Dumping structure for table db.timer
CREATE TABLE IF NOT EXISTS "timer" (
	"id" VARCHAR(36) NOT NULL DEFAULT NULL,
	"name" VARCHAR(255) NOT NULL DEFAULT NULL,
	"time" BIGINT NULL DEFAULT NULL, 
  "main" TINYINT NULL DEFAULT '0', 
  "is_active" TINYINT NULL DEFAULT '0', 
  -- NOTE: this is for the timer icon, not the timer itself
  "color" VARCHAR(11) NULL DEFAULT '#000000', 
	PRIMARY KEY ("id")
);

-- Data exporting was unselected.

-- Dumping structure for table db.timer_go_down_by
CREATE TABLE IF NOT EXISTS "timer_go_down_by" (
	"id" VARCHAR(36) NOT NULL DEFAULT NULL,
	"follow" INTEGER NULL,
	"sub_t1" INTEGER NULL,
	"sub_t2" INTEGER NULL,
	"sub_t3" INTEGER NULL,
	"dono_each_n" INTEGER NULL,
	"dono_n" INTEGER NULL,
	"bits_each_n" INTEGER NULL,
	"bits_n" INTEGER NULL,
	PRIMARY KEY ("id"),
	CONSTRAINT "ID" FOREIGN KEY ("id") REFERENCES "timer" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION
);

-- Data exporting was unselected.

-- Dumping structure for table db.twitch_data
CREATE TABLE IF NOT EXISTS "twitch_data" (
	"id" INTEGER NOT NULL,
	"user_token" VARCHAR(255) NULL DEFAULT NULL, "user_refresh" VARCHAR(255) NULL DEFAULT NULL, "token_type" VARCHAR(50) NULL DEFAULT NULL, "expires_in" INTEGER NULL,
	PRIMARY KEY ("id")
);

-- Data exporting was unselected.

-- Dumping structure for table db.user_data
CREATE TABLE IF NOT EXISTS "user_data" (
	"username" VARCHAR(50) NULL DEFAULT NULL,
	"display_name" VARCHAR(50) NULL DEFAULT NULL,
	"profile_pic" VARCHAR(255) NULL DEFAULT NULL,
	"broadcaster_type" VARCHAR(255) NULL DEFAULT NULL
, "id" VARCHAR(50) NULL DEFAULT NULL);

-- Data exporting was unselected.

-- Dumping structure for table db._oauth
CREATE TABLE IF NOT EXISTS "_oauth" (
	"id" INTEGER NOT NULL, "token" VARCHAR(50) NULL, "expires_in" INTEGER NULL, "token_type" VARCHAR(50) NULL,
	PRIMARY KEY ("id")
);

-- Data exporting was unselected.

-- Dumping structure for table db.timer_customization
CREATE TABLE "timer_customization" (
	"id" VARCHAR(36) NOT NULL,
	"upper_text" VARCHAR(255) NULL,
	"lower_text" VARCHAR(255) NULL,
	"text_color" VARCHAR(50) NULL,
	"border" INTEGER NULL DEFAULT NULL,
	"border_color" VARCHAR(50) NULL DEFAULT NULL,
	"animate" TINYINT NULL,
	PRIMARY KEY ("id")
)
;

/*!40103 SET TIME_ZONE=IFNULL(@OLD_TIME_ZONE, 'system') */;
/*!40101 SET SQL_MODE=IFNULL(@OLD_SQL_MODE, '') */;
/*!40014 SET FOREIGN_KEY_CHECKS=IFNULL(@OLD_FOREIGN_KEY_CHECKS, 1) */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40111 SET SQL_NOTES=IFNULL(@OLD_SQL_NOTES, 1) */;
