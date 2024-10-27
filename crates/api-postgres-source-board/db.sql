CREATE TABLE IF NOT EXISTS "osb_session" (
	"id" text PRIMARY KEY NOT NULL,
	"user_id" text NOT NULL,
	"expires_at" timestamp with time zone NOT NULL,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "osb_token" (
	"id" text PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"user_id" text NOT NULL,
	"access_token" text NOT NULL,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "osb_user" (
	"id" text PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"github_id" text NOT NULL,
	"username" text NOT NULL,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "osb_user_custom_config" (
	"id" text PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"user_id" text NOT NULL,
	"repo_name" text NOT NULL,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "osb_session" ADD CONSTRAINT "osb_session_user_id_osb_user_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."osb_user"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "osb_token" ADD CONSTRAINT "osb_token_user_id_osb_user_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."osb_user"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "osb_user_custom_config" ADD CONSTRAINT "osb_user_custom_config_user_id_osb_user_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."osb_user"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
CREATE UNIQUE INDEX IF NOT EXISTS "unique_username_githubId" ON "osb_user" USING btree ("github_id","username");--> statement-breakpoint
CREATE UNIQUE INDEX IF NOT EXISTS "unique_user_repo" ON "osb_user_custom_config" USING btree ("user_id","repo_name");