import * as z from "zod";
import { ProjectSchema, CreateProjectSchema } from "./validation";

export type IProject = z.infer<typeof ProjectSchema>;
export type IProjectCreate = z.infer<typeof CreateProjectSchema>;
export type IProjectUpdate = Partial<IProjectCreate>;

export enum IProjectCommands {
  CREATE = "create_project",
  READ = "get_all_projects",
  UPDATE = "update",
  DELETE = "delete_project",
}

export type IProjectActions = {
  fetchProjects: () => void;
  createProject: (data: IProjectCreate) => void;
  updateProject: (uuid: IProject["uuid"], data: IProjectUpdate) => void;
  deleteProject: (uuid: IProject["uuid"]) => void;
};
