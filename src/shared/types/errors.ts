export const AppErrorTypes = {
  Database: "Database",
  NotFound: "NotFound",
  Validation: "Validation",
  Internal: "Internal",
  LockError: "LockError",
} as const;

type TypeAppError = (typeof AppErrorTypes)[keyof typeof AppErrorTypes];

export interface AppError {
  type: TypeAppError;
  message: string;
}

export const isAppError = (error: unknown): error is AppError => {
  return (
    typeof error === "object" &&
    error !== null &&
    "type" in error &&
    typeof error["type"] === "string" &&
    "message" in error &&
    (Object.values(AppErrorTypes) as string[]).includes(error.type)
  );
};
