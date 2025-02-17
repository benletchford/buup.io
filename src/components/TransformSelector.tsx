import styled from '@emotion/styled';
import { FC, useState, useRef, useEffect, KeyboardEvent } from 'react';

const Container = styled.div`
    margin-bottom: 0.5rem;
`;

const InputContainer = styled.div`
    position: relative;
    width: 100%;
`;

const Input = styled.input`
    width: 100%;
    padding: 0.5rem 2rem 0.5rem 0.5rem;
    border: 1px solid ${props => props.theme.border};
    border-radius: 4px;
    background: ${props => props.theme.surface};
    color: ${props => props.theme.text};
    cursor: pointer;
    appearance: none;
    
    &:focus {
        outline: none;
        border-color: #4dabf7;
    }
`;

const DropdownArrow = styled.div`
    position: absolute;
    right: 0.5rem;
    top: 50%;
    transform: translateY(-50%);
    width: 0;
    height: 0;
    border-left: 5px solid transparent;
    border-right: 5px solid transparent;
    border-top: 5px solid ${props => props.theme.text};
    pointer-events: none;
`;

const Dropdown = styled.ul<{ show: boolean }>`
    display: ${props => props.show ? 'block' : 'none'};
    position: absolute;
    width: 100%;
    max-height: 200px;
    overflow-y: auto;
    margin: 0;
    padding: 0;
    list-style: none;
    border: 1px solid ${props => props.theme.border};
    border-radius: 4px;
    background: ${props => props.theme.surface};
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    z-index: 1000;
`;

const DropdownItem = styled.li<{ selected: boolean }>`
    padding: 0.5rem;
    cursor: pointer;
    background: ${props => props.selected ? props.theme.border : 'transparent'};
    color: ${props => props.theme.text};

    &:hover {
        background: ${props => props.theme.border};
    }
`;

interface TransformerOption {
    id: string;
    title: string;
    description: string;
}

interface TransformSelectorProps {
    value: string;
    onChange: (value: string) => void;
    options: TransformerOption[];
}

export const TransformSelector: FC<TransformSelectorProps> = ({ value, onChange, options }) => {
    const [inputValue, setInputValue] = useState('');
    const [isOpen, setIsOpen] = useState(false);
    const [selectedIndex, setSelectedIndex] = useState(-1);
    const containerRef = useRef<HTMLDivElement>(null);

    const filteredOptions = options.filter(option =>
        option.title.toLowerCase().includes(inputValue.toLowerCase())
    );

    const currentOption = options.find(opt => opt.id === value);

    useEffect(() => {
        if (currentOption) {
            setInputValue(currentOption.title);
        }
    }, [value, currentOption]);

    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (containerRef.current && !containerRef.current.contains(event.target as Node)) {
                setIsOpen(false);
            }
        };

        document.addEventListener('mousedown', handleClickOutside);
        return () => document.removeEventListener('mousedown', handleClickOutside);
    }, []);

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setInputValue(e.target.value);
        setIsOpen(true);
        setSelectedIndex(-1);
    };

    const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
        if (!isOpen) {
            if (e.key === 'ArrowDown' || e.key === 'ArrowUp') {
                setIsOpen(true);
                return;
            }
        }

        switch (e.key) {
            case 'ArrowDown':
                setSelectedIndex(prev =>
                    prev < filteredOptions.length - 1 ? prev + 1 : prev
                );
                e.preventDefault();
                break;
            case 'ArrowUp':
                setSelectedIndex(prev => prev > 0 ? prev - 1 : prev);
                e.preventDefault();
                break;
            case 'Enter':
                if (selectedIndex >= 0 && selectedIndex < filteredOptions.length) {
                    const selected = filteredOptions[selectedIndex];
                    onChange(selected.id);
                    setInputValue(selected.title);
                    setIsOpen(false);
                }
                break;
            case 'Escape':
                setIsOpen(false);
                if (currentOption) {
                    setInputValue(currentOption.title);
                }
                break;
        }
    };

    const handleOptionClick = (option: TransformerOption) => {
        onChange(option.id);
        setInputValue(option.title);
        setIsOpen(false);
    };

    return (
        <Container ref={containerRef}>
            <InputContainer>
                <Input
                    value={inputValue}
                    onChange={handleInputChange}
                    onFocus={() => setIsOpen(true)}
                    onKeyDown={handleKeyDown}
                    placeholder="Search transformers..."
                    readOnly={!isOpen}
                />
                <DropdownArrow />
            </InputContainer>
            <Dropdown show={isOpen}>
                {filteredOptions.map((option, index) => (
                    <DropdownItem
                        key={option.id}
                        selected={index === selectedIndex}
                        onClick={() => handleOptionClick(option)}
                        title={option.description}
                    >
                        {option.title}
                    </DropdownItem>
                ))}
            </Dropdown>
        </Container>
    );
};
