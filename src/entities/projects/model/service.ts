import { invoke } from "@tauri-apps/api/core";
import { IProject, IProjectCommands } from "./types";
import { useErrorHandler } from "@/shared/lib/useErrorHandler";

const { handleError } = useErrorHandler();
export const getProjects = async () => {
  try {
    const result = await invoke<IProject[]>(IProjectCommands.READ);
    return result;
  } catch (error: unknown) {
    handleError(error);
  }
};
