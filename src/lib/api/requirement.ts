import { invoke } from '@tauri-apps/api/core';
import type { Requirement } from '$lib/types/requirement';

export const requirementApi = {
  async create(client_id: number, name: string, description?: string): Promise<number> {
    return await invoke('create_requirement', { client_id, name, description });
  },

  async getById(id: number): Promise<Requirement> {
    return await invoke('get_requirement', { id });
  },

  async getByClient(client_id: number): Promise<Requirement[]> {
    return await invoke('get_requirements_by_client', { client_id });
  },

  async update(requirement: Requirement): Promise<void> {
    return await invoke('update_requirement', { requirement });
  },

  async delete(id: number): Promise<void> {
    return await invoke('delete_requirement', { id });
  },
};
