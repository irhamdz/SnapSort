import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import App from './App';

describe('App Component', () => {
  it('renders without crashing', () => {
    render(<App />);
    expect(screen.getByText('SnapSort')).toBeInTheDocument();
  });

  it('has expected basic structure', () => {
    render(<App />);
    const appElement = screen.getByText('SnapSort').closest('div');
    expect(appElement).toHaveClass('min-h-screen', 'bg-gray-100');
  });
});