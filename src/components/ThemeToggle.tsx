import styled from '@emotion/styled';
import { FC } from 'react';

const Button = styled.button`
    position: absolute;
    top: 1rem;
    right: 1rem;
    padding: 0.5rem 1rem;
    border: 1px solid ${props => props.theme.border};
    border-radius: 4px;
    background: ${props => props.theme.surface};
    color: ${props => props.theme.text};
    cursor: pointer;
    
    &:hover {
        opacity: 0.8;
    }
`;

interface ThemeToggleProps {
    isDark: boolean;
    onToggle: () => void;
}

export const ThemeToggle: FC<ThemeToggleProps> = ({ isDark, onToggle }) => {
    return (
        <Button onClick={onToggle}>
            {isDark ? '☀️' : '🌙'}
        </Button>
    );
};
