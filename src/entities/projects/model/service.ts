import { invoke } from "@tauri-apps/api/core";
import { IProject, IProjectCommands, IProjectCreate, IProjectUpdate } from "./types";
import { useErrorHandler } from "@/shared/lib/useErrorHandler";

const { handleError } = useErrorHandler();
const getProjects = async (): Promise<IProject[] | []> => {
  try {
    const result = await invoke<IProject[]>(IProjectCommands.READ);
    return result;
  } catch (error: unknown) {
    return handleError(error, []);
  }
};
const createProject = async (data: IProjectCreate): Promise<IProject | undefined> => {
  try {
    const result = await invoke<IProject>(IProjectCommands.CREATE, { data });
    return result;
  } catch (error) {
    return handleError(error, undefined);
  }
};
const deleteProject = async (uuid: IProject["uuid"]): Promise<null | undefined> => {
  try {
    const result = await invoke<null>(IProjectCommands.DELETE, { uuid });
    return result;
  } catch (error) {
    return handleError(error, undefined);
  }
};
const updateProject = async (
  uuid: IProject["uuid"],
  data: IProjectUpdate,
): Promise<IProject | undefined> => {
  try {
    const result = await invoke<IProject>(IProjectCommands.UPDATE, { uuid, data });
    return result;
  } catch (error) {
    return handleError(error, undefined);
  }
};
export const ProjectService = { getProjects, createProject, deleteProject, updateProject };
