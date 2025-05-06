-- Add migration script here

CREATE IF NOT EXISTS TABLE "user_data" (
	"username" VARCHAR(50) NULL DEFAULT NULL,
	"display_name" VARCHAR(50) NULL DEFAULT NULL,
	"profile_pic" VARCHAR(255) NULL DEFAULT NULL,
	"broadcaster_type" VARCHAR(255) NULL DEFAULT NULL
, "id" VARCHAR(50) NULL DEFAULT NULL)

CREATE IF NOT EXISTS TABLE "twitch_data" (
	"id" INTEGER NOT NULL,
	"user_token" VARCHAR(255) NULL DEFAULT NULL, 
  "user_refresh" VARCHAR(255) NULL DEFAULT NULL, 
  "token_type" VARCHAR(50) NULL DEFAULT NULL, 
  "expires_in" INTEGER NULL,
	PRIMARY KEY ("id")
)

CREATE IF NOT EXISTS TABLE "timer_go_down_by" (
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
)

CREATE IF NOT EXISTS TABLE "timer_customization" (
	"id" VARCHAR(36) NOT NULL,
	"upper_text" VARCHAR(255) NULL,
	"lower_text" VARCHAR(255) NULL,
	"text_color" VARCHAR(50) NULL,
	"border" INTEGER NULL DEFAULT NULL,
	"border_color" VARCHAR(50) NULL DEFAULT NULL,
	"animate" TINYINT NULL,
	PRIMARY KEY ("id")
)

CREATE IF NOT EXISTS TABLE "timer" (
	"id"	VARCHAR(36) NOT NULL DEFAULT NULL,
	"name"	VARCHAR(255) NOT NULL DEFAULT NULL,
	"time"	BIGINT DEFAULT NULL,
	"main"	TINYINT DEFAULT '0',
	"is_active"	TINYINT DEFAULT '0',
	"color"	VARCHAR(11) DEFAULT '#000000',
	"type"	INTEGER DEFAULT 0,
	PRIMARY KEY("id")
)

CREATE IF NOT EXISTS TABLE "subathon_data" (
	"id" VARCHAR(50) NOT NULL DEFAULT NULL, 
  "total_donos" INTEGER NULL, 
  "total_subs" INTEGER NULL, 
  "total_bits" INTEGER NULL,
	PRIMARY KEY ("id")
)


CREATE IF NOT EXISTS TABLE "history" (
	"id"	INTEGER,
	"uuid"	TEXT,
	"event_type"	TEXT,
	"amount"	INTEGER DEFAULT 0,
	"user"	TEXT,
	"extra"	TEXT,
	"timestamp"	INTEGER,
	PRIMARY KEY("id" AUTOINCREMENT)
)

CREATE TABLE "settings" (
	"id"	INTEGER DEFAULT 1 UNIQUE,
	"show_emotes"	INTEGER DEFAULT 0
)
