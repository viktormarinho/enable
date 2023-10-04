import { Environment } from "./environment";

export type ProjectPageData = {
  project: {
    name: string;
  };
  envs: Environment[]; 
};
