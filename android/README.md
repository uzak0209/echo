# Echo Android

Kotlin + Jetpack Compose + Apollo GraphQL Android app for the Echo SNS project.

## Features

- **Validation-Free**: No likes, no follows, no profile views
- **Anonymous**: Random names and avatars
- **Ephemeral**: Posts disappear after 10 views
- **Modern UI**: Built with Jetpack Compose and Material 3

## Setup

1. Open the project in Android Studio

2. Update the server URL in `ApolloClientProvider.kt` if needed:
```kotlin
private const val SERVER_URL = "http://10.0.2.2:8000/graphql"
```

3. Build and run the app

## Tech Stack

- **Language**: Kotlin
- **UI Framework**: Jetpack Compose
- **GraphQL Client**: Apollo Kotlin
- **Architecture**: MVVM
- **Image Loading**: Coil

## Project Structure

```
app/src/main/java/com/echo/app/
├── data/
│   └── ApolloClientProvider.kt
├── ui/
│   ├── components/
│   │   ├── CreatePostCard.kt
│   │   ├── TimelineList.kt
│   ├── viewmodel/
│   │   └── EchoViewModel.kt
│   ├── theme/
│   │   └── Theme.kt
│   └── EchoApp.kt
└── MainActivity.kt
```

## GraphQL Schema

The GraphQL schema and queries are defined in:
- `src/main/graphql/schema.graphqls`
- `src/main/graphql/queries.graphql`
- `src/main/graphql/mutations.graphql`

Apollo Kotlin generates type-safe Kotlin code from these GraphQL files during build.
