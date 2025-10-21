import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import PortfolioForm from './PortfolioForm.svelte';
import * as portfolioApi from '$lib/api/portfolio';

vi.mock('$lib/api/portfolio');

describe('PortfolioForm', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should render form with name and description fields', () => {
    const { container } = render(PortfolioForm);
    
    expect(screen.getByLabelText(/name/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/description/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /save/i })).toBeInTheDocument();
  });

  it('should call create API when submitting new portfolio', async () => {
    vi.mocked(portfolioApi.portfolioApi.create).mockResolvedValue(1);
    
    const mockOnSave = vi.fn();
    render(PortfolioForm, { props: { onSave: mockOnSave } });

    await fireEvent.input(screen.getByLabelText(/name/i), { target: { value: 'New Portfolio' } });
    await fireEvent.input(screen.getByLabelText(/description/i), { target: { value: 'Test description' } });
    await fireEvent.click(screen.getByRole('button', { name: /save/i }));

    await waitFor(() => {
      expect(portfolioApi.portfolioApi.create).toHaveBeenCalledWith('New Portfolio', 'Test description');
      expect(mockOnSave).toHaveBeenCalled();
    });
  });

  it('should call update API when editing existing portfolio', async () => {
    const existingPortfolio = {
      id: 1,
      name: 'Existing Portfolio',
      description: 'Old description'
    };

    vi.mocked(portfolioApi.portfolioApi.update).mockResolvedValue();
    
    const mockOnSave = vi.fn();
    render(PortfolioForm, { 
      props: { 
        portfolio: existingPortfolio,
        onSave: mockOnSave 
      } 
    });

    await fireEvent.input(screen.getByLabelText(/name/i), { target: { value: 'Updated Name' } });
    await fireEvent.click(screen.getByRole('button', { name: /save/i }));

    await waitFor(() => {
      expect(portfolioApi.portfolioApi.update).toHaveBeenCalledWith({
        id: 1,
        name: 'Updated Name',
        description: 'Old description'
      });
      expect(mockOnSave).toHaveBeenCalled();
    });
  });

  it('should show validation error for empty name', async () => {
    render(PortfolioForm);

    await fireEvent.click(screen.getByRole('button', { name: /save/i }));

    await waitFor(() => {
      expect(screen.getByText(/name is required/i)).toBeInTheDocument();
    });
  });

  it('should call onCancel when cancel button clicked', async () => {
    const mockOnCancel = vi.fn();
    render(PortfolioForm, { props: { onCancel: mockOnCancel } });

    await fireEvent.click(screen.getByRole('button', { name: /cancel/i }));

    expect(mockOnCancel).toHaveBeenCalled();
  });

  it('should disable submit button while saving', async () => {
    vi.mocked(portfolioApi.portfolioApi.create).mockReturnValue(
      new Promise(() => {}) // Never resolves
    );

    render(PortfolioForm);

    await fireEvent.input(screen.getByLabelText(/name/i), { target: { value: 'Test' } });
    const saveButton = screen.getByRole('button', { name: /save/i });
    
    await fireEvent.click(saveButton);

    await waitFor(() => {
      expect(saveButton).toBeDisabled();
    });
  });
});
