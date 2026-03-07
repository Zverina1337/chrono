import * as z from "zod";

export const ProjectSchema = z.object({
  uuid: z.uuidv4(),
  name: z.string().max(255),
  description: z.string().max(255),
  position: z.int32(),
  createdAt: z.iso.datetime(),
  updatedAt: z.iso.datetime(),
});
