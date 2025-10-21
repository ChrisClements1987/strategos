export interface Product {
  id?: number;
  portfolio_id: number;
  parent_product_id?: number;
  name: string;
  description?: string;
  product_type: 'product' | 'module';
  status: string;
  created_at?: string;
  updated_at?: string;
}
