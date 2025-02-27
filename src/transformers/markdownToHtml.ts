import { Transformer } from '../types';

const markdownToHtml: Transformer = {
    id: 'markdowntohtml',
    title: 'Markdown to HTML',
    description: 'Convert Markdown syntax to HTML',
    transform: (input: string): string => {
        try {
            if (input.trim() === '') return '';
            
            let html = input;
            
            // Handle headers
            html = html.replace(/^# (.*$)/gm, '<h1>$1</h1>');
            html = html.replace(/^## (.*$)/gm, '<h2>$1</h2>');
            html = html.replace(/^### (.*$)/gm, '<h3>$1</h3>');
            html = html.replace(/^#### (.*$)/gm, '<h4>$1</h4>');
            html = html.replace(/^##### (.*$)/gm, '<h5>$1</h5>');
            html = html.replace(/^###### (.*$)/gm, '<h6>$1</h6>');
            
            // Handle code blocks - do this early to avoid processing markdown inside code blocks
            html = html.replace(/```([\s\S]*?)```/g, (_, code) => {
                // Remove leading and trailing newlines
                code = code.replace(/^\n/, '').replace(/\n$/, '');
                return `<pre><code>${code}</code></pre>`;
            });
            
            // Handle inline code
            html = html.replace(/`(.*?)`/g, '<code>$1</code>');
            
            // Handle horizontal rule
            html = html.replace(/^\s*---\s*$/gm, '<hr>');
            
            // Handle unordered lists
            // First, identify list blocks - ensure we capture all items
            const listBlockRegex = /^(- .+(\n|$))+/gm;
            html = html.replace(listBlockRegex, match => {
                // Process each list item
                const listItems = match.split('\n').filter(item => item.trim() && item.startsWith('- '));
                const processedItems = listItems.map(item => 
                    `<li>${item.replace(/^- /, '')}</li>`
                ).join('');
                return `<ul>${processedItems}</ul>`;
            });
            
            // Handle ordered lists
            // First, identify list blocks - ensure we capture all items
            const orderedListBlockRegex = /^(\d+\. .+(\n|$))+/gm;
            html = html.replace(orderedListBlockRegex, match => {
                // Process each list item
                const listItems = match.split('\n').filter(item => item.trim() && /^\d+\./.test(item));
                const processedItems = listItems.map(item => 
                    `<li>${item.replace(/^\d+\. /, '')}</li>`
                ).join('');
                return `<ol>${processedItems}</ol>`;
            });
            
            // Handle images - must be done before links
            html = html.replace(/!\[(.*?)\]\((.*?)\)/g, '<img alt="$1" src="$2">');
            
            // Handle links
            html = html.replace(/\[(.*?)\]\((.*?)\)/g, '<a href="$2">$1</a>');
            
            // Handle bold
            html = html.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>');
            html = html.replace(/__(.*?)__/g, '<strong>$1</strong>');
            
            // Handle italic
            html = html.replace(/\*(.*?)\*/g, '<em>$1</em>');
            html = html.replace(/_(.*?)_/g, '<em>$1</em>');
            
            // Ensure single-line emphasis is wrapped in paragraph tags if not already in a block
            if (/^<(em|strong|code)>.*<\/\1>$/.test(html)) {
                html = `<p>${html}</p>`;
            }
            
            // Handle paragraphs (must be done last)
            // Split by double newlines and wrap non-empty, non-HTML blocks in <p> tags
            const paragraphs = html.split(/\n\s*\n/);
            html = paragraphs.map(p => {
                const trimmed = p.trim();
                if (!trimmed) return '';
                // Skip if it's already an HTML block
                if (/^<(\w+)>/.test(trimmed)) return trimmed;
                return `<p>${trimmed}</p>`;
            }).join('\n\n');
            
            return html;
        } catch {
            return 'Error converting Markdown to HTML';
        }
    }
};

export default {
    markdowntohtml: markdownToHtml
} as const;
