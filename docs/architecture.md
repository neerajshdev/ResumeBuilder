# Resume Builder Architecture

## Overview

The Resume Builder application is a web-based tool built with Rust using the Dioxus framework. It follows a component-based architecture inspired by React but implemented in Rust.

## Core Components

### Data Models

The application uses the following data structures:

- `Resume`: The main container for all resume data
- `PersonalInfo`: Basic personal information
- `Education`: Educational background information
- `Experience`: Work experience entries
- `Project`: Project showcase entries
- `Theme`: Enumeration of available resume themes

### UI Components

The application is organized into tabs for different sections of the resume:

- Personal Information
- Education
- Work Experience
- Skills
- Projects
- Resume Preview

Each tab contains forms for data entry or preview components to display the resume.

## State Management

We use Dioxus's `use_signal` hook for reactive state management. The main application state is a `Resume` struct that contains all the data needed to render a complete resume.

## Styling

The application uses Tailwind CSS for styling, loaded via CDN in the index.html file.

## Data Flow

1. User inputs data in form fields
2. Form events update the Resume state
3. Preview tab renders the current state of the Resume

## Extending the Application

To add new features to the Resume Builder:

1. Update the data models if needed
2. Create new UI components
3. Add new entries to the Tab enum if needed
4. Extend existing forms or add new ones

## Build and Deployment

The application can be built as a standard web application using:

```
cargo build --release --features web
```

The resulting output can be deployed to any static web hosting service. 