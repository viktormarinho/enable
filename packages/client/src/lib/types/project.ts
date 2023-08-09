import { Feature } from "./feature";

export type ProjectPageData = {
  project: {
    name: string;
  };
  features: Feature[];
};
