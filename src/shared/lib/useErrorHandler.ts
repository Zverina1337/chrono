import { AppErrorTypes } from "../types/errors";
import { isAppError } from "@/shared/types/errors";

export const useErrorHandler = () => {
  const ApiError = (type: string) => (message: string) => (info: string) => {
    console.error(`[${type}] - ${message} \n ${info}`);
  };

  const DatabaseError = ApiError(AppErrorTypes.Database);
  const NotFoundError = ApiError(AppErrorTypes.NotFound);
  const ValidationError = ApiError(AppErrorTypes.Validation);
  const InternalError = ApiError(AppErrorTypes.Internal);
  const LockError = ApiError(AppErrorTypes.LockError);

  const handleError = (error: unknown) => {
    if (isAppError(error)) {
      switch (error.type) {
        case "NotFound":
          NotFoundError("Уведомление: проекты не найдеры");
          break;
        case "Validation":
          ValidationError("Уведомление: проблема с валидацией");
          break;
        case "Database":
          DatabaseError("Что-то пошло не так");
          break;
        case "Internal":
          InternalError("Что-то пошло не так");
          break;
        case "LockError":
          LockError("Что-то пошло не так");
          break;
      }
    } else {
      return error;
    }
  };

  return { handleError };
};
