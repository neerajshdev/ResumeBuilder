# Resume Themes

## Overview

The Resume Builder application offers multiple themes for styling resumes. Each theme provides a unique look and feel to help users create professional, eye-catching resumes.

## Available Themes

### Professional

The Professional theme is clean, traditional, and suitable for most industries. It uses a conventional layout with subdued colors, making it appropriate for corporate environments.

Features:
- Clean, minimal design
- Traditional section layout
- Subdued color scheme (blue and gray)
- Emphasis on content readability

### Minimal

The Minimal theme focuses on simplicity and whitespace. It's perfect for those who prefer an uncluttered, modern look.

Features:
- Abundant whitespace
- Simple typography
- Minimalist section dividers
- Black and white with light gray accents

### Creative

The Creative theme is designed for those in creative industries like design, art, or marketing. It incorporates more visual elements and a less conventional layout.

Features:
- More vibrant color options
- Creative layout with asymmetrical elements
- Stylized section headers
- Modern typography

### Modern

The Modern theme balances professionalism with contemporary design elements. It's suitable for tech industries and forward-thinking companies.

Features:
- Bold typography
- Strategic use of color accents
- Clean, grid-based layout
- Modern section dividers

## Implementation

Themes are implemented through different CSS classes applied to the resume preview. The theme selection is stored in the Resume data structure and can be changed in the preview tab.

## Extending Themes

To add a new theme:

1. Add a new variant to the `Theme` enum
2. Update the theme selection UI in the preview component
3. Add CSS styles for the new theme
4. Update this documentation 