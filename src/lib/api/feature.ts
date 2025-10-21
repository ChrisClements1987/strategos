import { invoke } from '@tauri-apps/api/core';
import type { Feature } from '$lib/types/feature';

export const featureApi = {
  async create(product_id: number, name: string, description?: string): Promise<number> {
    return await invoke('create_feature', { product_id, name, description });
  },

  async getById(id: number): Promise<Feature> {
    return await invoke('get_feature', { id });
  },

  async getByProduct(product_id: number): Promise<Feature[]> {
    return await invoke('get_features_by_product', { product_id });
  },

  async update(feature: Feature): Promise<void> {
    return await invoke('update_feature', { feature });
  },

  async delete(id: number): Promise<void> {
    return await invoke('delete_feature', { id });
  },
};
