# Buup Web

A clean, minimalist web interface for Buup text transformations, inspired by Apple and Google design principles.

## Features

- Clean, minimalist design with focus on content
- Categorized transformer selection
- Quick search functionality to find transformations
- Elegant dark/light mode toggle
- Copy results with a single click
- Smooth animations and transitions
- Responsive design for all devices
- Clear visual hierarchy
- Native feeling UI
- Sensible max heights for input and output areas
- Informative footer with attribution and links

## Running Locally

```bash
# Make sure you have the Dioxus CLI installed
cargo install dioxus-cli

# Run the web app in development mode
dx serve
```

## Building for Production

```bash
# Build static files for web
dx build --release
```

## Design Philosophy

The interface was crafted with these principles in mind:

- **Content First**: The transformers and text are the focus
- **Minimal Distractions**: UI elements appear only when needed
- **Intuitive Interactions**: Familiar patterns that feel natural
- **Typography Matters**: Clear, legible text with proper hierarchy
- **Thoughtful Animation**: Subtle cues to enhance understanding
- **Adaptable Design**: Works beautifully across all screen sizes
- **Accessibility**: Design that works for everyone

## Development

The application is built using:

- [Dioxus](https://dioxuslabs.com/) - A React-like framework for Rust
- [buup](../) - Core text transformation library
