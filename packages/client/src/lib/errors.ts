type Error = { general: string };

export type ErrorResponse = { error: string; fields: string[] };

export namespace errors {
  export function make<T>(data: T): T & Error {
    return {
      ...data,
      general: "",
    };
  }

  export function reset<T>(object: Record<string, string>): T {
    for (const key in object) {
      object[key] = "";
    }
    return object as T;
  }

  export function apply<T extends Error>(object: T, err: ErrorResponse) {
    if (err) {
      if (!err.fields.length) {
        object.general = err.error;
        return;
      }
      for (const field of err.fields) {
        object[field] = err.error;
      }
    }
    return object;
  }
}
