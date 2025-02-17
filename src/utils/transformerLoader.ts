import { TransformerModule } from '../types';

// Import all transformer modules
const transformerModules = import.meta.glob<TransformerModule>('../transformers/*.ts', { eager: true });

// Combine all transformers into a single object
export const loadTransformers = () => {
    const transformers: TransformerModule = {};
    
    Object.values(transformerModules).forEach((module) => {
        if (module.default) {
            Object.assign(transformers, module.default);
        }
    });
    
    return transformers;
};

// Get all transformer options for the selector
export const getTransformerOptions = () => {
    const transformers = loadTransformers();
    return Object.values(transformers).map(t => ({
        id: t.id,
        title: t.title,
        description: t.description
    }));
};
