# Echo Frontend

Next.js + TypeScript + Tailwind CSS + shadcn/ui frontend for the Echo SNS project.

## Features

- **Validation-Free**: No likes, no follows, no profile views
- **Anonymous**: Random names and avatars
- **Ephemeral**: Posts disappear after 10 views
- **Real-time**: Timeline updates automatically

## Setup

1. Install dependencies:
```bash
npm install
```

2. Copy `.env.local.example` to `.env.local`:
```bash
cp .env.local.example .env.local
```

3. Start the development server:
```bash
npm run dev
```

The app will be available at http://localhost:3000

## Tech Stack

- **Framework**: Next.js 14 (App Router)
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **UI Components**: shadcn/ui
- **GraphQL Client**: Apollo Client
- **Icons**: Lucide React

## Project Structure

```
src/
├── app/                # Next.js App Router pages
├── components/         # React components
│   ├── ui/            # shadcn/ui components
│   ├── CreatePost.tsx # Post creation form
│   ├── Timeline.tsx   # Timeline view
│   └── PostCard.tsx   # Individual post card
└── lib/               # Utilities and configurations
    ├── graphql/       # GraphQL queries and mutations
    └── apollo-client.ts
```

## Development

```bash
npm run dev      # Start development server
npm run build    # Build for production
npm run start    # Start production server
npm run lint     # Run ESLint
```
