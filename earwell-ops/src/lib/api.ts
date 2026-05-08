import { invoke } from "@tauri-apps/api/core";
import type { AppStatus, Session, SetupArgs, UnlockArgs, LoginArgs, CreateUserArgs } from "@/lib/types";

export const api = {
  appStatus:         () => invoke<AppStatus>("app_status"),
  setup:             (args: SetupArgs)  => invoke<AppStatus>("setup",  { args }),
  unlock:            (args: UnlockArgs) => invoke<AppStatus>("unlock", { args }),
  lock:              () => invoke<AppStatus>("lock"),
  auditVerifyChain:  () => invoke<[number, number | null]>("audit_verify_chain"),
  login:             (args: LoginArgs)  => invoke<Session>("login", { args }),
  logout:            () => invoke<void>("logout"),
  currentSession:    () => invoke<Session | null>("current_session"),
  createUser:        (args: CreateUserArgs) => invoke<void>("create_user", { args }),
};
