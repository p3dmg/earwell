export type Role = "admin" | "bookkeeper" | "partner_view";
export type AppPhase = "needs_setup" | "locked" | "unlocked" | "active";

export interface Session {
  user_id: string;
  username: string;
  full_name: string;
  role: Role;
  started_at: string;
}
export interface AppStatus {
  phase: AppPhase;
  session: Session | null;
  db_path: string;
  user_count: number | null;
}
export interface SetupArgs   { passphrase: string; admin_username: string; admin_full_name: string; admin_password: string; }
export interface UnlockArgs  { passphrase: string; }
export interface LoginArgs   { username: string; password: string; }
export interface CreateUserArgs { username: string; full_name: string; role: Role; password: string; }
