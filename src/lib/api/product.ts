import { invoke } from '@tauri-apps/api/core';
import type { Product } from '$lib/types/product';

export const productApi = {
  async create(
    portfolio_id: number,
    name: string,
    product_type: 'product' | 'module' = 'product',
    description?: string,
    parent_product_id?: number
  ): Promise<number> {
    return await invoke('create_product', {
      portfolio_id,
      parent_product_id,
      name,
      description,
      product_type,
    });
  },

  async getById(id: number): Promise<Product> {
    return await invoke('get_product', { id });
  },

  async getByPortfolio(portfolio_id: number): Promise<Product[]> {
    return await invoke('get_products_by_portfolio', { portfolio_id });
  },

  async getModules(parent_product_id: number): Promise<Product[]> {
    return await invoke('get_product_modules', { parent_product_id });
  },

  async update(product: Product): Promise<void> {
    return await invoke('update_product', { product });
  },

  async delete(id: number): Promise<void> {
    return await invoke('delete_product', { id });
  },
};
