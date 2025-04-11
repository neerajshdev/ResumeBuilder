# Clean Architecture in Resume Builder

## Overview

The Resume Builder application has been refactored to follow Clean Architecture principles. This architecture separates the application into concentric layers with clear dependencies pointing inward:

1. **Domain Layer** (innermost) - Core business entities and rules
2. **Application Layer** - Use cases that orchestrate domain entities
3. **Infrastructure Layer** - Implementation of interfaces defined in the application layer
4. **Presentation Layer** (outermost) - UI components and view models

## Key Benefits

- **Separation of Concerns**: Each layer has a specific responsibility
- **Dependency Rule**: Inner layers do not depend on outer layers
- **Testability**: Domain and application logic can be tested independently
- **Flexibility**: Easy to swap implementations (e.g., storage backends)

## Layer Structure

### Domain Layer

Contains the core business entities and value objects. These are pure Rust data structures that represent the fundamental concepts in our application:

- `Resume` - The main aggregate root
- `PersonalInfo`, `Education`, `Experience`, etc. - Domain entities
- `ResumeTheme` - Value object representing theme options

The domain layer has no dependencies on other layers or external libraries except for serialization.

### Application Layer

Contains the business logic of the application, defined as use cases:

- `ResumeRepository` - Interface for data persistence
- `ResumeUseCase` - Service that implements business operations

These use cases operate on domain entities and define interfaces that will be implemented by the infrastructure layer.

### Infrastructure Layer

Implements interfaces defined in the application layer:

- `InMemoryResumeRepository` - In-memory implementation for state management
- `LocalStorageResumeRepository` - Web storage implementation for persistence

The infrastructure layer provides concrete implementations that connect the application to external frameworks and services.

### Presentation Layer

Contains the UI components and view models:

- `ResumeViewModel` - Adapts use cases for the UI
- UI Components - Reactive components based on Dioxus

The presentation layer is responsible for rendering the UI and forwarding user actions to the application layer.

## Data Flow

1. User interacts with UI components
2. Components call view model methods
3. View model delegates to use cases
4. Use cases manipulate domain entities
5. Use cases may interact with repositories
6. Repositories handle data persistence through infrastructure
7. Changes flow back through the layers to update the UI

## Directory Structure

```
src/
├── domain/
│   ├── mod.rs         # Exports domain entities
│   └── models.rs      # Core business entities
├── application/
│   ├── mod.rs         # Exports application services
│   ├── repository.rs  # Repository interfaces
│   └── use_cases.rs   # Business logic services
├── infrastructure/
│   ├── mod.rs         # Exports infrastructure implementations
│   └── storage.rs     # Repository implementations
├── presentation/
│   ├── mod.rs         # Exports presentation components
│   ├── components/    # UI components
│   └── view_model.rs  # View model adapters
├── lib.rs             # Library exports
└── main.rs            # Application entry point
```

## Extending the Application

When adding new features:

1. Add or update domain entities in the domain layer
2. Add or update use cases in the application layer
3. Implement or update infrastructure components if needed
4. Create or update UI components and view models in the presentation layer

Always ensure that dependencies flow inward, with inner layers unaware of outer layers. 