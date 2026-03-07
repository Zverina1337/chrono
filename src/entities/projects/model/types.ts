import * as z from "zod";
import { ProjectSchema } from "./validation";

export type IProject = z.infer<typeof ProjectSchema>;
export enum IProjectCommands {
  CREATE = "create_project",
  READ = "get_all_projects",
  UPDATE = "update",
  DELETE = "delete_project",
}
