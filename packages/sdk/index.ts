export type EnableClientConfig = {
  url: string;
  token: string;
};

export type StatusResponse = {
  active: boolean;
};

export class EnableClient {
  private baseUrl: string;
  private token: string;

  constructor(config: EnableClientConfig) {
    this.baseUrl = config.url;
    this.token = config.token;
  }

  private async getStatus(featureId: string): Promise<StatusResponse> {
    const res = await fetch(`${this.baseUrl}/api/feature/${featureId}`, {
      headers: {
        Authorization: `Bearer ${this.token}`,
      },
    });

    return await res.json();
  }

  async isActive(featureId: string): Promise<boolean> {
    const { active } = await this.getStatus(featureId);
    return active;
  }

  async isDisabled(featureId: string): Promise<boolean> {
    const { active } = await this.getStatus(featureId);
    return !active;
  }
}
