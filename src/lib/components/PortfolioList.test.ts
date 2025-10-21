import { render, screen, waitFor } from '@testing-library/svelte';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import PortfolioList from './PortfolioList.svelte';
import * as portfolioApi from '$lib/api/portfolio';

vi.mock('$lib/api/portfolio');

describe('PortfolioList', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should render loading state initially', () => {
    vi.mocked(portfolioApi.portfolioApi.getAll).mockReturnValue(
      new Promise(() => {})
    );

    render(PortfolioList);
    expect(screen.getByText(/loading/i)).toBeInTheDocument();
  });

  it('should display portfolios when loaded', async () => {
    const mockPortfolios = [
      { id: 1, name: 'Portfolio 1', description: 'First portfolio' },
      { id: 2, name: 'Portfolio 2', description: 'Second portfolio' },
    ];

    vi.mocked(portfolioApi.portfolioApi.getAll).mockResolvedValue(mockPortfolios);

    render(PortfolioList);

    await waitFor(() => {
      expect(screen.getByText('Portfolio 1')).toBeInTheDocument();
      expect(screen.getByText('Portfolio 2')).toBeInTheDocument();
    });
  });

  it('should display empty state when no portfolios', async () => {
    vi.mocked(portfolioApi.portfolioApi.getAll).mockResolvedValue([]);

    render(PortfolioList);

    await waitFor(() => {
      expect(screen.getByText(/no portfolios/i)).toBeInTheDocument();
    });
  });

  it('should display error message on fetch failure', async () => {
    vi.mocked(portfolioApi.portfolioApi.getAll).mockRejectedValue(
      new Error('Failed to fetch')
    );

    render(PortfolioList);

    await waitFor(() => {
      expect(screen.getByText(/error/i)).toBeInTheDocument();
    });
  });
});
